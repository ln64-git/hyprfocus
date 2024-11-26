use crate::Command;
use serde_json::Value;

pub fn get_open_windows(app_class: &str) -> Vec<String> {
    let output = Command::new("hyprctl")
        .arg("clients")
        .arg("-j")
        .output()
        .expect("Failed to execute hyprctl");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let clients: Value = serde_json::from_str(&stdout).expect("Failed to parse JSON");

    clients
        .as_array()
        .unwrap()
        .iter()
        .filter(|client| client["class"].as_str().unwrap().contains(app_class))
        .map(|client| client["address"].as_str().unwrap().to_string())
        .collect()
}

pub fn get_workspace_id(window_id: &str) -> Option<String> {
    let output = Command::new("hyprctl")
        .arg("clients")
        .arg("-j")
        .output()
        .expect("Failed to execute hyprctl");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let clients: Value = serde_json::from_str(&stdout).expect("Failed to parse JSON");

    clients
        .as_array()
        .unwrap()
        .iter()
        .find(|client| client["address"].as_str().unwrap() == window_id)
        .and_then(|client| client["workspace"]["id"].as_str().map(|s| s.to_string()))
}
