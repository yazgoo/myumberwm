extern crate umberwm;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path;
use std::process::Command;
use std::thread;
use umberwm::{
    umberwm, Conf, CustomAction, EventsCallbacks, Keybind, SerializableConf, MOD_MASK_1, MOD_MASK_4,
};

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
    let args: Vec<String> = env::args().collect();
    let meta = if args.len() > 1 && args[1] == "mod4" {
        MOD_MASK_4
    } else {
        MOD_MASK_1
    };
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
        custom_actions: vec![
            (
                Keybind::new(meta, "r"),
                Box::new(|| {
                    thread::spawn(move || {
                        let _ = Command::new("rofi").arg("-show").arg("run").status();
                    });
                }) as CustomAction,
            ),
            (
                Keybind::new(meta, "Return"),
                Box::new(|| {
                    thread::spawn(move || {
                        let _ = Command::new("bash").arg("t").status();
                    } /* launch my favorite terminal emulator */);
                }),
            ),
            (
                Keybind::new(meta, "s"),
                Box::new(|| {
                    thread::spawn(move || {
                        let _ = Command::new("bash").arg("alsaterm").status();
                    } /* launch alsamixer in a terminal */);
                }),
            ),
            (
                Keybind::new(meta, "l"),
                Box::new(|| {
                    thread::spawn(move || {
                        let _ = Command::new("lxlock");
                    });
                }),
            ),
            (
                Keybind::new(meta, "n"),
                Box::new(|| {
                    thread::spawn(move || {
                        let _ = Command::new("xcalib").arg("-i").arg("-a").status();
                    });
                }),
            ),
            (
                Keybind::new(meta, "x"),
                Box::new(|| {
                    thread::spawn(move || {
                        let _ = Command::new("t")
                            .arg("--class")
                            .arg("quickmarks")
                            .arg("--config-file")
                            .arg("/home/yazgoo/.config/alacritty/alacritty_white.yml")
                            .arg("-e")
                            .arg("quickmarks")
                            .status();
                    });
                }),
            ),
            (
                Keybind::new(meta, "m"),
                Box::new(|| {
                    let _ = Command::new("autorandr").arg("--change").status();
                }),
            ),
        ]
        .into_iter()
        .collect(),
    })
    .run();
}
