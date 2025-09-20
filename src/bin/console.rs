use eframe::egui;

struct DarkElf {
    value: i32,
    slider_value: i32,
    radio_choice: i32,
    is_enabled: bool,
    x_sign: Sign,
    y_sign: Sign,
}

#[derive(PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}
impl eframe::App for DarkElf {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        eframe.set_window_title("DarkElf - Device is not connected");
        // Top panel (auto height)
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("This is the top panel!");
        });

        // Bottom panel (fixed height)
        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(100.0)
            .show(ctx, |ui| {
                egui::Grid::new("settings_row")
                    .num_columns(9)
                    .show(ui, |ui| {
                        ui.add_sized([50.0, 0.0], egui::Label::new("Channel:"));
                        ui.add_sized(
                            [30.0, 0.0],
                            egui::DragValue::new(&mut self.value).clamp_range(1..=512),
                        );
                        ui.add_sized([80.0, 0.0], egui::Label::new("Display Range:"));
                        ui.add_sized(
                            [200.0, 0.0],
                            egui::Slider::new(&mut self.slider_value, 10..=100),
                        );

                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(60.0, 0.0), // fixed width, flexible height
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                egui::ComboBox::from_label("X")
                                    .selected_text(match self.x_sign {
                                        Sign::Plus => "+",
                                        Sign::Minus => "-",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.x_sign, Sign::Plus, "+");
                                        ui.selectable_value(&mut self.x_sign, Sign::Minus, "-");
                                    });
                            },
                        );

                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(60.0, 0.0), // fixed width, flexible height
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                egui::ComboBox::from_label("Y")
                                    .selected_text(match self.y_sign {
                                        Sign::Plus => "+",
                                        Sign::Minus => "-",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.y_sign, Sign::Plus, "+");
                                        ui.selectable_value(&mut self.y_sign, Sign::Minus, "-");
                                    });
                            },
                        );

                        ui.add_sized(
                            [50.0, 0.0],
                            egui::Checkbox::new(&mut self.is_enabled, "Intechange"),
                        );

                        // Center both radios vertically in the cell
                        // -- First Radio: Fixed width, vertically centered --
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(60.0, 0.0), // fixed width, flexible height
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.radio_value(&mut self.radio_choice, 0, "Mono");
                            },
                        );

                        // -- Second Radio: Fixed width, vertically centered --
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(50.0, 0.0),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.radio_value(&mut self.radio_choice, 1, "RGB");
                            },
                        );

                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(60.0, 0.0),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.toggle_value(&mut self.is_enabled, "ON/OFF");
                            },
                        );
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
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 720.0])
            .with_min_inner_size([1200.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DarkElf Console",
        options,
        Box::new(|_cc| {
            Box::new(DarkElf {
                value: 1,
                slider_value: 10,
                radio_choice: 1,
                is_enabled: false,
                x_sign: Sign::Plus,
                y_sign: Sign::Plus,
            })
        }),
    )
}
