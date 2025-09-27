use crate::{model::PlaybackMode, ui::console::Console};
use egui::*;
use crate::model;

pub fn show_mode_buttons(console: &mut Console, ctx: &egui::Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let mut is_dmx = console.mode == PlaybackMode::Dmx;
            if ui.toggle_value(&mut is_dmx, "DMX").changed() && is_dmx {
                console.mode = PlaybackMode::Dmx;
            }
            let mut is_random = console.mode == PlaybackMode::RandomPlayback;
            if ui.toggle_value(&mut is_random, "Random").changed() && is_random {
                console.mode = PlaybackMode::RandomPlayback;
            }
            let mut is_animation = console.mode == PlaybackMode::AnimationPlayback;
            if ui.toggle_value(&mut is_animation, "Animation").changed() && is_animation {
                console.mode = PlaybackMode::AnimationPlayback;
            }
            let mut is_line = console.mode == PlaybackMode::TimelinePlayback;
            if ui.toggle_value(&mut is_line, "Line").changed() && is_line {
                console.mode = PlaybackMode::TimelinePlayback;
            }
            let mut is_christmas = console.mode == PlaybackMode::ChristmasBroadcast;
            if ui.toggle_value(&mut is_christmas, "Christmas").changed() && is_christmas {
                console.mode = PlaybackMode::ChristmasBroadcast;
            }
            let mut is_outdoor = console.mode == PlaybackMode::OutdoorPlayback;
            if ui.toggle_value(&mut is_outdoor, "Outdoor").changed() && is_outdoor {
                console.mode = PlaybackMode::OutdoorPlayback;
            }
        });
    });
}
