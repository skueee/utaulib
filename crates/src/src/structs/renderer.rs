use encoding_rs::SHIFT_JIS;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::structs::voicebank::{Character, Voicebank};
use crate::voicebank::parser;

pub struct Renderer {
    pub voicebank: Voicebank,
}

impl Renderer {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let vb_path: PathBuf = path.as_ref().to_path_buf();

        let bytes = fs::read(vb_path.join("character.txt"))?;
        let (cow, _, _) = SHIFT_JIS.decode(&bytes);
        let character_file: String = cow.into_owned();

        let character: Character = parser::parse_character(&character_file);

        let voicebank = Voicebank { character };

        Ok(Renderer { voicebank })
    }
}
