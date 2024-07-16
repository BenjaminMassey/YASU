use crate::gui;
use crate::util;

use eframe::egui;

#[derive(Default)]
pub struct YasuApp {
    pub player_edits: Vec<String>,
    pub score_edits: Vec<String>,
    pub info_edits: Vec<String>,
    pub image_select: Vec<usize>,
    pub image_options: Vec<String>, // non-ui element, storage
}
impl YasuApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let image_options = std::fs::read_dir(crate::IMG_DIR)
            .unwrap()
            .map(|f| f.unwrap().path().as_path().to_str().unwrap().to_owned())
            .collect::<Vec<String>>();

        for image in &image_options {
            cc.egui_ctx.include_bytes(
                "bytes://".to_owned() + image,
                egui::load::Bytes::from(std::fs::read(image).unwrap()),
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
}

impl eframe::App for YasuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |cui| {
            cui.add(
                egui::TextEdit::singleline(&mut "Yet Another Streaming Utility (YASU)")
                    .font(gui::primary_font(22.0))
                    .clip_text(false)
                    .interactive(false)
            );
            gui::vertical_spacer(cui, 1.0);
            cui.add(
                egui::TextEdit::singleline(&mut "Player Info")
                    .font(gui::primary_font(18.0))
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
                    let mut modify = gui::ScoreModify::No;
                    if hui
                        .add_sized(egui::vec2(15.0, 20.0), egui::Button::new("-"))
                        .clicked()
                    {
                        modify = gui::ScoreModify::Subtract;
                    }
                    hui.add_sized(
                        egui::vec2(25.0, 20.0),
                        egui::TextEdit::singleline(&mut self.score_edits[i]),
                    );
                    if hui
                        .add_sized(egui::vec2(15.0, 20.0), egui::Button::new("+"))
                        .clicked()
                    {
                        modify = gui::ScoreModify::Add;
                    }
                    if modify != gui::ScoreModify::No {
                        if let Ok(score) = self.score_edits[i].parse::<i32>() {
                            let modifier = if modify == gui::ScoreModify::Subtract {
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
                                |j| util::path_to_name(self.image_options.clone()[j].clone()),
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
                    util::write_data(&self, false, true, false);
                }
                if self.player_edits.len() == 2 &&
                    hui.add(egui::Button::new("Swap Players")).clicked() {
                    util::swap_first_second_player(self);
                    util::write_data(&self, true, true, false);
                }
            });
            to_delete.clear();
            gui::vertical_spacer(cui, 1.0);
            cui.add(
                egui::TextEdit::singleline(&mut "General Info")
                    .font(gui::primary_font(18.0))
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
            gui::vertical_spacer(cui, 25.0);
            cui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                gui::primary_font_id(22.0),
            );
            cui.horizontal(|hui| {
                gui::horizontal_spacer(hui, 162.0);
                if hui.add_sized(
                    egui::Vec2::new(120.0, 60.0),
                    egui::Button::new("Submit")
                ).clicked() {
                    util::write_data(&self, true, true, true);
                }
            });
        });
    }
}