use serde::de::Deserializer;
use serde::ser::{Serializer, SerializeSeq};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TagInformation {
    name: String,
    category: String,
    sorting_order: i32,
    notes: String,
    popularity_score: i32
}

#[derive(Serialize, Deserialize)]
struct TagInfoArray(
    String,
    String,
    i32,
    String,
    i32
);

impl<'de> Deserialize<'de> for TagInformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let TagInfoArray(name, category, sorting_order, notes, popularity_score) =
            TagInfoArray::deserialize(deserializer)?;

        Ok(TagInformation {
            name,
            category,
            sorting_order,
            notes,
            popularity_score
        })
    }
}

impl Serialize for TagInformation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //TODO change to constant
        let mut seq = serializer.serialize_seq(Some(5))?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.category)?;
        seq.serialize_element(&self.sorting_order)?;
        seq.serialize_element(&self.notes)?;
        seq.serialize_element(&self.popularity_score)?;
        seq.end()
    }
}
pub type DictionaryTagBankV3 = Vec<TagInformation>;
