use std::path::{Path, PathBuf};
use std::io;
use std::fs;
use encoding_rs::SHIFT_JIS;

use crate::structs::voicebank::Character;
use crate::voicebank::parser;

pub struct Renderer {
     pub character: Character
}

impl Renderer {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let vb_path: PathBuf = path.as_ref().to_path_buf();

        let bytes = fs::read(vb_path.join("character.txt"))?;
        let (cow, _, _) = SHIFT_JIS.decode(&bytes);
        let character_file: String = cow.into_owned();

        let character = parser::parse_character(&character_file);

        Ok(Renderer {character})
    }
}
