use crate::dmx::device::DmxDevice;
use crate::dmx::model::Fixture;
use crate::ui::app::App;
use eframe::egui;
use egui_taffy::{TuiBuilderLogic, taffy, tui};
use std::sync::Arc;

pub fn show_dmx(app: &mut App, ctx: &eframe::egui::Context) {
    // Initialize fixture if missing
    if app.fixture.is_none() {
        if let Ok(file) = std::fs::File::open("assets/fixtures/laser_light_8340.json") {
            if let Ok(fixture) = serde_json::from_reader(file) {
                app.fixture = Some(fixture);
            }
        }
    }

    // Initialize ports if empty
    if app.dmx_ports.is_empty() {
        app.dmx_ports = crate::dmx::controller::scan_dmx_ports();
        // Auto-select if only one
        if app.dmx_ports.len() == 1 && app.selected_dmx_port.is_none() {
            select_and_init_port(app, 0);
        }
    }

    egui::TopBottomPanel::bottom("dmx_status_bar").show(ctx, |ui| {
        if let Some(_dev) = &app.dmx_device {
            ui.label(format!(
                "DMX Device Connected: {}",
                app.selected_dmx_port.as_deref().unwrap_or("Unknown")
            ));
        } else {
            ui.label("DMX Device Not Connected");
        }
    });

    egui::TopBottomPanel::bottom("dmx_port_selector_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("DMX Port:");
            let mut selected_idx = app
                .selected_dmx_port
                .as_ref()
                .and_then(|port| app.dmx_ports.iter().position(|p| p == port))
                .unwrap_or(0);

            let selected_text = app
                .selected_dmx_port
                .clone()
                .unwrap_or_else(|| "Select port".to_string());
            let ports = app.dmx_ports.clone();

            egui::ComboBox::from_id_source("dmx_port_selector")
                .selected_text(selected_text)
                .show_ui(ui, |ui| {
                    for (idx, port) in ports.iter().enumerate() {
                        if ui.selectable_value(&mut selected_idx, idx, port).changed() {
                            select_and_init_port(app, idx);
                        }
                    }
                });
        });
    });

    if let Some(fixture) = &app.fixture {
        let channels = &fixture.channels;
        let mid = channels.len() / 2;
        let (left, right) = channels.split_at(mid);

        egui::CentralPanel::default().show(ctx, |ui| {
            tui(ui, ui.id().with("channels_panel"))
                .reserve_available_space()
                .style(taffy::Style {
                    flex_direction: taffy::FlexDirection::Row,
                    gap: taffy::style_helpers::length(12.0),
                    padding: taffy::style_helpers::length(12.0),
                    ..Default::default()
                })
                .show(|tui| {
                    tui.style(taffy::Style {
                        flex_direction: taffy::FlexDirection::Column,
                        gap: taffy::style_helpers::length(8.0),
                        ..Default::default()
                    })
                    .add(|tui| {
                        if let Some(device) = &app.dmx_device {
                            render_channel_column(tui, left, 0, device);
                        }
                    });

                    tui.style(taffy::Style {
                        flex_direction: taffy::FlexDirection::Column,
                        gap: taffy::style_helpers::length(8.0),
                        ..Default::default()
                    })
                    .add(|tui| {
                        if let Some(device) = &app.dmx_device {
                            render_channel_column(tui, right, mid, device);
                        }
                    });
                });
        });
    } else {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("No fixture loaded. Please check assets/fixtures/laser_light_8340.json");
        });
    }
}

fn select_and_init_port(app: &mut App, idx: usize) {
    if idx < app.dmx_ports.len() {
        let new_port = app.dmx_ports[idx].clone();
        if app.selected_dmx_port.as_ref() != Some(&new_port) {
            app.selected_dmx_port = Some(new_port.clone());
            let dmx_channel = 1;
            if let Some(fixture) = &app.fixture {
                match DmxDevice::new(&new_port, dmx_channel, fixture.clone()) {
                    Ok(dev) => {
                        if dev.start().is_ok() {
                            app.dmx_device = Some(Arc::new(dev));
                        } else {
                            app.dmx_device = None;
                        }
                    }
                    Err(_) => {
                        app.dmx_device = None;
                    }
                }
            }
        }
    }
}

fn render_channel_column(
    tui: &mut egui_taffy::Tui,
    channels: &[crate::dmx::model::Channel],
    offset: usize,
    device: &Arc<DmxDevice>,
) {
    for (i, channel) in channels.iter().enumerate() {
        let label = &channel.name;
        let tooltip = channel
            .capabilities
            .iter()
            .map(|cap| {
                let range = format!("{}-{}", cap.dmx_range[0], cap.dmx_range[1]);
                let name = cap.menu_name.as_ref().unwrap_or(&cap.type_);
                let desc = cap.description.as_deref().unwrap_or("");
                format!("{}: {}\n{}", range, name, desc)
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        if let Some(mut value) = device.get_dmx_channel(offset + i + 1) {
            tui.style(taffy::Style {
                flex_direction: taffy::FlexDirection::Row,
                align_items: Some(taffy::AlignItems::Center),
                gap: taffy::style_helpers::length(8.0),
                ..Default::default()
            })
            .add(|tui| {
                tui.style(taffy::Style {
                    size: taffy::Size {
                        width: taffy::style_helpers::length(120.0),
                        height: taffy::style_helpers::auto(),
                    },
                    ..Default::default()
                })
                .label(label);

                let drag = egui::DragValue::new(&mut value)
                    .range(0..=255)
                    .speed(1.0)
                    .suffix("");

                if tui.ui_add(drag).on_hover_text(tooltip).changed() {
                    device.set_dmx_channel(offset + i + 1, value).ok();
                }
            });
        }
    }
}
