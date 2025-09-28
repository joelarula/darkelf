use eframe::egui::{self, CentralPanel, TopBottomPanel, Label};

pub fn show_playback_settings_ui(ctx: &egui::Context) {


	// Bottom bar
	TopBottomPanel::bottom("bottom_bar_playback_settings").show(ctx, |ui| {
		ui.horizontal(|ui| {
			ui.heading("Playback Settings");
		});
	});


}
