#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use catppuccin_egui::{set_theme, LATTE, MOCHA};
use colorschemes::LibraryManager;
use eframe::egui;
use egui::{Sense, Stroke, Vec2};
use egui_extras::RetainedImage;
use rfd::FileDialog;
use std::{fs, path::PathBuf};
use sublime_fuzzy::best_match;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "farbenfroh.io - faerber",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    image: Option<RetainedImage>,
    // original_image: Option<RetainedImage>,
    // converted_image: Option<RetainedImage>,
    opened_file: Option<PathBuf>,
    library: LibraryManager,
    color_scheme: String,
    flavor: String,
    flavor_filter: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image: None,
            opened_file: None,
            library: LibraryManager::new(),
            color_scheme: "wezterm".to_string(),
            flavor: "catppuccin mocha".to_string(),
            flavor_filter: "".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match frame.info().system_theme {
            Some(eframe::Theme::Dark) => set_theme(ctx, MOCHA),
            Some(eframe::Theme::Light) => set_theme(ctx, LATTE),
            None => set_theme(ctx, LATTE),
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open").clicked() {
                self.opened_file = FileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg"])
                    .add_filter("Vector files", &["svg"])
                    .pick_file();
            }

            if let Some(file) = &mut self.opened_file {
                let data = fs::read(file).unwrap();
                self.image = Some(RetainedImage::from_image_bytes("image", &data).unwrap());
            }

            if let Some(image) = &self.image {
                image.show_max_size(ui, egui::vec2(300.0, 300.0));
            }

            ui.separator();
            ui.label("Color scheme");
            ui.horizontal_wrapped(|ui| {
                self.library.library.keys().for_each(|color_scheme| {
                    ui.selectable_value(
                        &mut self.color_scheme,
                        color_scheme.to_string(),
                        color_scheme,
                    );
                });
                if ui.button("+").clicked() {}
            });
            ui.separator();
            ui.label("Flavor");
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.flavor_filter));
                if ui.button("X").clicked() {
                    self.flavor_filter = "".to_string();
                }
            });
            ui.horizontal(|ui| {
                self.library
                    .library
                    .get(&self.color_scheme)
                    .unwrap()
                    .keys()
                    .filter_map(|f| {
                        if self.flavor_filter.is_empty() {
                            Some(f)
                        } else {
                            best_match(&self.flavor_filter, f).map(|_| f)
                        }
                    })
                    .for_each(|flavor| {
                        ui.selectable_value(&mut self.flavor, flavor.to_string(), flavor);
                    });
                if ui.button("+").clicked() {}
            });
            ui.label("Colors");
            ui.horizontal(|ui| {
                let library = self.library.clone();
                library
                    .library
                    .get(&self.color_scheme)
                    .unwrap()
                    .get(&self.flavor)
                    .unwrap()
                    .palette
                    .iter()
                    .for_each(|color| {
                        let r = 10.0;
                        let size = Vec2::splat(2.0 * r + 5.0);
                        let (rect, sense) = ui
                            .allocate_at_least(size, Sense::union(Sense::hover(), Sense::click()));

                        let hint_text = format!("{}: {}", color.1.name, color.1.enabled);
                        sense
                            .clone()
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .on_hover_text(hint_text);

                        let cv = color.1.value;
                        let c_view = egui::Color32::from_rgba_premultiplied(
                            (cv >> 16 & 0xFF) as u8,
                            (cv >> 8 & 0xFF) as u8,
                            (cv & 0xFF) as u8,
                            255,
                        );

                        let c = match color.1.enabled {
                            true => c_view,
                            false => c_view.gamma_multiply(0.3),
                        };

                        if sense.clicked() {
                            self.library
                                .set_color(
                                    &self.color_scheme,
                                    &self.flavor,
                                    color.0,
                                    !color.1.enabled,
                                )
                                .unwrap();
                        }

                        // ui.painter().circle_filled(rect.center(), r, c);
                        ui.painter().rect_filled(rect, 0.0, c);
                        ui.painter()
                            .rect_stroke(rect, 0.0, Stroke::new(1.0, egui::Color32::BLACK));
                    });
            });
        });
    }
}
