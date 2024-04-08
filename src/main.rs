#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, FontId, Widget};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const OUT_DIR: &str = "./output/";
const IMG_DIR: &str = "./images/";

fn main() {
    for dir in [OUT_DIR, IMG_DIR] {
        if !Path::new(dir).exists() {
            fs::create_dir(dir).unwrap();
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
            Box::new(YasuApp::new(cc))
        }),
    );
}

#[derive(Default)]
struct YasuApp {
    player_edits: Vec<String>,
    score_edits: Vec<String>,
    info_edits: Vec<String>,
    image_select: Vec<usize>,
    image_options: Vec<String>, // non-ui element, storage
}

#[derive(PartialEq)]
enum FileType {
    Player,
    Score,
    Info,
}

impl YasuApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let image_options = fs::read_dir(IMG_DIR)
            .unwrap()
            .map(|f| f.unwrap().path().as_path().to_str().unwrap().to_owned())
            .collect::<Vec<String>>();

        for image in &image_options {
            cc.egui_ctx.include_bytes(
                "bytes://".to_owned() + image,
                egui::load::Bytes::from(fs::read(image).unwrap()),
            );
        }

        YasuApp {
            player_edits: vec![String::new()],
            score_edits: vec!["0".to_owned()],
            info_edits: vec![String::new()],
            image_select: vec![0],
            image_options,
        }
    }
    fn write_data(&self, players: bool, scores: bool, infos: bool) {
        // Text files
        for file_type in [FileType::Player, FileType::Score, FileType::Info] {
            if (file_type == FileType::Player && !players)
                || (file_type == FileType::Score && !scores)
                || (file_type == FileType::Info && !infos)
            {
                continue;
            }
            let length = if file_type == FileType::Info {
                self.info_edits.len()
            } else {
                self.player_edits.len()
            };
            for i in 0..length {
                let file_name = OUT_DIR.to_owned()
                    + match file_type {
                        FileType::Player => "player_",
                        FileType::Score => "score_",
                        FileType::Info => "info_",
                    }
                    + &(i + 1).to_string()
                    + ".txt";
                let mut file = File::options()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(&file_name)
                    .expect(&format!("Failed to open \"{}\"", file_name));
                let contents = 
                    match file_type {
                        FileType::Player => &self.player_edits,
                        FileType::Score => &self.score_edits,
                        FileType::Info => &self.info_edits,
                    }[i]
                    .clone();
                let _ = file.write_all(contents.as_bytes());
                let _ = file.flush();
            }
        }

        // Image files
        for i in 0..self.image_select.len() {
            let target = OUT_DIR.to_string() + "image_" + &(i + 1).to_string() + ".png";
            fs::copy(
                self.image_options.clone()[self.image_select.clone()[i]].clone(),
                target,
            )
            .unwrap();
        }
    }
}

fn path_to_name(path: String) -> String {
    let pieces = path.split('/').collect::<Vec<&str>>();
    if pieces.is_empty() {
        return path;
    }
    let split = pieces[pieces.len() - 1].split('.').collect::<Vec<&str>>();
    if pieces.is_empty() {
        return path;
    }
    split[0].to_owned()
}

#[derive(PartialEq)]
enum ScoreModify {
    No,
    Subtract,
    Add,
}

fn primary_font_id(font_size: f32) -> egui::FontId {
    egui::FontId::new(
        font_size,
        egui::FontFamily::Name("Roboto".into())
    )
}

fn primary_font(font_size: f32) -> egui::FontSelection {
    egui::FontSelection::FontId(
        primary_font_id(font_size)
    )
}

fn spacer(ui: &mut egui::Ui, width: f32, height: f32) {
    ui.add_sized(
        egui::Vec2::new(width, height),
        egui::Label::new(""),
    );
}

fn vertical_spacer(ui: &mut egui::Ui, height: f32) {
    spacer(ui, 1.0, height);
}

fn horizontal_spacer(ui: &mut egui::Ui, width: f32) {
    spacer(ui, width, 1.0)
}

