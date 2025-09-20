


pub struct Console {  
    pub channel: i32,
    pub display_range: i32,
    pub light: Light,
    pub x_y_interchange: bool,
    pub x_sign: Sign,
    pub y_sign: Sign,
    pub on: bool,
    pub device_connected: bool,
    pub device_name: String,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            channel: 1,
            display_range: 50,
            light: Light::Mono,
            x_y_interchange: false,
            x_sign: Sign::Plus,
            y_sign: Sign::Plus,
            on: false,
            device_connected: false,
            device_name: String::new(),
        }
    }
}

#[derive(PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}

impl Default for Sign {
    fn default() -> Self {
        Sign::Plus
    }
}

#[derive(PartialEq)]
pub enum Light {
    Mono,
    RGB,
}

impl Default for Light {
    fn default() -> Self {
        Light::Mono
    }
}

impl eframe::App for Console {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Top panel (auto height)
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("This is the top panel!");
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status: Ready");
                // You can add more widgets here (progress bar, text, etc)
                ui.separator();
                ui.label("Other info");
            });
        });

        // Bottom panel (fixed height)
        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(50.0)
            .show(ctx, |ui| {
                egui::Grid::new("settings_row")
                    .num_columns(9)
                    .show(ui, |ui| {

                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(60.0, 0.0),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.toggle_value(&mut self.on, "ON/OFF");
                            },
                        );
                        ui.add_sized([80.0, 0.0], egui::Label::new("Display Range:"));
                        ui.add_sized(
                            [200.0, 0.0],
                            egui::Slider::new(&mut self.display_range, 10..=100),
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
                            egui::Checkbox::new(&mut self.x_y_interchange, "Intechange"),
                        );

                        // Center both radios vertically in the cell
                        // -- First Radio: Fixed width, vertically centered --
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(60.0, 0.0), // fixed width, flexible height
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.radio_value(&mut self.light, Light::Mono, "Mono");
                            },
                        );

                        // -- Second Radio: Fixed width, vertically centered --
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(50.0, 0.0),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.radio_value(&mut self.light, Light::RGB, "RGB");
                            },
                        );

                        ui.add_sized([50.0, 0.0], egui::Label::new("Channel:"));
                        ui.add_sized(
                            [30.0, 0.0],
                            egui::DragValue::new(&mut self.channel).clamp_range(1..=512),
                        );

                    });
            });

        // Central panel (fills the remaining space)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, dark elf!");
        });
    }
}
