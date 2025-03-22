use yomi_dict_db::YomitanDatabase;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use schemas::DictionaryTermBankV3;

fn main() -> std::io::Result<()> {
    let db = YomitanDatabase::open_database("test.db").unwrap();

    let path = "C:\\code\\yomidb\\testzip\\term_bank_1.json";
    //let path = "C:\\code\\yomidb\\testzip\\term_bank_1.json";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let jd = &mut serde_json::Deserializer::from_reader(reader);
    let result: Result<DictionaryTermBankV3, _> = serde_path_to_error::deserialize(jd);

    match result {
        Ok(_) => {
            let term_bank = result.unwrap();

            for term in term_bank {
                db.insert_term(term, "TestDict").unwrap();
            }
            
        }
        Err(err) => {
            let path = err.path().to_string();
            println!("Fail {:#?}", path);
        }
    }
    Ok(())
}