impl eframe::App for YasuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |cui| {
            cui.add(
                egui::TextEdit::singleline(&mut "Yet Another Streaming Utility (YASU)")
                    .font(primary_font(22.0))
                    .clip_text(false)
                    .interactive(false)
            );
            vertical_spacer(cui, 1.0);
            cui.add(
                egui::TextEdit::singleline(&mut "Player Info")
                    .font(primary_font(18.0))
                    .clip_text(false)
                    .interactive(false)
            );
            cui.separator();
            let mut to_delete = vec![];
            for i in 0..self.player_edits.len() {
                cui.horizontal(|hui| {
                    hui.add_sized(
                        egui::vec2(125.0, 20.0),
                        egui::TextEdit::singleline(&mut self.player_edits[i])
                            .hint_text("Enter player name..."),
                    );
                    let mut modify = ScoreModify::No;
                    if hui
                        .add_sized(egui::vec2(15.0, 20.0), egui::Button::new("-"))
                        .clicked()
                    {
                        modify = ScoreModify::Subtract;
                    }
                    hui.add_sized(
                        egui::vec2(25.0, 20.0),
                        egui::TextEdit::singleline(&mut self.score_edits[i]),
                    );
                    if hui
                        .add_sized(egui::vec2(15.0, 20.0), egui::Button::new("+"))
                        .clicked()
                    {
                        modify = ScoreModify::Add;
                    }
                    if modify != ScoreModify::No {
                        if let Ok(score) = self.score_edits[i].parse::<i32>() {
                            let modifier = if modify == ScoreModify::Subtract {
                                -1i32
                            } else {
                                1i32
                            };
                            let new_score = score + modifier;
                            self.score_edits[i] = new_score.to_string();
                        }
                    }
                    if !(self.image_options.is_empty() || self.image_select.is_empty()) {
                        hui.add_sized(
                            egui::vec2(20.0, 20.0),
                            egui::Image::new(
                                "bytes://".to_owned() + &self.image_options[self.image_select[i]],
                            ),
                        );
                        hui.push_id(i + 77, |cui| {
                            egui::ComboBox::from_label("").show_index(
                                cui,
                                &mut self.image_select[i],
                                self.image_options.len(),
                                |j| path_to_name(self.image_options.clone()[j].clone()),
                            )
                        });
                    }
                    if self.player_edits.len() > 1
                        && hui.add(egui::Button::new("Remove Player")).clicked()
                    {
                        to_delete.push(i);
                    }
                });
            }
            for index in &to_delete {
                self.player_edits.remove(*index);
                self.score_edits.remove(*index);
            }
            cui.horizontal(|hui| {
                if hui
                    .add_sized(egui::vec2(125.0, 20.0), egui::Button::new("Add Player"))
                    .clicked()
                {
                    self.player_edits.push(String::new());
                    self.score_edits.push("0".to_owned());
                    self.image_select.push(0);
                }
                if hui.add(egui::Button::new("Zero Scores")).clicked() {
                    for i in 0..self.score_edits.len() {
                        self.score_edits[i] = "0".to_owned();
                    }
                    self.write_data(false, true, false);
                }
            });
            to_delete.clear();
            vertical_spacer(cui, 1.0);
            cui.add(
                egui::TextEdit::singleline(&mut "General Info")
                    .font(primary_font(18.0))
                    .clip_text(false)
                    .interactive(false)
            );
            cui.separator();
            for i in 0..self.info_edits.len() {
                cui.horizontal(|hui| {
                    hui.add_sized(
                        egui::vec2(200.0, 20.0),
                        egui::TextEdit::singleline(&mut self.info_edits[i])
                            .hint_text("Enter extra info..."),
                    );
                    if self.info_edits.len() > 1
                        && hui.add(egui::Button::new("Remove Info")).clicked()
                    {
                        to_delete.push(i);
                    }
                });
            }
            for index in &to_delete {
                self.info_edits.remove(*index);
            }
            if cui.add(egui::Button::new("Add Info Text")).clicked() {
                self.info_edits.push(String::new());
            }
            vertical_spacer(cui, 25.0);
            cui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                primary_font_id(22.0),
            );
            cui.horizontal(|hui| {
                horizontal_spacer(hui, 162.0);
                if hui.add_sized(
                    egui::Vec2::new(120.0, 60.0),
                    egui::Button::new("Submit")
                ).clicked() {
                    self.write_data(true, true, true);
                }
            });
        });
    }
}
