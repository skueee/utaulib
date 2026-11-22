use std::str::FromStr;

use crate::structs::voicebank::{self, Character, Voice, VoiceData};
use saphyr::{LoadableYamlNode, Yaml};

pub fn parse_character(content: &String) -> voicebank::Character {
    let mut character = Character {
        name: None,
        author: None,
        image: None,
        web: None,
        version: None,
        voice: None,
    };

    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "name" => character.name = Some(value.to_string()),
                "author" => character.author = Some(value.to_string()),
                "image" => character.image = Some(value.to_string()),
                "web" => character.web = Some(value.to_string()),
                "version" => character.version = Some(value.to_string()),
                "voice" => character.voice = Some(value.to_string()),
                _ => {}
            }
        }
    }

    character
}

pub fn parse_voices_from_character(content_str: &String) -> Vec<Voice> {
    let mut voices = Vec::new();

    let yaml = match Yaml::load_from_str(content_str) {
        Ok(d) => d,
        Err(_) => return voices
    };

    let content = &yaml[0];

    if let Some(subbanks) = content["subbanks"].as_vec() {
        for subbank in subbanks {
            let color = subbank["color"].as_str().unwrap_or_default();
            let prefix = subbank["prefix"].as_str().unwrap_or_default();
            let suffix = subbank["suffix"].as_str().unwrap_or_default();

            let range = subbank["tone_ranges"].as_vec().and_then(|v| v.first()).and_then(|item| item.as_str()).unwrap_or_default();

            let id = if !color.is_empty() {
                format!("{color}_{range}")
            } else if !prefix.is_empty() {
                if !suffix.is_empty() {
                    format!("{prefix}_{range}_{suffix}")
                } else {
                    format!("{prefix}_{range}")
                }
            } else if !suffix.is_empty() {
                format!("{range}_{suffix}")
            } else {
                format!("{range}")
            };

            voices.push(Voice {
                id,
                prefix: prefix.to_string(),
                suffix: suffix.to_string(),
                color: color.to_string(),
                range: range.to_string(),
                path: "",
                data: VoiceData::Unloaded,
            });
        }
    };

    voices
}
