/// Shows a grid of 10x5 on/off buttons labeled 1-50.
pub fn show_selector_grid(ui: &mut egui::Ui) {
    use egui::{Grid, Button};
    Grid::new("device_grid")
        .num_columns(10)
        .striped(true)
        .show(ui, |ui| {
            let mut btn_count = 0;
        for row in 0..5 {
            for col in 0..10 {
                let idx = btn_count + 1;
                if btn_count >= 50 {
                    break;
                }
                let mut is_on = false; // You can replace with actual state if available
                let label = format!("{}", idx);
                let button = Button::new(label).min_size(egui::Vec2::splat(50.0));
                if ui.add(button).clicked() {
                    is_on = !is_on;
                    // TODO: handle on/off logic for device idx
                }
                btn_count += 1;
            }
            ui.end_row();
            if btn_count >= 50 {
                break;
            }
        }
        });
}