use std::path::Path;
use schemas::*;
use rusqlite::{Connection, Error};
use std::result::Result;
//spitting some ideas here

pub struct YomitanDatabase {
    connection: Connection,
}

impl YomitanDatabase {
    //opens an existing database or create a new one
    fn open_database(dict: &Path) -> Result<YomitanDatabase, Error> {
        let connection= Connection::open(dict)?;
        
        //create tables if this is a new database
        create_tables(&connection)?;

        Ok(YomitanDatabase { connection })
    }

    
}

fn create_tables(conn: &Connection) -> Result<(), Error> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS dictionaries(
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                title TEXT NOT NULL,
                version INTEGER,
                revision TEXT NOT NULL,
                import_date DATE,
                prefix_wildcards_supported BOOLEAN,
                sequenced BOOLEAN,
                counts INTEGER DEFAULT 0",
    (),
    ) {
        Err(err) => return Err(err),
        Ok(_) => println!("Created dictionaries table"),
    }

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS kanji(
             id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
             dictionary TEXT NOT NULL,
             kunyomi TEXT NOT NULL,
             meanings TEXT NOT NULL,
             onyomi TEXT NOT NULL,
             stats TEXT NOT NULL,
             tag TEXT NOT NULL",
    (),
    ) {
        Err(err) => return Err(err),
        Ok(_) => println!("Created kanji table"),
    }

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS media(
             key INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
             id INTEGER NOT NULL,
             dictionary TEXT NOT NULL,
             height INTEGER NOT NULL,
             width INTEGER NOT NULL,
             mediatype TEXT NOT NULL,
             path TEXT NOT NULL,
             content BLOB NOT NULL",
    (),
    ) {
        Err(err) => return Err(err),
        Ok(_) => println!("Created media table"),
    }

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS tagMeta(
             id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
             dictionary TEXT NOT NULL,
             category TEXT NOT NULL,
             name TEXT NOT NULL,
             notes TEXT NOT NULL,
             order INTEGER NOT NULL,
             score INTEGER NOT NULL",
    (),
    ) {
        Err(err) => return Err(err),
        Ok(_) => println!("Created tagMeta table"),
    }

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS termMeta(
             id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
             dictionary TEXT NOT NULL,
             data TEXT NOT NULL,
             expression TEXT NOT NULL,
             mode TEXT NOT NULL",
    (),
    ) {
        Err(err) => return Err(err),
        Ok(_) => println!("Created termMeta table"),
    }

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS terms(
             id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
             definitionTags TEXT NOT NULL,
             dictionary TEXT NOT NULL,
             expression TEXT NOT NULL,
             expressionReverse TEXT NOT NULL,
             glossary TEXT NOT NULL,
             id INTEGER NOT NULL,
             reading TEXT NOT NULL,
             readingReverse TEXT NOT NULL,
             rules TEXT NOT NULL,
             score INTEGER NOT NULL,
             sequence INTEGER NOT NULL,
             termTags TEXT NOT NULL",
    (),
    ) {
        Err(err) => return Err(err),
        Ok(_) => println!("Created terms table"),
    }
    Ok(())
}