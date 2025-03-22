use std::path::Path;
use schemas::*;
use rusqlite::{params, types::ToSqlOutput, Connection, Error, ToSql};
use std::result::Result;

pub struct YomitanDatabase {
    connection: Connection,
}

struct FormatWrapper(Format);

impl ToSql for FormatWrapper {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self.0 {
            Format::V1 => Ok(ToSqlOutput::from(1)),
            Format::V2 => Ok(ToSqlOutput::from(2)),
            Format::V3 => Ok(ToSqlOutput::from(3)),
        }
    }
}

impl YomitanDatabase {
    //opens an existing database or create a new one
    pub fn open_database<P: AsRef<Path>>(dict: P) -> Result<YomitanDatabase, Error> {
        let connection= Connection::open(dict)?;
        
        //create tables if this is a new database
        create_tables(&connection)?;

        Ok(YomitanDatabase { connection })
    }

    //gui opens zip file and sends the files individually over
    //this is done because otherwise im not sure how else to display progress xdxd
    //ui checks for if index.json is present and if the banks are numbered appropriately
    pub fn insert_index(&self, index: &DictionaryIndex, prefix_wildcards_support: bool) -> Result<(), Error> {
        let mut format: Option<FormatWrapper> = None;
        if let Some(version) = &index.format {
            format = Some(FormatWrapper(version.clone()));
        }

        match self.connection.execute(
            "INSERT INTO dictionaries (title, version, revision, import_date, prefix_wildcards_supported, sequenced, counts) \
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (
                    &index.title,
                    format,
                    &index.revision,
                    chrono::offset::Local::now(),
                    prefix_wildcards_support,
                    &index.sequenced,
                    0
                )
        ) {
            Err(err) => return Err(err),
            Ok(_) => println!("Inserted index.json"),
        }

        Ok(())
    }

    pub fn insert_term(&self, term: TermInformation, dictionary: &str) -> Result<(), Error> {
        const INSERT_QUERY: &str = "INSERT INTO terms (definitionTags, dictionary, expression, expressionReverse, glossary, reading, readingReverse, rules, score, sequence, termTags) \
                                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)";
            

        
            self.connection.execute(INSERT_QUERY, params![term.definition_tags,
                                                            dictionary,
                                                            term.term,
                                                            term.term.chars().rev().collect::<String>(),
                                                            term.definitions_as_json(),
                                                            term.reading,
                                                            term.reading.chars().rev().collect::<String>(),
                                                            term.deinflectors,
                                                            term.popularity,
                                                            term.sequence_number,
                                                            term.term_tags])?;
        Ok(())
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
                counts INTEGER DEFAULT 0)",
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
             tag TEXT NOT NULL)",
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
             content BLOB NOT NULL)",
    (),
    ) {
        Err(err) => return Err(err),
        Ok(_) => println!("Created media table"),
    }

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS tagMeta( \
             id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
             dictionary TEXT NOT NULL, \
             category TEXT NOT NULL, \
             name TEXT NOT NULL, \
             notes TEXT NOT NULL, \
             sortOrder INTEGER NOT NULL, \
             score INTEGER NOT NULL)",
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
             mode TEXT NOT NULL)",
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
             reading TEXT NOT NULL,
             readingReverse TEXT NOT NULL,
             rules TEXT NOT NULL,
             score INTEGER NOT NULL,
             sequence INTEGER NOT NULL,
             termTags TEXT NOT NULL)",
    (),
    ) {
        Err(err) => return Err(err),
        Ok(_) => println!("Created terms table"),
    }
    Ok(())
}
