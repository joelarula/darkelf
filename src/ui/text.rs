use crate::ui::{app::App, model};
use eframe::egui::*;

pub fn show_text_ui(console: &mut App, ctx: &eframe::egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            
            // Title
            ui.heading("Display Text Message");
            ui.add_space(30.0);
            
            // Prominent text field
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.vertical(|ui| {
                    ui.label("Enter message:");
                    ui.add_space(10.0);
                    
                    // Large text input field
                    let text_edit = egui::TextEdit::singleline(&mut console.text_command)
                        .desired_width(ui.available_width() - 40.0)
                        .font(egui::TextStyle::Heading)
                        .hint_text("Type your message here...");
                    
                    let text_response = ui.add(text_edit);
                    
                    // Handle Enter key press
                    if text_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        send_text_command(console);
                    }
                });
                ui.add_space(20.0);
            });
            
            ui.add_space(30.0);
            
            // Prominent Send button
            let send_button = egui::Button::new("Send")
                .min_size(egui::vec2(200.0, 50.0))
                .fill(egui::Color32::from_rgb(0, 120, 215)); // Blue button
            
            let button_enabled = !console.text_command.trim().is_empty();
            let button_response = ui.add_enabled(button_enabled, send_button);
            
            if button_response.clicked() {
                send_text_command(console);
            }
            
            ui.add_space(20.0);
            
            // Status message
            if button_enabled {
                ui.small("Press Enter or click Send to execute the message");
            } else {
                ui.colored_label(egui::Color32::GRAY, "Enter a message to enable the Send button");
            }
        });
    });
}

fn send_text_command(console: &mut App) {
    if !console.text_command.trim().is_empty() {
        log::info!("Sending text command: {}", console.text_command);
        
        // Send the text command (you can modify this to send the appropriate command type)
        let _ = console.command_sender.send(model::DeviceCommand::SendText(console.text_command.clone()));
        
        // Clear the text field after sending
        console.text_command.clear();
    }
}