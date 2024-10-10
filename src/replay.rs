const IN_VIDEOS_PATH: &str = "C:\\Users\\User\\Videos\\";
const VIDEO_EXT: &str = ".mp4";
const PAUSE_TIME: u64 = 4;

pub fn perform() {
    println!("Pausing for replay saving...");
    std::thread::sleep(std::time::Duration::from_secs(PAUSE_TIME));
    println!("Done pausing.");
    let recent = recent_path();
    if recent.is_none() {
        println!("Failed to save replay because no recent video replay file found.");
        return;
    }
    let target = crate::OUT_DIR.to_owned() + "replay" + VIDEO_EXT;
    let result = std::fs::copy(&recent.unwrap(), &target);
    if let Err(e) = result {
        println!("Error in replay copying: {e:?}");
    } else {
        println!("Succeeded in replay update.");
    }
}

fn recent_path() -> Option<String> {
    let param = IN_VIDEOS_PATH.to_owned() + "Replay*" + VIDEO_EXT;
    let vid_paths = glob::glob(&param)
        .unwrap()
        .filter_map(std::result::Result::ok);
    let mut vids = vid_paths
        .map(|p| p.into_os_string().into_string().unwrap())
        .collect::<Vec<String>>();
    vids.sort_by(|a, b| a.to_string().to_lowercase().cmp(&b.to_lowercase()));
    vids.last().cloned()
}