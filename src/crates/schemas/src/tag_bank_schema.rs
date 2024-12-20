#![cfg(target_os = "windows")]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagInformation {
    tag_name: String,
    category: String,
    sorting_order: i32,
    notes: String,
    popularity_score: i32
}

pub type DictionaryTagBankV3 = Vec<Vec<TagInformation>>;
