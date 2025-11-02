use std::sync::{Arc, Mutex as StdMutex};
use anyhow::anyhow;
use darkelf::bluedevice::BlueLaserDevice;
use darkelf::model::{DeviceInfo, DeviceSettings, MainCommandData};
use darkelf::ui::model::{DeviceList, DeviceMessage,DeviceCommand};
use darkelf::winblue::WinBlueController;
use darkelf::{
    ui::app::{App},
    util, winblue,
};

use log::{error};
use std::{thread};
use tokio::sync::{Mutex, mpsc};

fn main() -> eframe::Result<()> {

    util::setup_logging();

    let (devicesender, devicereceiver) =
        mpsc::unbounded_channel::<DeviceMessage>();
  
    let (winsender, mut winreciever) =
        mpsc::unbounded_channel::<DeviceCommand>();

    let device_list = Arc::new(StdMutex::new(DeviceList::new()));
    let device_list_clone = device_list.clone();

    let device: Arc<StdMutex<Option<BlueLaserDevice>>> = Arc::new(StdMutex::new(None));
    let device_clone = device.clone();

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            
            log::info!("Starting application thread");
            
            let mut device_info: DeviceInfo = DeviceInfo::default();
            
            let mut device_settings: DeviceSettings = DeviceSettings::default();
            
            let mut device_command: MainCommandData = MainCommandData::default();

            loop {

                let mut devlist = device_list_clone.lock().unwrap();
                if devlist.devices.is_empty() {

                    match winblue::scan_laser_devices().await {
                        Ok(devs) => {
                            devlist.devices = devs;
                            devlist.selected_index = if devlist.devices.is_empty() { None } else { Some(0) };
                        },
                        Err(e) => {
                            error!("Failed to scan for devices: {:?}", e);
                            devlist.devices.clear();
                            devlist.selected_index = None;
                        }

                    }

                }    
                
                let _ = devicesender.send(DeviceMessage::DeviceList(
                    devlist.clone()
                ));
                           
                if let Some(ble_device_info) =  devlist.selected_device().cloned() {
                        
                    let mut device = device_clone.lock().unwrap();
                    if device.is_none() {
                            

                        let controller = match WinBlueController::new(Some(&ble_device_info)).await {
                            Ok(ctrl) => Some(ctrl),
                            Err(e) => {
                                error!("Failed to create WinBlueController: {:?}", e);
                                None
                            }
                        };
                    
                        if let Some(controller) = controller {
                            *device = Some(BlueLaserDevice::new(controller));
                        }                    

                    }

                    if let Some(device) = device.as_mut() {

                        if !device.is_connected() {
                            if let Err(e) = device.connect().await
                               .map_err(|e| anyhow!(e.to_string())){
                                    error!("Failed to connect: {:?}", e);
                            } 
                        }

                        let _ = devicesender
                            .send(DeviceMessage:: ConnectionStatus(device.is_connected()));

                        if !device.is_initialized() {
                            device.setup().await;
                        }

                        let _ = devicesender
                            .send(DeviceMessage::SetupStatus(device.is_initialized()));

                        if(device.is_initialized()) {

                            if let Ok(cmd) = winreciever.try_recv() {
                                
                                match cmd {
                                                
                                    DeviceCommand::SetSettings(settings) => {
                                        device.set_settings(settings).await;
                                    }
                                                
                                    DeviceCommand::On(on) => {
                                        if on {
                                            device.on().await;
                                        } else {
                                            device.off().await;
                                        }
                                    }

                                    DeviceCommand::SetMainCommand(command ) => {
                                        device.set_main_command(command).await;
                                    }   

                                    DeviceCommand::Draw(points, draw_config) => {
                                        device.draw(points, draw_config).await;
                                    }
                                                
                                    DeviceCommand::SendText(text) => {
                                        log::info!("Received text command: {}", text);
                                    }
                                            
                                }    
                                    
                            }

                            let device_state = device.get_device_response();
                            if let Some(device_state) = device_state {
                                
                                if device_state.device_info != device_info {
                                    let _ = devicesender
                                        .send(DeviceMessage::DeviceInfo(device_state.device_info.clone()));
                                    device_info = device_state.device_info;
                                }

                                if device_state.settings != device_settings {
                                    let _ = devicesender
                                        .send(DeviceMessage::DeviceSettings(device_state.settings.clone()));
                                    device_settings = device_state.settings;
                                }
                                if device_state.main_data != device_command {
                                    let _ = devicesender
                                        .send(DeviceMessage::DeviceCommand(device_state.main_data.clone()));
                                    device_command = device_state.main_data;
                                }

                            }
                        
                        }
                    }
                                              
                }
                            
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
        
        
    });
});
   




    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 500.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DarkElf",
        options,
        Box::new(|_cc| {
            Ok(Box::new(App::new(
                Arc::new(Mutex::new(devicereceiver)),
                winsender,
            )))
        }),
    )
}


