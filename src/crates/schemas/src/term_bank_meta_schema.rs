#![cfg(target_os = "windows")]

use std::fmt::Debug;
use serde::{de::Error, Serialize, Deserialize, Deserializer};
use serde_json;

//need to manually check FrequencyTerm, PitchTerm and TermPhoneticTranscriptions
//for if they contain a string "freq", "pitch", and "ipa" respectively

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum TermFrequency {
    Text(String),
    Number(i32),
    DisplayNumber {
        value: i32,
        #[serde(rename = "displayValue")]
        display_value: String
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ReadingFrequency {
    Term(TermFrequency),
    ReadFrequency {
        reading: String,
        frequency: TermFrequency,
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FrequencyTerm(String, String, ReadingFrequency);

#[derive(Serialize, Deserialize, Debug)]
struct PitchAccentInfo {
    position: i32,
    #[serde(deserialize_with = "from_number")]
    nasal: Option<Vec<i32>>,
    #[serde(deserialize_with = "from_number")]
    devoice: Option<Vec<i32>>,
    tags: Option<Vec<String>>
}

fn from_number<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where D: Deserializer<'de>,
{

}
