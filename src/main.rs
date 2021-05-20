extern crate umberwm;

use std::env;
use std::path;
use std::process::Command;
use std::thread;
use umberwm::{
    umberwm, Actions, Conf, CustomAction, DisplayBorder, EventsCallbacks, Key, Keybind, WindowBorder,
    MOD_MASK_1, MOD_MASK_4, MOD_MASK_CONTROL, MOD_MASK_SHIFT,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let meta = if args.len() > 1 && args[1] == "mod4" { MOD_MASK_4 } else { MOD_MASK_1 };

    umberwm(Conf {
        /* the main key used to detect WM events */
        meta: meta,
        /* borders defining space the WM wont tile windows to (usefull when using task bars) */
        display_borders: vec![
            DisplayBorder {
                left: 0,
                right: 0,
                top: 16,
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
        workspaces_names: vec![
            vec![ "a".to_string(), "u".to_string(), "i".to_string()],
            vec!["b".to_string(), "eacute".to_string(), "o".to_string(), "p".to_string() ]],
        /* mapping between key names (must be a name in xmodmap -pke) and user-defined actions */
        custom_actions: 
            vec![
            (Keybind::new(meta, "r"), Box::new(|| { 
                thread::spawn(
                    move || {let _ = Command::new("rofi").arg("-show").arg("run").status();}
                );
            }) as CustomAction),
            (Keybind::new(meta, "Return"), Box::new(|| {
                thread::spawn(
                    move || {let _= Command::new("bash").arg("t").status();} /* launch my favorite terminal emulator */
                );
            })),
            (Keybind::new(meta, "s"), Box::new(|| {
                thread::spawn(
                    move || {let _= Command::new("bash").arg("alsaterm").status();} /* launch alsamixer in a terminal */
                );
            })),
            (Keybind::new(meta, "l"), Box::new(|| {
                thread::spawn(
                    move || {let _= Command::new("lxlock");}
                );
            })),
            (Keybind::new(meta, "n"), Box::new(|| {
                thread::spawn(
                    move || {let _= Command::new("xcalib").arg("-i").arg("-a").status();}
                );
            })),
            (Keybind::new(meta, "x"), Box::new(|| {
                thread::spawn(
                    move || {let _= Command::new("t").arg("--class").arg("quickmarks").arg("--config-file").arg("/home/yazgoo/.config/alacritty/alacritty_white.yml").arg("-e").arg("quickmarks").status();}
                );
            })),
            (Keybind::new(meta, "m"), Box::new(|| {
                let _= Command::new("autorandr").arg("--change").status();
            })),
            (Keybind::new(meta, "q"), Box::new(|| std::process::exit(0))),
            ].into_iter().collect(),
        /* mapping between key names (must be a name in xmodmap -pke) and window manager specific actions */
        wm_actions: 
            vec![
            (Keybind::new(meta, "space"), Actions::SwitchWindow),
            (Keybind::new(meta, "w"), Actions::CloseWindow),
            (Keybind::new(meta, "f"), Actions::ChangeLayout),
            (Keybind::new(meta, "d"), Actions::SerializeAndQuit),
            (Keybind::new(meta, "g"), Actions::ToggleGap)].into_iter().collect(),
        /* won't tile windows with this WM_CLASS */
        ignore_classes: vec!["xscreensaver", "Discover-overlay", "compton", "xsecurelock"]
            .into_iter().map( |x| x.to_string() ).collect(),
        float_classes: vec!["confirm", "dialog", "error", "splash", "toolbar", "screenkey", "audacious", "Download", "dropbox", "file_progress", "file-roller",
                          "Komodo_confirm_repl", "Komodo_find2", "pidgin", "skype", "Transmission", "Update", "Xephyr", "obs", "rofi", "xscreensaver", "quickmarks", "discover-overlay", "Discover-overlay"]
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
        },
        with_gap: false,
    }).run();
}
