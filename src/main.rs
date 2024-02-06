#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::fs::create_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let _ = eframe::run_native(
        "YASU Window",
        eframe::NativeOptions {
            initial_window_size: Option::from(egui::Vec2::new(325f32, 325f32)),
            ..Default::default()
        },
        Box::new(|cc| Box::new(YasuApp::new(cc))),
    );
}

#[derive(Default)]
struct YasuApp {
    player_edits: Vec<String>,
    score_edits: Vec<String>,
    info_edits: Vec<String>,
}

#[derive(PartialEq)]
enum FileType {
    Player,
    Score,
    Info,
}

impl YasuApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        YasuApp {
            player_edits: vec![String::new()],
            score_edits: vec!["0".to_owned()],
            info_edits: vec![String::new()],
        }
    }
    fn write_data(&self, players: bool, scores: bool, infos: bool) {
        let text_folder = "./text/";
        if !Path::new(text_folder).exists() {
            let _ = create_dir(text_folder).unwrap();
        }
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
                let file_name = text_folder.to_owned() +
                    match file_type {
                        FileType::Player => "player_",
                        FileType::Score => "score_",
                        FileType::Info => "info_",
                    }
                    + &(i + 1).to_string()
                    + ".txt";
                if !Path::new(&file_name).exists() {
                    File::create(&file_name).unwrap();
                }
                let mut file = File::options().write(true).open(file_name).unwrap();
                let _ = writeln!(
                    &mut file,
                    "{}",
                    match file_type {
                        FileType::Player => &self.player_edits,
                        FileType::Score => &self.score_edits,
                        FileType::Info => &self.info_edits,
                    }[i]
                        .clone()
                );
                file.flush().unwrap();
            }
        }
    }
}

#[derive(PartialEq)]
enum ScoreModify {
    No,
    Subtract,
    Add,
}

impl eframe::App for YasuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |cui| {
            cui.heading("YASU");
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
                if hui.add_sized(
                      egui::vec2(125.0, 20.0), 
                      egui::Button::new("Add Player")
                   )
                   .clicked() {
                      self.player_edits.push(String::new());
                      self.score_edits.push("0".to_owned());
                }
                if hui.add(egui::Button::new("Zero Scores")).clicked() {
                    for i in 0..self.score_edits.len() {
                        self.score_edits[i] = "0".to_owned();
                    }
                    self.write_data(false, true, false);
                }
            });
            cui.separator();
            to_delete.clear();
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
            cui.separator();
            if cui.add(egui::Button::new("Submit")).clicked() {
                self.write_data(true, true, true);
            }
        });
    }
}
