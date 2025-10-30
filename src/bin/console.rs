use std::sync::{Arc, Mutex as StdMutex};
use anyhow::anyhow;
use darkelf::blue::BlueController;
use darkelf::bluedevice::BlueLaserDevice;
use darkelf::model::PlaybackCommand;
use darkelf::ui::model::{DeviceList, DeviceMessage,DeviceCommand};
use darkelf::winblue::WinBlueController;
use darkelf::{
    ui::console::{Console},
    util, winblue,
};
use eframe::egui::*;
use log::{error, info};
use std::env;
use std::{thread};
use tokio::sync::{Mutex, mpsc};

fn main() -> eframe::Result<()> {

    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }

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

                             
                if let Some(device_info) =  devlist.selected_device().cloned() {
                        
                    let _ = devicesender.send(DeviceMessage::DeviceInfo(
                        device_info.clone(),
                    ));

                    let mut device = device_clone.lock().unwrap();
                    if device.is_none() {
                            

                        let mut controller = match WinBlueController::new(Some(&device_info)).await {
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
                                                
                                    DeviceCommand::SetMode { mode: playback_mode, selected_shows } => {
                                        device.set_playback_mode(PlaybackCommand::default(playback_mode)).await;
                                    }
                                                
                                    DeviceCommand::Draw(points, draw_config) => {
                                        device.draw(points, draw_config).await;
                                    }
                                                
                                    DeviceCommand::SendText(text) => {
                                        log::info!("Received text command: {}", text);
                                    }
                                            
                                }    
                                    
                            }

                            let response_opt = device.get_device_response();
                            if let Some(response) = response_opt {
                                let _ = devicesender
                                    .send(DeviceMessage::DeviceResponse(response));
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
            Ok(Box::new(Console::new(
                Arc::new(Mutex::new(devicereceiver)),
                winsender,
            )))
        }),
    )
}


