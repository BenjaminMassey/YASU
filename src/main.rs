#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::Vec2;
use std::fs;
use std::collections::HashMap;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    /*native_options.initial_window_size = Option::from(
        Vec2::new(765f32, 350f32)
    );*/
    let _ = eframe::run_native(
        "YASU Window",
        native_options,
        Box::new(|cc| Box::new(YasuApp::new(cc)))
    );
}

#[derive(Default)]
struct YasuApp {
    player_edits: Vec<String>,
    score_edits: Vec<String>,
}

impl YasuApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self { 
        YasuApp {
            player_edits: vec![String::new()],
            score_edits: vec!["0".to_owned()],
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
                            .hint_text("Enter player name...")
                    );
                    let mut modify = ScoreModify::No;
                    if hui.add_sized(
                        egui::vec2(15.0, 20.0),
                        egui::Button::new("-")
                    ).clicked() {
                        modify = ScoreModify::Subtract;
                    }
                    hui.add_sized(
                        egui::vec2(25.0, 20.0),
                        egui::TextEdit::singleline(&mut self.score_edits[i])
                    );
                    if hui.add_sized(
                        egui::vec2(15.0, 20.0),
                        egui::Button::new("+")
                    ).clicked() {
                        modify = ScoreModify::Add;
                    }
                    if modify != ScoreModify::No {
                        let score = self.score_edits[i].parse::<i32>();
                        if score.is_ok() {
                            let modifier = if modify == ScoreModify::Subtract { -1i32 } else { 1i32 };
                            let new_score = score.unwrap() + modifier;
                            self.score_edits[i] = new_score.to_string();
                        }
                    }
                    if self.player_edits.len() > 1 {
                        if hui.add(egui::Button::new("Remove Player")).clicked() {
                            to_delete.push(i);
                        }
                    }
                });
            }
            for index in &to_delete {
                self.player_edits.remove(*index);
                self.score_edits.remove(*index);
            }
            cui.separator();
            if cui.add(egui::Button::new("Add Player")).clicked() {
                self.player_edits.push(String::new());
                self.score_edits.push("0".to_owned());
            }
        });
    }
}