use std::path::PathBuf;

pub struct Voicebank {
    pub path: PathBuf,
    pub character: Character,
    pub subbanks: Vec<Subbank>
}

#[derive(Debug)]
pub struct Character {
    pub name: Option<String>,
    pub author: Option<String>,
    pub image: Option<String>,
    pub web: Option<String>,
    pub version: Option<String>,
    pub voice: Option<String>,
}

// Structure to store a voice data
pub struct Voice {
    pub id: String, // id is a name used to refer to the voice, that is the color + the range or the prefix (if any) + the range + the suffix (if any)
    pub prefix: String,
    pub suffix: String,
    pub color: String,
    pub range: String,
    pub otos: Vec<Oto>,
    pub data: VoiceData,
}

pub struct Oto {
    pub path: PathBuf,
}

pub enum VoiceData {
    Unloaded,
    Loaded,
}
