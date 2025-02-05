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

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
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
    pub title: String,
    pub revision: String,
    pub sequenced: Option<bool>,
    pub format: Option<Format>,
    pub author: Option<String>,
    pub is_updatable: Option<bool>,
    pub index_url: Option<String>,
    pub download_url: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub attribution: Option<String>,
    pub source_language: Option<IsoLanguageCode>,
}
