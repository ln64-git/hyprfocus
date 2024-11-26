use clap::Parser;
use hyprfocus::Args;
use std::fs::{self};
use std::process::Command;
use utils::hyprctl::{get_open_windows, get_workspace_id};
mod utils {
    pub mod hyprctl;
}

fn main() {
    let args = Args::parse();
    let command = args.command;
    if args.launch {
        launch_application(&command);
    } else {
        focus_or_launch_application(&command);
    }
}

fn focus_or_launch_application(command: &str) {
    let app_class = match command {
        "code" => "Code",
        "obs" => "com.obsproject.Studio",
        _ => command,
    };

    let windows = get_open_windows(app_class);
    if !windows.is_empty() {
        cycle_through_windows(app_class, &windows);
    } else {
        launch_application(command);
    }
}

fn launch_application(command: &str) {
    println!("Launching {}", command);
    if let Err(e) = Command::new(command).spawn() {
        eprintln!("Failed to launch {}: {}", command, e);
    }
}

fn cycle_through_windows(app_class: &str, windows: &[String]) {
    let tmp_file = format!("/tmp/{}_last_window_id", app_class);
    let last_window_id = fs::read_to_string(&tmp_file).unwrap_or_default();

    let mut next_window_id = String::new();
    let mut found_last = false;

    for window_id in windows {
        if found_last {
            next_window_id = window_id.clone();
            break;
        }
        if *window_id == last_window_id {
            found_last = true;
        }
    }

    if next_window_id.is_empty() {
        next_window_id = windows.first().unwrap().clone();
    }

    let workspace_id = get_workspace_id(&next_window_id);
    if let Some(workspace_id) = workspace_id {
        Command::new("hyprctl")
            .arg("dispatch")
            .arg("workspace")
            .arg(workspace_id)
            .output()
            .expect("Failed to switch workspace");
    }

    Command::new("hyprctl")
        .arg("dispatch")
        .arg("focuswindow")
        .arg(&next_window_id)
        .output()
        .expect("Failed to focus window");

    fs::write(tmp_file, next_window_id).expect("Failed to write to tmp file");
}
