use eframe::egui::{self, CentralPanel, TopBottomPanel, Label};
use log::info;

use crate::ui::app::App;

pub fn show_playback_settings_ui(app: &mut App, ctx: &eframe::egui::Context) {


	TopBottomPanel::bottom("playback_panel")
		.exact_height(50.0)
		.show(ctx, |ui| {
			egui::Grid::new("settings_row")
				.num_columns(3)
				.show(ui, |ui| {
					let mut audio_mode_enabled = app.command_data.audio_mode == 255;
					let checkbox_response = ui.add_sized(
						[100.0, 0.0],
						egui::Checkbox::new(&mut audio_mode_enabled, "Audio Mode:"),
					);
					if checkbox_response.changed() {
						app.command_data.audio_mode = if audio_mode_enabled { 255 } else { 0 };
						info!("Audio Mode changed: {}", app.command_data.audio_mode);
					}

					ui.add_sized([80.0, 0.0], egui::Label::new("Sound Sensitivity:"));
					ui.add_sized(
						[200.0, 0.0],
						egui::Slider::new(&mut app.command_data.sound_value, 0..=255),
					);

					ui.add_sized([80.0, 0.0], egui::Label::new("Playback Speed:"));
					ui.add_sized(
						[200.0, 0.0],
						egui::Slider::new(&mut app.command_data.run_speed, 1..=255),
					);
				});
		});
}