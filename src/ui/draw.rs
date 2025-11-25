use crate::{blue::model::{DrawCommandData, DrawConfig, DrawData, Point}, ui::{self, app::App, model::DeviceCommand}};
use eframe::egui::*;




pub fn show_draw_ui(app: &mut App, ctx:  &eframe::egui::Context) {

    
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
                        ui.heading("Draw Preview");
                    });
                })
                .body(|mut body| {
                    body.row(available_height - 20.0, |mut row| { // Reduce row height slightly for spacing
                        // Left side - Draw Commands (2/3 width)
                        row.col(|ui| {
                            let mut text = app.draw_data.clone();
                            ui.add_sized(
                                [ui.available_width(), available_height - 20.0],
                                egui::TextEdit::multiline(&mut text)
                                    .hint_text("Enter your drawing data here..")
                            );
                            app.draw_data = text;
                        });
                        
                        row.col(|ui| {



                            // Painter with footer showing mouse coordinates
                            let (rect, response) = ui.allocate_exact_size(
                                egui::Vec2::new(405.0, 405.0),
                                egui::Sense::hover()
                            );
                            let painter = ui.painter();
                            painter.rect_filled(rect, 0.0, egui::Color32::BLACK);
                            painter.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, egui::Color32::BLACK), egui::StrokeKind::Outside);

                            // Show mouse coordinates relative to painter
                            use std::time::{Instant, Duration};
                            static mut LAST_SEND: Option<Instant> = None;
                            if let Some(pos) = response.hover_pos() {
                                let rel_x = pos.x - rect.left();
                                let rel_y = pos.y - rect.top();
                                // Translate to 800x800 system with origin at center
                                let virt_x = rel_x * 800.0 / 405.0 - 400.0;
                                let virt_y = 400.0 - rel_y * 800.0 / 405.0;
                                ui.label(format!("Cursor (800x800): ({:.1}, {:.1})", virt_x, virt_y));

                                // Send draw command with cross at mouse coordinates every 100ms
                                let now = Instant::now();
                                let should_send = unsafe {
                                    match LAST_SEND {
                                        Some(last) => now.duration_since(last) > Duration::from_millis(100),
                                        None => true,
                                    }
                                };
                                if should_send {
                                    // Build cross shape at (virt_x, virt_y) with correct types
                               //     let cross_size = 40.0;
                               //     let points = vec![
                              //          Point { x: (virt_x - cross_size) as f64, y: virt_y as f64, color: 9, pen_state: 1u8 },
                              //          Point { x: (virt_x + cross_size) as f64, y: virt_y as f64, color: 9, pen_state: 1u8 },
                               //         Point { x: virt_x as f64, y: (virt_y - cross_size) as f64, color: 9, pen_state: 1u8 },
                               //         Point { x: virt_x as f64, y: (virt_y + cross_size) as f64, color: 9, pen_state: 1u8 },
                               //     ];

                                    
                               //     let draw_data = DrawData { points, config: app.cached_points_result.as_ref().map_or(DrawCommandData::default(), |d| d.config.clone()) };
                               //     let _ = app.command_sender.send(DeviceCommand::Draw(draw_data.points.clone(), draw_data.config.clone()));
                               //     unsafe { LAST_SEND = Some(now); }
                                }
                            } else {
                                ui.label("Cursor: (not over painter)");
                            }
                            
                        });
                    });
                });
            
            ui.add_space(20.0); 
            
        
            let text = app.draw_data.clone();
            if text != app.cached_draw_text {
                app.cached_draw_text = text.clone();
                if !text.is_empty() {
                    let (result, error_msg) = parse_points_from_json(&text);
                    if let Ok(result) = result {
                        app.cached_points_result = Some(result);
                        app.cached_error_result = None;
                    }
                    if let Some(err) = error_msg {
                        app.cached_error_result = Some(err);
                    }
                } else {
                    app.cached_points_result = Some(DrawData{ points: Vec::new(), config: DrawConfig::default() });
                }
            }
            
            let mut has_errors = false;
            if let Some(ref error) = app.cached_error_result {
                ui.colored_label(egui::Color32::RED, format!("Points data Error: {}", error));
                has_errors = true;
            }

            if has_errors {
                ui.add_space(5.0);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
             
                let has_valid_points = if let Some(ref cached_result) = app.cached_points_result {
                    !cached_result.points.is_empty()
                } else {
                    false
                };
                
                let can_send = has_valid_points;
                
                let mut send_button = egui::Button::new("Draw")
                    .min_size(egui::vec2(200.0, 40.0));
                
                // Disable button if no valid points or config
                if !can_send {
                    send_button = send_button.fill(egui::Color32::GRAY);
                }
                
                let button_response = ui.add_enabled(can_send, send_button);
                
                if button_response.clicked() && can_send {
                    if let Some(draw_data) = &app.cached_points_result {
                        log::info!("Sending draw commands with config {:?} {:?} points", draw_data.config, draw_data.points);
                        let _ = app.command_sender.send(DeviceCommand::Draw(draw_data.points.clone(), draw_data.config.clone()));
                    }
                }
                
                ui.add_space(5.0);
                if !can_send {
                    ui.small("Points: [{\"x\": 0.0, \"y\": 0.0, \"color\": 0, \"pen_state\": 1}, ...] | Config: {\"cnf_valus\": [0,0,0,0,0,0,0,0,0,0,0,0,3], \"tx_point_time\": 55}");
                } else if !has_valid_points {
                    ui.colored_label(egui::Color32::GRAY, "Enter valid points JSON to enable Draw button");
                } else {
                    ui.colored_label(egui::Color32::GRAY, "Enter valid config JSON to enable Draw button");
                }
            });
        });
    });
}


fn parse_points_from_json(json_text: &str) -> (Result<DrawData, serde_json::Error>, Option<String>) {
    match serde_json::from_str::<DrawData>(json_text) {
        Ok(points) => (Ok(points), None),
        Err(e) => {
            let error_msg = format!("JSON parsing failed: {}", e);
            (Err(e), Some(error_msg))
        },
    }
}

fn parse_config_from_json(json_text: &str) -> (Result<DrawCommandData, serde_json::Error>, Option<String>) {
    match serde_json::from_str::<DrawCommandData>(json_text) {
        Ok(config) => (Ok(config), None),
        Err(e) => {
            let error_msg = format!("Config JSON parsing failed: {}", e);
            (Err(e), Some(error_msg))
        },
    }
}