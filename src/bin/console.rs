use anyhow::anyhow;
use darkelf::device::LaserDevice;
use darkelf::ui::console::DeviceMessage;
use darkelf::winblue::WinBlueController;
use darkelf::{
    model::DeviceResponse,
    ui::console::{Console, DeviceCommand, Sign},
    util, winblue,
};
use eframe::egui;
use log::{error, info};
use std::env;
use std::{sync::Arc, thread};
use tokio::sync::{Mutex, mpsc};
use windows::Devices::Enumeration::DeviceInformation;

fn main() -> eframe::Result<()> {
    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }

    let (ui_message_channel_outbound, ui_message_channel_incomming) =
        mpsc::unbounded_channel::<DeviceMessage>();
    let (device_command_channel_outbound, mut device_command_incomming) =
        mpsc::unbounded_channel::<DeviceCommand>();
    //let ui_tx_clone = ui_message_channel_outbound.clone();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            log::info!("Starting device thread");

            let devices: Vec<DeviceInformation> = match winblue::scan_laser_devices().await {
                Ok(devices) => devices,
                Err(e) => {
                    error!("Failed to scan for devices: {:?}", e);
                    return;
                }
            };

            if (!devices.is_empty()) {
                let _ = ui_message_channel_outbound.send(DeviceMessage::DeviceName(
                    devices[0].Name().unwrap_or_default().to_string(),
                ));

                let mut controller = match WinBlueController::new(devices.get(0)).await {
                    Ok(ctrl) => ctrl,
                    Err(e) => {
                        error!("Failed to create WinBlueController: {:?}", e);
                        return;
                    }
                };

                if let Err(e) = controller
                    .connect()
                    .await
                    .map_err(|e| anyhow!(e.to_string()))
                {
                    error!("Failed to connect: {:?}", e);
                    return;
                }

                let _ = ui_message_channel_outbound
                    .send(DeviceMessage::DeviceStatus(controller.is_connected()));

                if controller.is_connected() {
                    let mut device: LaserDevice = LaserDevice::new(controller);

                    device.setup().await;

                    loop {
                        // Handle incoming commands (non-blocking)
                        if let Ok(cmd) = device_command_incomming.try_recv() {
                            match cmd {
                                DeviceCommand::SetSettings(settings) => {
                                    device.set_settings(settings).await;
                                }
                                DeviceCommand::On(on) => {
                                    if (on) {
                                        device.on().await;
                                    } else {
                                        device.off().await;
                                    }
                                }
                                DeviceCommand::SetMode { mode: playback_mode, selected_shows } => {
                                    device.set_playback_mode(playback_mode, selected_shows).await;
                                }
                            }
                        }

                        let response_opt = device.get_device_response();
                        if let Some(response) = response_opt {
                            let _ = ui_message_channel_outbound
                                .send(DeviceMessage::DeviceResponse(response));
                        }

                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                }
            }
        });
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 720.0])
            .with_min_inner_size([1200.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DarkElf",
        options,
        Box::new(|_cc| {
            Box::new(Console::new(
                Arc::new(Mutex::new(ui_message_channel_incomming)),
                device_command_channel_outbound,
            ))
        }),
    )
}
