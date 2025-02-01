use serde::{Deserialize, Serialize};
use serde::de::{Deserializer, Error, SeqAccess, Unexpected, Visitor};
use serde::ser::{Serializer, SerializeSeq};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum FrequencyData {
    Text(String),
    Number(i32),
    DisplayNumber {
        value: i32,
        display_value: Option<String>,
        reading: Option<String>,
    }
}

#[derive(Debug)]
pub struct KanjiMetaData {
    character: String,
    mode: String,
    data: FrequencyData
}

#[derive(Serialize, Deserialize)]
struct KanjiMetaDataArray(
    String,
    String,
    FrequencyData
);

impl<'de> Deserialize<'de> for KanjiMetaData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {

        let KanjiMetaDataArray(character, mode, data) = 
            KanjiMetaDataArray::deserialize(deserializer)?;

        //mode must say "freq"
        if mode != "freq" {
            Err(<D::Error as Error>::invalid_value(Unexpected::Str(mode.as_str()), &"freq"))
        }
        else {
            Ok(KanjiMetaData { character, mode, data})
        }
    }
}

impl Serialize for KanjiMetaData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.character)?;
        seq.serialize_element(&self.mode)?;
        seq.serialize_element(&self.data)?;    
        seq.end()
    }
}

pub type DictionaryKanjiMetaBankV3 = Vec<KanjiMetaData>;

#[derive(Debug, Serialize, Deserialize)]
pub enum TermMetaFrequencyData {
    Generic(FrequencyData),
    WithReading{
        reading: String,
        frequency: FrequencyData
    }
}

#[derive(Debug)]
pub struct TermMetaFrequency {
    expression: String,
    mode: String,
    data: TermMetaFrequencyData
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumOrArray {
    Number(i32),
    Array(Vec<i32>)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PitchData {
    position: i32,
    nasal: Option<NumOrArray>,
    devoice: Option<NumOrArray>,
    tags: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TermMetaPitchData {
    reading: String,
    pitches: Vec<PitchData>
}

#[derive(Debug)]
pub struct TermMetaPitch {
    expression: String,
    mode: String,
    data: TermMetaPitchData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transcription {
    ipa: String,
    tags: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TermMetaPhoneticData {
    reading: String,
    transcriptions: Vec<Transcription>
}

#[derive(Debug)]
pub struct TermMetaPhonetic {
    expression: String,
    mode: String,
    data: TermMetaPhoneticData
}

//TermMeta can be TermMetaFrequency, TermMetaPitch, or TermMetaPhonetic
//check the mode string to tell
#[derive(Debug)]
pub enum TermMeta {
    Frequency(TermMetaFrequency),
    Pitch(TermMetaPitch),
    Phonetic(TermMetaPhonetic)
}

impl<'de> Deserialize<'de> for TermMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct TermMetaVisitor;

        impl<'de> Visitor<'de> for TermMetaVisitor {
            type Value = TermMeta;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("term meta")
            }

            fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
            where 
                S: SeqAccess<'de>
            {
                const TYPE_FIELDS: &[&str] = &["freq", "pitch", "ipa"];
                let expression: Option<String> = access.next_element()?;
                if expression.is_none() {
                    return Err(<S::Error as Error>::missing_field("expression"));
                }

                let mode: Option<String> = access.next_element()?;
                if mode.is_none() {
                    return Err(<S::Error as Error>::missing_field("type"));
                }
                
                let mode_str = mode.unwrap();
                match mode_str.as_str() {
                    "pitch" =>  {
                        let pitch_data = access.next_element::<TermMetaPitchData>()?;
                        if let Some(data) = pitch_data {
                            return Ok(TermMeta::Pitch(TermMetaPitch {
                                expression: expression.unwrap(),
                                mode: mode_str,
                                data
                            }));
                        }
                        else {
                            return Err(<S::Error as Error>::missing_field("data"));
                        }
                    },
                    "ipa" => {
                        let phonetic_data = access.next_element::<TermMetaPhoneticData>()?;
                        if let Some(data) = phonetic_data {
                            return Ok(TermMeta::Phonetic(TermMetaPhonetic {
                                expression: expression.unwrap(),
                                mode: mode_str,
                                data
                            }));
                        }
                        else {
                            return Err(<S::Error as Error>::missing_field("data"));
                        }
                    },
                    "freq" => {
                        let freq_data = access.next_element::<TermMetaFrequencyData>()?;
                        if let Some(data) = freq_data {
                            return Ok(TermMeta::Frequency(TermMetaFrequency {
                                expression: expression.unwrap(),
                                mode: mode_str,
                                data
                            }));
                        }
                        else {
                            return Err(<S::Error as Error>::missing_field("data"));
                        }
                    },
                    _ => Err(<S::Error as Error>::unknown_variant(mode_str.as_str(), TYPE_FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(TermMetaVisitor)
    }
}

impl Serialize for TermMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut seq = serializer.serialize_seq(None)?;
        match *self {
            TermMeta::Frequency(ref freq) => {
                seq.serialize_element(&freq.expression)?;
                seq.serialize_element(&freq.mode)?;
                seq.serialize_element(&freq.data)?;
            },
            TermMeta::Phonetic(ref phonetic) => {
                seq.serialize_element(&phonetic.expression)?;
                seq.serialize_element(&phonetic.mode)?;
                seq.serialize_element(&phonetic.data)?;
            },
            TermMeta::Pitch(ref pitch) => {
                seq.serialize_element(&pitch.expression)?;
                seq.serialize_element(&pitch.mode)?;
                seq.serialize_element(&pitch.data)?;
            },
        }
        seq.end()
    }
}