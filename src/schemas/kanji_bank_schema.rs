use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type KanjiStats = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct KanjiInformation {
    kanji: String,
    onyomi: String,
    kunyomi: String,
    tags: String,
    meanings: Vec<String>,
    stats: KanjiStats
}

pub type DictionaryKanjiBankV3 = Vec<Vec<KanjiInformation>>;