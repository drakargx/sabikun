#![cfg(target_os = "windows")]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum KanjiFrequency {
    Text(String),
    Number(i32),
    DisplayNumber {
        value: i32,
        #[serde(rename = displayValue)]
        display_value: String
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KanjiCharacterMetaData {
    kanji: String,
    #[serde(rename = "type")]
    freq_type: String,
    data: KanjiFrequency
}

pub type DictionaryKanjiMetaBankV3 = Vec<Vec<KanjiCharacterMetaData>>;
