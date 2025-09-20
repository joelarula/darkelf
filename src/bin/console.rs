use eframe::egui;
use darkelf::ui::console::{Console, Sign};

fn main() -> eframe::Result<()> {
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
            Box::new(Console::default())
        }),
    )
}
