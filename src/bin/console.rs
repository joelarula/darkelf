use eframe::egui;

struct DarkElf {
    value: i32,
    slider_value: i32,
}

impl eframe::App for DarkElf {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Top panel (auto height)
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("This is the top panel!");
        });

        // Bottom panel (fixed height)
egui::TopBottomPanel::bottom("bottom_panel")
    .exact_height(100.0)
    .show(ctx, |ui| {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.columns(4, |columns| {


                columns[0].set_min_width(150.0); 
                columns[0].horizontal(|ui| {
                    ui.label("Channel:");
                    ui.add(
                        egui::DragValue::new(&mut self.value)
                            .clamp_range(1..=512)
                            .speed(1),
                    );
                });
                columns[1].set_min_width(150.0); 
                columns[1].horizontal(|ui| {
                    ui.label("Display range:");
                    ui.add(
                        egui::Slider::new(&mut self.slider_value, 10..=100)
                            .show_value(true),
                    );
                });
                columns[2].set_min_width(150.0); 
                if columns[2].button("Button 3").clicked() {
                    // Action for Button 3
                }
                columns[3].set_min_width(150.0); 
                if columns[3].button("ON/OFF").clicked() {
                    // Action for Button 4
                }



                
            });
        });
    });

    // Central panel (fills the remaining space)
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("Hello, dark elf!");
    });
}
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };


    eframe::run_native(
        "DarkElf Console",
        options,
        Box::new(|_cc| Box::new(DarkElf { value: 1, slider_value: 10 })),
    )
}