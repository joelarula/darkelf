use crate::ui::console::Console;
use egui::*;

pub fn show_mode_buttons(console: &mut Console, ctx: &egui::Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let mut is_dmx = console.mode == 1;
            if ui.toggle_value(&mut is_dmx, "DMX").changed() && is_dmx {
                console.mode = 1;
            }
            let mut is_random = console.mode == 2;
            if ui.toggle_value(&mut is_random, "Random").changed() && is_random {
                console.mode = 2;
            }
            let mut is_animation = console.mode == 3;
            if ui.toggle_value(&mut is_animation, "Animation").changed() && is_animation {
                console.mode = 3;
            }
            let mut is_line = console.mode == 4;
            if ui.toggle_value(&mut is_line, "Line").changed() && is_line {
                console.mode = 4;
            }
            let mut is_christmas = console.mode == 5;
            if ui.toggle_value(&mut is_christmas, "Christmas").changed() && is_christmas {
                console.mode = 5;
            }
            let mut is_outdoor = console.mode == 6;
            if ui.toggle_value(&mut is_outdoor, "Outdoor").changed() && is_outdoor {
                console.mode = 6;
            }
        });
    });
}
