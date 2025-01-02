use serde::de::Deserializer;
use serde::ser::{Serializer, SerializeSeq};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type KanjiStats = HashMap<String, String>;

#[derive(Debug)]
pub struct KanjiInformation {
    kanji: String,
    onyomi: String,
    kunyomi: String,
    tags: String,
    meanings: Vec<String>,
    stats: KanjiStats,
}

//struct tuple used as an intermediary for serializing/deserializing
//because kanji bank has no named elements, this is needed so that serde can
//generate the correct derive macros
#[derive(Serialize, Deserialize)]
struct KanjiInfoArray(
    String,
    String,
    String,
    String,
    Vec<String>,
    KanjiStats
);

impl<'de> Deserialize<'de> for KanjiInformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let KanjiInfoArray(kanji, onyomi, kunyomi, tags, meanings, stats) =
            KanjiInfoArray::deserialize(deserializer)?;

        Ok(KanjiInformation {
            kanji,
            onyomi,
            kunyomi,
            tags,
            meanings,
            stats
        })
    }
}

impl Serialize for KanjiInformation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //TODO change to constant
        let mut seq = serializer.serialize_seq(Some(6))?;
        seq.serialize_element(&self.kanji)?;
        seq.serialize_element(&self.onyomi)?;
        seq.serialize_element(&self.kunyomi)?;
        seq.serialize_element(&self.tags)?;
        seq.serialize_element(&self.meanings)?;
        seq.serialize_element(&self.stats)?;
        seq.end()
    }
}

pub type DictionaryKanjiBankV3 = Vec<KanjiInformation>;