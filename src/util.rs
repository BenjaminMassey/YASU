use crate::app;

use std::io::Write;

#[derive(PartialEq)]
enum FileType {
    Player,
    Score,
    Info,
}

pub fn path_to_name(path: String) -> String {
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

pub fn swap_first_second_player(yasu: &mut app::YasuApp) {
    assert_eq!(yasu.player_edits.len(), 2);
    let temp: (String, String, usize) = (
        yasu.player_edits[0].clone(),
        yasu.score_edits[0].clone(),
        yasu.image_select[0],
    );
    yasu.player_edits.remove(0);
    yasu.score_edits.remove(0);
    yasu.image_select.remove(0);
    yasu.player_edits.push(temp.0.to_owned());
    yasu.score_edits.push(temp.1.to_owned());
    yasu.image_select.push(temp.2);
}

pub fn write_data(yasu: &app::YasuApp, players: bool, scores: bool, infos: bool) {
    // Text files
    for file_type in [FileType::Player, FileType::Score, FileType::Info] {
        if (file_type == FileType::Player && !players)
            || (file_type == FileType::Score && !scores)
            || (file_type == FileType::Info && !infos)
        {
            continue;
        }
        let length = if file_type == FileType::Info {
            yasu.info_edits.len()
        } else {
            yasu.player_edits.len()
        };
        for i in 0..length {
            let file_name = crate::OUT_DIR.to_owned()
                + match file_type {
                    FileType::Player => "player_",
                    FileType::Score => "score_",
                    FileType::Info => "info_",
                }
                + &(i + 1).to_string()
                + ".txt";
            let mut file = std::fs::File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&file_name)
                .expect(&format!("Failed to open \"{}\"", file_name));
            let contents = 
                match file_type {
                    FileType::Player => &yasu.player_edits,
                    FileType::Score => &yasu.score_edits,
                    FileType::Info => &yasu.info_edits,
                }[i]
                .clone();
            let _ = file.write_all(contents.as_bytes());
            let _ = file.flush();
        }
    }

    // Image files
    for i in 0..yasu.image_select.len() {
        let target = crate::OUT_DIR.to_string() + "image_" + &(i + 1).to_string() + ".png";
        std::fs::copy(
            yasu.image_options.clone()[yasu.image_select.clone()[i]].clone(),
            target,
        )
        .unwrap();
    }
}