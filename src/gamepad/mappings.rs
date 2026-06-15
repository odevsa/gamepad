use gilrs::{Axis, Button};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct RawMapping {
    pub matcher: String,
    #[serde(default)]
    pub axis_codes: HashMap<String, String>,
    #[serde(default)]
    pub axis_names: HashMap<String, String>,
    #[serde(default)]
    pub button_names: HashMap<String, String>,
    #[serde(default)]
    pub invert_axes: HashMap<String, bool>,
}

#[derive(Debug, Clone)]
pub struct Mapping {
    pub matcher: String,
    pub axis_codes: HashMap<i32, Axis>,
    pub axis_names: HashMap<String, Axis>,
    pub button_names: HashMap<String, Button>,
    pub axis_invert: std::collections::HashSet<String>,
}

fn parse_axis(s: &str) -> Option<Axis> {
    match s {
        "LeftStickX" => Some(Axis::LeftStickX),
        "LeftStickY" => Some(Axis::LeftStickY),
        "RightStickX" => Some(Axis::RightStickX),
        "RightStickY" => Some(Axis::RightStickY),
        "LeftZ" => Some(Axis::LeftZ),
        "RightZ" => Some(Axis::RightZ),
        "DPadX" => Some(Axis::DPadX),
        "DPadY" => Some(Axis::DPadY),
        "Unknown" => Some(Axis::Unknown),
        _ => None,
    }
}

fn parse_button(s: &str) -> Option<Button> {
    match s {
        "South" => Some(Button::South),
        "East" => Some(Button::East),
        "West" => Some(Button::West),
        "North" => Some(Button::North),
        "LeftTrigger" => Some(Button::LeftTrigger),
        "RightTrigger" => Some(Button::RightTrigger),
        "LeftThumb" => Some(Button::LeftThumb),
        "RightThumb" => Some(Button::RightThumb),
        "Select" => Some(Button::Select),
        "Start" => Some(Button::Start),
        "Mode" => Some(Button::Mode),
        "DPadUp" => Some(Button::DPadUp),
        "DPadDown" => Some(Button::DPadDown),
        "DPadLeft" => Some(Button::DPadLeft),
        "DPadRight" => Some(Button::DPadRight),
        _ => None,
    }
}

pub fn load_mappings_from_dir<P: AsRef<std::path::Path>>(dir: P) -> Vec<Mapping> {
    let mut out = Vec::new();

    let rd = match fs::read_dir(dir) {
        Ok(it) => it,
        Err(_) => return out,
    };

    for entry in rd.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let s = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[gamepad] failed to read mapping file {:?}: {}", path, e);
                continue;
            }
        };

        match serde_json::from_str::<RawMapping>(&s) {
            Ok(raw) => {
                let mut axis_codes = HashMap::new();
                for (k, v) in raw.axis_codes.iter() {
                    if let Ok(n) = k.parse::<i32>() {
                        if let Some(ax) = parse_axis(v) {
                            axis_codes.insert(n, ax);
                        }
                    }
                }

                let mut axis_names = HashMap::new();
                for (k, v) in raw.axis_names.iter() {
                    if let Some(src) = parse_axis(k) {
                        if let Some(dst) = parse_axis(v) {
                            axis_names.insert(format!("{:?}", src), dst);
                        }
                    }
                }

                let mut button_names = HashMap::new();
                for (k, v) in raw.button_names.iter() {
                    if let Some(dst) = parse_button(v) {
                        button_names.insert(k.clone(), dst);
                    }
                }

                let mut axis_invert = std::collections::HashSet::new();
                for (k, v) in raw.invert_axes.iter() {
                    if *v {
                        axis_invert.insert(k.clone());
                    }
                }

                out.push(Mapping {
                    matcher: raw.matcher,
                    axis_codes,
                    axis_names,
                    button_names,
                    axis_invert,
                });
            }
            Err(e) => {
                eprintln!("[gamepad] failed to parse mapping file {:?}: {}", path, e);
            }
        }
    }

    out
}

pub fn find_mapping<'a>(mappings: &'a [Mapping], device_name: &str) -> Option<&'a Mapping> {
    mappings.iter().find(|m| device_name.contains(&m.matcher))
}
