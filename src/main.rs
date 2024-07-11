#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod gui;
mod util;

use eframe::egui;

const OUT_DIR: &str = "./output/";
const IMG_DIR: &str = "./images/";

fn main() {
    for dir in [OUT_DIR, IMG_DIR] {
        if !std::path::Path::new(dir).exists() {
            std::fs::create_dir(dir).unwrap();
        }
    }

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Roboto".to_owned(),
        egui::FontData::from_static(include_bytes!("../fonts/Roboto-Regular.ttf"))
    );
    fonts.families.insert(egui::FontFamily::Name("Roboto".into()), vec!["Roboto".to_owned()]);

    let _ = eframe::run_native(
        "YASU Window",
        eframe::NativeOptions {
            viewport: egui::viewport::ViewportBuilder {
                inner_size: Option::from(egui::Vec2::new(475f32, 475f32)),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let _ = &cc.egui_ctx.set_fonts(fonts);
            Box::new(app::YasuApp::new(cc))
        }),
    );
}