use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[clap(long, default_value_t = String::from("C:\\Users\\User\\Videos\\"))]
    pub obs_replay_path: String,
    #[clap(long, default_value_t = String::from(".mp4"))]
    pub obs_video_ext: String,
    #[clap(long, default_value_t = 4u64)]
    pub replay_save_delay: u64,
}

pub fn perform() {
    let args = Args::parse();
    println!("Pausing for replay saving...");
    std::thread::sleep(std::time::Duration::from_secs(args.replay_save_delay));
    println!("Done pausing.");
    let recent = recent_path(&args);
    if recent.is_none() {
        println!("Failed to save replay because no recent video replay file found.");
        return;
    }
    let target = crate::OUT_DIR.to_owned() + "replay" + &args.obs_video_ext;
    let result = std::fs::copy(&recent.unwrap(), &target);
    if let Err(e) = result {
        println!("Error in replay copying: {e:?}");
    } else {
        println!("Succeeded in replay update.");
    }
}

fn recent_path(args: &Args) -> Option<String> {
    let param = args.obs_replay_path.to_owned() + "Replay*" + &args.obs_video_ext;
    let vid_paths = glob::glob(&param)
        .unwrap()
        .filter_map(std::result::Result::ok);
    let mut vids = vid_paths
        .map(|p| p.into_os_string().into_string().unwrap())
        .collect::<Vec<String>>();
    vids.sort_by(|a, b| a.to_string().to_lowercase().cmp(&b.to_lowercase()));
    vids.last().cloned()
}