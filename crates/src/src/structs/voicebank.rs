pub struct Voicebank {
    pub character: Character
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
    pub path: String,
    pub data: VoiceData,
}

pub enum VoiceData {
    Unloaded,
    Loaded,
}
