use crate::settings;

pub fn perform(settings: &settings::Settings) {
    println!("Pausing for replay saving...");
    std::thread::sleep(std::time::Duration::from_secs(settings.replay.save_delay));
    println!("Done pausing.");
    let recent = recent_path(&settings);
    if recent.is_none() {
        println!("Failed to save replay because no recent video replay file found.");
        return;
    }
    let target = crate::OUT_DIR.to_owned() + "replay" + &settings.replay.video_ext;
    let result = std::fs::copy(&recent.unwrap(), &target);
    if let Err(e) = result {
        println!("Error in replay copying: {e:?}");
    } else {
        println!("Succeeded in replay update.");
    }
}

fn recent_path(settings: &settings::Settings) -> Option<String> {
    let param = settings.replay.obs_path.to_owned() + "Replay*" + &settings.replay.video_ext;
    let vid_paths = glob::glob(&param)
        .unwrap()
        .filter_map(std::result::Result::ok);
    let mut vids = vid_paths
        .map(|p| p.into_os_string().into_string().unwrap())
        .collect::<Vec<String>>();
    vids.sort_by(|a, b| a.to_string().to_lowercase().cmp(&b.to_lowercase()));
    vids.last().cloned()
}