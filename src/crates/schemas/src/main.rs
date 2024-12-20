mod dictionary_index_schema;
mod iso_languages;
mod kanji_bank_schema;

use dictionary_index_schema::DictionaryIndex;
use kanji_bank_schema::DictionaryKanjiBankV3;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

//dictionary index
// fn main() -> std::io::Result<()> {
//     let path = "C:\\code\\yomidb\\testzip\\index.json";
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//     let jd = &mut serde_json::Deserializer::from_reader(reader);
//     let result: Result<DictionaryIndex, _> = serde_path_to_error::deserialize(jd);
//     match result {
//         Ok(_) => println!("{:?}", result.unwrap()),
//         Err(err) => {
//             let path = err.path().to_string();
//             println!("{:#?}", path);
//         }
//     }
//     Ok(())
// }

//kanji bank
fn main() -> std::io::Result<()> {
    let path = "C:\\code\\yomidb\\testzip\\single_kanji_bank_1.json";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let jd = &mut serde_json::Deserializer::from_reader(reader);
    let result: Result<DictionaryKanjiBankV3, _> = serde_path_to_error::deserialize(jd);
    match result {
        Ok(_) => println!("Ok {:?}", result.unwrap().len()),
        Err(err) => {
            let path = err.path().to_string();
            println!("{:#?}", path);
        }
    }
    Ok(())
}
