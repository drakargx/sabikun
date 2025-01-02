mod dictionary_index_schema;
mod iso_languages;
mod kanji_bank_schema;
mod tag_bank_schema;
mod term_bank_schema;

use dictionary_index_schema::DictionaryIndex;
use kanji_bank_schema::DictionaryKanjiBankV3;
use kanji_bank_schema::KanjiInformation;
use tag_bank_schema::DictionaryTagBankV3;
use term_bank_schema::DictionaryTermBankV3;
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
// fn main() -> std::io::Result<()> {
//     let path = "C:\\code\\yomidb\\testzip\\kanji_bank_1.json";
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//     let jd = &mut serde_json::Deserializer::from_reader(reader);
//     let result: Result<DictionaryKanjiBankV3, _> = serde_path_to_error::deserialize(jd);
//     match result {
//         Ok(_) => {
//             //println!("Ok {:?}", result.unwrap())
//             //let serialized = serde_json::to_string(&result.unwrap()).unwrap();
//             //println!("{}", serialized);
//             println!("Ok");
//         }, 
//         Err(err) => {
//             let path = err.path().to_string();
//             println!("Fail {:#?}", path);
//         }
//     }
    
//     Ok(())
// }

//tag bank
// fn main() -> std::io::Result<()> {
//     let path = "C:\\code\\yomidb\\testzip\\tag_bank_1.json";
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//     let jd = &mut serde_json::Deserializer::from_reader(reader);
//     let result: Result<DictionaryTagBankV3, _> = serde_path_to_error::deserialize(jd);
//     match result {
//         Ok(_) => {
//             //println!("Ok {:?}", result.unwrap())
//             let serialized = serde_json::to_string(&result.unwrap()).unwrap();
//             println!("{}", serialized);
//             //println!("Ok");
//         }, 
//         Err(err) => {
//             let path = err.path().to_string();
//             println!("Fail {:#?}", path);
//         }
//     }
    
//     Ok(())
// }

//term bank..
fn main() -> std::io::Result<()> {
    let path = "C:\\code\\yomidb\\testzip\\single_term_bank_1.json";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let jd = &mut serde_json::Deserializer::from_reader(reader);
    let result: Result<DictionaryTermBankV3, _> = serde_path_to_error::deserialize(jd);
    match result {
        Ok(_) => {
            //println!("Ok {:?}", result.unwrap())
            let serialized = serde_json::to_string(&result.unwrap()).unwrap();
            println!("{}", serialized);
            //println!("Ok");
        }, 
        Err(err) => {
            let path = err.path().to_string();
            println!("Fail {:#?}", path);
        }
    }
    
    Ok(())
}