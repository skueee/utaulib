use crate::structs::voicebank::{self, Character};

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
