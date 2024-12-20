use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type KanjiStats = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub struct KanjiInformation {
    kanji: String,
    onyomi: String,
    kunyomi: String,
    tags: String,
    meanings: Vec<String>,
    stats: KanjiStats,
}

pub type DictionaryKanjiBankV3 = Vec<KanjiInformation>;
