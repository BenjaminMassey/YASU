use eframe::egui;

#[derive(PartialEq)]
pub enum ScoreModify {
    No,
    Subtract,
    Add,
}

pub fn primary_font_id(font_size: f32) -> egui::FontId {
    egui::FontId::new(
        font_size,
        egui::FontFamily::Name("Roboto".into())
    )
}

pub fn primary_font(font_size: f32) -> egui::FontSelection {
    egui::FontSelection::FontId(
        primary_font_id(font_size)
    )
}

pub fn spacer(ui: &mut egui::Ui, width: f32, height: f32) {
    ui.add_sized(
        egui::Vec2::new(width, height),
        egui::Label::new(""),
    );
}

pub fn vertical_spacer(ui: &mut egui::Ui, height: f32) {
    spacer(ui, 1.0, height);
}

pub fn horizontal_spacer(ui: &mut egui::Ui, width: f32) {
    spacer(ui, width, 1.0)
}
