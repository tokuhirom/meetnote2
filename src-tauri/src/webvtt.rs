use regex::Regex;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct Caption {
    pub(crate) start_time: String,
    end_time: String,
    pub(crate) text: String,
}

impl Caption {
    pub fn parse_start_time(&self) -> u32 {
        let time_parts: Vec<&str> = self.start_time.split(':').collect();
        let hour: u32 = time_parts[0].parse().unwrap();
        let minute: u32 = time_parts[1].parse().unwrap();
        let second: Vec<&str> = time_parts[2].split('.').collect();
        let sec: u32 = second[0].parse().unwrap();
        let millisec: u32 = second[1].parse().unwrap();
        millisec + sec * 1000 + minute * 60 * 1000 + hour * 60 * 60 * 1000
    }
}

impl Debug for Caption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {}, end: {}, text: {}", self.start_time, self.end_time, self.text)
    }
}

pub fn parse_webvtt(webvtt: &str) -> Vec<Caption> {
    let mut captions = Vec::new();

    // Matches "HH:MM:SS.sss --> HH:MM:SS.sss"
    let time_pattern = Regex::new(r"(\d{2}:\d{2}:\d{2}.\d{3}) --> (\d{2}:\d{2}:\d{2}.\d{3})").unwrap();

    let mut current_start_time = String::new();
    let mut current_end_time = String::new();
    let mut current_text = String::new();

    for line in webvtt.lines() {
        if time_pattern.is_match(line) {
            let caps = time_pattern.captures(line).unwrap();
            current_start_time = caps.get(1).unwrap().as_str().to_string();
            current_end_time = caps.get(2).unwrap().as_str().to_string();
        } else if (!line.is_empty()) && (line != "WEBVTT") {
            let caption = Caption {
                start_time: current_start_time.clone().to_string(),
                end_time: current_end_time.clone().to_string(),
                text: line.trim().to_string(),
            };
            captions.push(caption);
            current_text.clear();
        }
    }

    captions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_webvtt() {
        let input = "WEBVTT

00:00:00.000 --> 00:00:09.300
Caption text

00:00:09.300 --> 00:00:18.140
More caption text";

        let result = parse_webvtt(input);

        assert_eq!(
            result, vec![
                Caption { start_time: "00:00:00.000".to_string(), end_time: "00:00:09.300".to_string(), text: "Caption text".to_string() },
                Caption { start_time: "00:00:09.300".to_string(), end_time: "00:00:18.140".to_string(), text: "More caption text".to_string() }
            ]
        );
    }
}
