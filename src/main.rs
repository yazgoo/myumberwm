extern crate umberwm;

use std::process::Command;
use umberwm::{Actions, Conf, umberwm, WindowBorder, DisplayBorder, Key, CustomAction, Meta, EventsCallbacks};
use std::env;
use std::collections::HashMap;
use std::thread;
use std::path;

fn main() {

    let args: Vec<String> = env::args().collect();

    umberwm(Conf {
        /* the main key used to detect WM events */
        meta: if args.len() > 1 && args[1] == "mod4" { Meta::Mod4 } else { Meta::Mod1 },
        /* borders defining space the WM wont tile windows to (usefull when using task bars) */
        display_borders: vec![
            DisplayBorder {
                left: 0,
                right: 0,
                top: 20,
                bottom: 0,
                /* gap between windows */
                gap: 10,
            },
            DisplayBorder {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
                gap: 0,
        }
        ],
        border: WindowBorder {
            width: 1,
            focus_color: 0x906cff,
            normal_color: 0x000000,
        },
        /* key names of the workspaces (must be a name in xmodmap -pke), per displays */
        workspaces_names: vec![vec![ "a".to_string(), "u".to_string(), "i".to_string()], vec!["o".to_string(), "p".to_string() ]],
        /* mapping between key names (must be a name in xmodmap -pke) and user-defined actions */
        custom_actions: 
            vec![
            ("r".to_string(), Box::new(|| { 
                thread::spawn(
                    move || {let _ = Command::new("rofi").arg("-show").arg("run").status();}
                );
            }) as CustomAction),
            ("Return".to_string(), Box::new(|| {
                thread::spawn(
                    move || {let _= Command::new("bash").arg("t").status();} /* launch my favorite terminal emulator */
                );
            })),
            ("s".to_string(), Box::new(|| {
                thread::spawn(
                    move || {let _= Command::new("bash").arg("alsaterm").status();} /* launch alsamixer in a terminal */
                );
            })),
            ("n".to_string(), Box::new(|| {
                thread::spawn(
                    move || {let _= Command::new("xcalib").arg("-i").arg("-a").status();}
                );
            })),
            ("m".to_string(), Box::new(|| {
                let _= Command::new("autorandr").arg("--change").status();
            })),
            ("q".to_string(), Box::new(|| std::process::exit(0))),
            ].into_iter().collect::<HashMap<Key, CustomAction>>(),
        /* mapping between key names (must be a name in xmodmap -pke) and window manager specific actions */
        wm_actions: 
            vec![
            ("space".to_string(), Actions::SwitchWindow),
            ("w".to_string(), Actions::CloseWindow),
            ("f".to_string(), Actions::ChangeLayout)].into_iter().collect::<HashMap<Key, Actions>>(),
        /* won't tile windows with this WM_CLASS */
        float_classes: vec!["screenkey", "audacious", "Download", "dropbox", "file_progress", "file-roller", "gimp",
                          "Komodo_confirm_repl", "Komodo_find2", "pidgin", "skype", "Transmission", "Update", "Xephyr", "obs", "rofi", "zoom"]
                              .into_iter().map( |x| x.to_string() ).collect(),
        /* will leave alone windows with this _NET_WM_WINDOW_TYPE */
        auto_float_types: vec!["notification", "toolbar", "splash", "dialog", "popup_menu", "utility", "tooltip", "dock"]
            .into_iter().map( |x| x.to_string() ).collect(),
        /* those are user custom callbacks */
        events_callbacks: EventsCallbacks {
            /* when we change a workspace */
            on_change_workspace: Some(Box::new(|workspace, display| { 
                thread::spawn(
                    move || {
                        /* set the wallpaper using nitrogen */
                        let background_path = format!("{}/Pictures/wallpapers/umberwm_{}.jpg", 
                            env::var("HOME").unwrap(), workspace);
                        if path::Path::new(&background_path).exists() {
                            let _ = Command::new("nitrogen").arg("--set-scaled").arg(format!("--head={}", display)).arg(background_path).status();
                        }
                    }
                );
            })) 
        }
    }).run();

}
