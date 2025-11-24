use eframe::egui::{self, TopBottomPanel};
use log::info;
use crate::model::BeamColor;
use crate::ui::{app::App, model::DeviceCommand};

pub fn show_playback_settings_ui(app: &mut App, ctx: &eframe::egui::Context) {


	TopBottomPanel::bottom("playback_panel")
		.exact_height(50.0)
		.show(ctx, |ui| {
			egui::Grid::new("settings_row")
				.num_columns(3)
				.show(ui, |ui| {
					// Display Color dropdown as first element
					ui.allocate_ui_with_layout(
						egui::Vec2::new(60.0, 0.0),
						egui::Layout::centered_and_justified(egui::Direction::TopDown),
						|ui| {
							ui.horizontal(|ui| {
								ui.label("Display Color:");
								let prev_color = app.command_data.color;
								egui::ComboBox::from_id_salt("display_color_combo")
									.selected_text(format!("{:?}", app.command_data.color))
									.show_ui(ui, |ui| {
										for &color in &[BeamColor::Red, BeamColor::Yellow, BeamColor::Green, BeamColor::Cyan, BeamColor::Blue, BeamColor::Purple, BeamColor::White, BeamColor::Jump, BeamColor::RGB] {
											ui.selectable_value(&mut app.command_data.color, color, format!("{:?}", color));
										}
									});
								if prev_color != app.command_data.color {
									info!("Display Color changed: {:?}", app.command_data.color);
									let command = app.command_data.clone();
									let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));
								}
							});
						}
					);

					// Audio Mode
					let mut audio_mode_enabled = app.command_data.audio_mode == 255;
					let checkbox_response = ui.add_sized(
						[100.0, 0.0],
						egui::Checkbox::new(&mut audio_mode_enabled, "Audio Mode:"),
					);
					if checkbox_response.changed() {
						app.command_data.audio_mode = if audio_mode_enabled { 255 } else { 0 };
						info!("Audio Mode changed: {}", app.command_data.audio_mode);
						let command = app.command_data.clone();
						let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));
					}

					// Sound Sensitivity
					ui.add_sized([80.0, 0.0], egui::Label::new("Sound Sensitivity:"));
					ui.add_enabled_ui(audio_mode_enabled, |ui| {
						let sound_slider_response = ui.add_sized(
							[200.0, 0.0],
							egui::Slider::new(&mut app.command_data.sound_value, 0..=255),
						);
						if sound_slider_response.drag_stopped() {
							info!("Sound Sensitivity changed: {}", app.command_data.sound_value);
							let command = app.command_data.clone();
							let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));
						}
					});

					// Playback Speed
					ui.add_sized([80.0, 0.0], egui::Label::new("Playback Speed:"));
					ui.add_enabled_ui(!audio_mode_enabled, |ui| {
						let speed_slider_response = ui.add_sized(
							[200.0, 0.0],
							egui::Slider::new(&mut app.command_data.run_speed, 1..=255),
						);
						if speed_slider_response.drag_stopped() {
							info!("Playback Speed changed: {}", app.command_data.run_speed);
							let command = app.command_data.clone();
							let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));
						}
					});
				});
		});
}