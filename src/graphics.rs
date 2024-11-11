use crate::settings;

use std::io::Write;

const GRAPHIC_BACKGROUND: &str = "./graphics/graphic_background.png";
const PLAYER_PICTURES_DIRECTORY: &str = "./players/";

pub fn output_graphic(
    player_name: &str,
    icon_file: &str,
    output_path: &str,
    settings: &settings::Settings
) {
    let base = raster::open(GRAPHIC_BACKGROUND).expect("failed to open base graphic background");
    let mut player_image_filename = PLAYER_PICTURES_DIRECTORY.to_owned() + &player_name.to_lowercase() + ".png";
    if !std::path::Path::new(&player_image_filename).exists() {
        player_image_filename = PLAYER_PICTURES_DIRECTORY.to_owned() + "default.png";
    }
    let mut player_image = raster::open(&player_image_filename).unwrap();
    if settings.graphics.player_image_fill {
        let _ = raster::transform::resize_fill(
            &mut player_image,
            settings.graphics.player_image_size[0] as i32,
            settings.graphics.player_image_size[1] as i32,
        );
    }
    else {
        let _ = raster::transform::resize_fit(
            &mut player_image,
            settings.graphics.player_image_size[0] as i32,
            settings.graphics.player_image_size[1] as i32,
        );
    }
    let with_image = raster::editor::blend(
        &base,
        &mut player_image,
        raster::BlendMode::Normal,
        1.0,
        raster::PositionMode::Center,
        0,
        0,
    ).expect("failed to put player image on graphic");
    let mut with_icon = raster::editor::blend(
        &with_image,
        &mut raster::open(&icon_file).unwrap(),
        raster::BlendMode::Normal,
        1.0,
        raster::PositionMode::Center,
        0,
        ((player_image.height as f32 * 0.5f32) + 100f32) as i32,
    ).expect("failed to put icon on graphic");
    let with_text = text_on_image(
        player_name,
        &mut with_icon,
        0xFFFFFF,
        raster::PositionMode::Center,
        (0, ((player_image.height as f32 * -0.5f32) - 100f32) as i32),
    );
    raster::save(&with_text, output_path).unwrap();
}

fn text_on_image(
    text: &str,
    image: &mut raster::Image,
    color: u32,
    pos: raster::PositionMode,
    offset: (i32, i32)
) -> raster::Image {
    let temp_file = "./text.png";
    let renderer =
        text_to_png::TextRenderer::try_new_with_ttf_font_data(include_bytes!("../fonts/Roboto-Regular.ttf"))
        .expect("Failed to load font");
    let rendered_text = renderer.render_text_to_png_data(text, 128, color).expect("Failed to text_to_png");
    let mut text_file = std::fs::File::options()
        .create(true)
        .write(true)
        .open(temp_file)
        .expect("Couldn't make or write text.png");
    let _ = text_file.write_all(&rendered_text.data);
    let _ = text_file.flush();
    let image_with_text = raster::editor::blend(
        &image,
        &mut raster::open(temp_file).unwrap(),
        raster::BlendMode::Normal,
        1.0,
        pos,
        offset.0,
        offset.1,
    ).expect("failed to put text on image");
    let _ = std::fs::remove_file(temp_file);
    image_with_text
}