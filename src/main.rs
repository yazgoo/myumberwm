extern crate umberwm;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path;
use std::process::Command;
use std::thread;
use umberwm::model::{Conf, EventsCallbacks, SerializableConf};
use umberwm::umberwm;

fn save_local_conf() {
    let conf_str = include_str!("umberwm.ron");
    let conf_path = format!(
        "{}/umberwm.ron",
        dirs::config_dir().unwrap().to_str().unwrap()
    );
    let mut file = File::create(conf_path).unwrap();
    write!(file, "{}", conf_str).unwrap();
}

fn main() {
    save_local_conf();
    let serializable = SerializableConf::load().unwrap();
    umberwm(Conf {
        serializable: serializable,
        events_callbacks: EventsCallbacks {
            /* when we change a workspace */
            on_change_workspace: Some(Box::new(|workspace, display| {
                thread::spawn(move || {
                    /* set the wallpaper using nitrogen */
                    let background_path = format!(
                        "{}/Pictures/wallpapers/umberwm_{}.jpg",
                        env::var("HOME").unwrap(),
                        workspace
                    );
                    if path::Path::new(&background_path).exists() {
                        let _ = Command::new("nitrogen")
                            .arg("--set-scaled")
                            .arg(format!("--head={}", display))
                            .arg(background_path)
                            .status();
                    }
                });
            })),
        },
        /* mapping between key names (must be a name in xmodmap -pke) and user-defined actions */
        custom_actions: vec![].into_iter().collect(),
    })
    .run();
}
