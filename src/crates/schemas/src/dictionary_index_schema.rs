use serde::{Serialize, Deserialize};
use serde_repr::*;
use crate::iso_languages::IsoLanguageCode;

#[derive(Serialize, Deserialize, Debug)]
pub struct TagInfo {
    category: Option<String>,
    order: Option<i32>,
    notes: Option<String>,
    score: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum FrequencyMode {
    OccurenceBased,
    RankBased
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum Format {
    #[serde(rename = "1")]
    V1 = 1,
    #[serde(rename = "2")]
    V2 = 2,
    #[serde(rename = "3")]
    V3 = 3
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictionaryIndex {
    title: String,
    revision: String,
    sequenced: Option<bool>,
    format: Option<Format>,
    author: Option<String>,
    is_updatable: Option<bool>,
    index_url: Option<String>,
    download_url: Option<String>,
    url: Option<String>,
    description: Option<String>,
    attribution: Option<String>,
    source_language: Option<IsoLanguageCode>,
}
