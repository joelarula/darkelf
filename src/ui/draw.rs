use crate::{model::{Point, PisObject}, ui::console::Console};
use eframe::egui::*;

fn parse_points_from_json(json_text: &str) -> (Result<Vec<Point>, serde_json::Error>, Option<String>) {
    match serde_json::from_str::<Vec<Point>>(json_text) {
        Ok(points) => (Ok(points), None),
        Err(e) => {
            let error_msg = format!("JSON parsing failed: {}", e);
            (Err(e), Some(error_msg))
        },
    }
}

fn parse_config_from_json(json_text: &str) -> (Result<PisObject, serde_json::Error>, Option<String>) {
    match serde_json::from_str::<PisObject>(json_text) {
        Ok(config) => (Ok(config), None),
        Err(e) => {
            let error_msg = format!("Config JSON parsing failed: {}", e);
            (Err(e), Some(error_msg))
        },
    }
}

pub fn show_draw_ui(console: &mut Console, ctx:  &eframe::egui::Context) {

    
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.add_space(10.0);
            
            let available_height = ui.available_height() - 120.0; // Reserve more space for button and messages
            
            // Side-by-side text areas using table with 100% width
            use egui_extras::{TableBuilder, Column};
            
            TableBuilder::new(ui)
                .column(Column::remainder().resizable(true).at_least(400.0))  // 2/3 for Draw Commands
                .column(Column::remainder().resizable(true).at_least(200.0))  // 1/3 for Draw Config
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Draw Points");
                    });
                    header.col(|ui| {
                        ui.heading("Draw Config");
                    });
                })
                .body(|mut body| {
                    body.row(available_height - 20.0, |mut row| { // Reduce row height slightly for spacing
                        // Left side - Draw Commands (2/3 width)
                        row.col(|ui| {
                            let mut text = console.draw_data.clone();
                            ui.add_sized(
                                [ui.available_width(), available_height - 20.0],
                                egui::TextEdit::multiline(&mut text)
                                    .hint_text("Enter your drawing data here..")
                            );
                            console.draw_data = text;
                        });
                        
                        // Right side - Draw Config (1/3 width)
                        row.col(|ui| {
                            let mut config_text = console.draw_config_data.clone();
                            ui.add_sized(
                                [ui.available_width(), available_height - 20.0],
                                egui::TextEdit::multiline(&mut config_text)
                                    .hint_text("Enter draw config here..")
                            );
                            console.draw_config_data = config_text;
                        });
                    });
                });
            
            ui.add_space(20.0); // Increased spacing after table
            
            // Cache points parsing
            let text = console.draw_data.clone();
            if text != console.cached_draw_text {
                console.cached_draw_text = text.clone();
                if !text.is_empty() {
                    let (result, _) = parse_points_from_json(&text);
                    console.cached_points_result = Some(result.map_err(|e| e.to_string()));
                } else {
                    console.cached_points_result = Some(Ok(Vec::new()));
                }
            }
            
            // Cache config parsing
            let config_text = console.draw_config_data.clone();
            if config_text != console.cached_config_text {
                console.cached_config_text = config_text.clone();
                if !config_text.is_empty() {
                    let (result, _) = parse_config_from_json(&config_text);
                    console.cached_config_result = Some(result.map_err(|e| e.to_string()));
                } else {
                    // Use default PisObject when config is empty
                    console.cached_config_result = Some(Ok(PisObject {
                        cnf_valus: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
                        tx_point_time: 55,
                    }));
                }
            }
            
            // Show error messages if JSON parsing failed
            let mut has_errors = false;
            if let Some(ref cached_result) = console.cached_points_result {
                if let Err(error) = cached_result {
                    ui.colored_label(egui::Color32::RED, format!("Points Error: {}", error));
                    has_errors = true;
                }
            }
            if let Some(ref cached_result) = console.cached_config_result {
                if let Err(error) = cached_result {
                    ui.colored_label(egui::Color32::RED, format!("Config Error: {}", error));
                    has_errors = true;
                }
            }
            if has_errors {
                ui.add_space(5.0);
            }
            
            // Prominent send button at the bottom
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                // Check if we have valid points with length > 0
                let has_valid_points = if let Some(ref cached_result) = console.cached_points_result {
                    match cached_result {
                        Ok(points) => !points.is_empty(),
                        Err(_) => false,
                    }
                } else {
                    false
                };
                
                // Check if we have valid config
                let has_valid_config = if let Some(ref cached_result) = console.cached_config_result {
                    matches!(cached_result, Ok(_))
                } else {
                    false
                };
                
                let can_send = has_valid_points && has_valid_config;
                
                let mut send_button = egui::Button::new("Draw")
                    .min_size(egui::vec2(200.0, 40.0));
                
                // Disable button if no valid points or config
                if !can_send {
                    send_button = send_button.fill(egui::Color32::GRAY);
                }
                
                let button_response = ui.add_enabled(can_send, send_button);
                
                if button_response.clicked() && can_send {
                    if let (Some(Ok(points)), Some(Ok(draw_config))) = (
                        console.cached_points_result.as_ref(),
                        console.cached_config_result.as_ref()
                    ) {
                        log::info!("Sending draw commands with {} points", points.len());
                        let _ = console.command_sender.send(crate::ui::console::DeviceCommand::Draw(points.clone(), draw_config.clone()));
                    }
                }
                
                ui.add_space(5.0);
                if can_send {
                    ui.small("Points: [{\"x\": 0.0, \"y\": 0.0, \"color\": 0, \"pen_state\": 1}, ...] | Config: {\"cnf_valus\": [0,0,0,0,0,0,0,0,0,0,0,0,3], \"tx_point_time\": 55}");
                } else if !has_valid_points && !has_valid_config {
                    ui.colored_label(egui::Color32::GRAY, "Enter valid JSON for both points and config to enable Draw button");
                } else if !has_valid_points {
                    ui.colored_label(egui::Color32::GRAY, "Enter valid points JSON to enable Draw button");
                } else {
                    ui.colored_label(egui::Color32::GRAY, "Enter valid config JSON to enable Draw button");
                }
            });
        });
    });
}