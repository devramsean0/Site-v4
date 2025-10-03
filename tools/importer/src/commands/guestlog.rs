use async_sqlite::PoolBuilder;
use random_str as random;
use std::fs::{self, File, OpenOptions};
use std::io::{Cursor, Error, ErrorKind, Write};

use crate::ternary;

pub async fn guestlog(verbose: bool, api_key: String, base_id: String) -> Result<(), Error> {
    let db_url = std::env::var("DB_URL").unwrap_or_else(|_| String::from("./db.sqlite3"));
    let upload_folder: String =
        std::env::var("UPLOADS_PATH").unwrap_or_else(|_| "./uploads".to_string());
    crate::logger::debug("Importing Guestlog from airtable", verbose);
    let client = reqwest::Client::builder()
        .user_agent(format!(
            "Sean's Site Data Importer {}",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap();

    let pool = match PoolBuilder::new().path(db_url).open().await {
        Ok(pool) => {
            crate::logger::info("Connected to DB".to_string());
            pool
        }
        Err(e) => {
            crate::logger::info(format!("Error estalishing DB pool {e}"));
            return Err(Error::new(
                ErrorKind::Other,
                "database pool could not be established",
            ));
        }
    };

    let airtable_records = client
        .get(format!(
            "https://api.airtable.com/v0/{base_id}/guestlog?view=active"
        ))
        .bearer_auth(api_key)
        .send()
        .await
        .unwrap()
        .json::<AirtableListReq>()
        .await
        .unwrap();
    crate::logger::info(format!(
        "Grabbed {} records",
        airtable_records.records.len()
    ));
    for record in airtable_records.records {
        let mut avatar_path = String::new();
        // Fetch avatar
        if record.fields.gravatar_url.is_some() {
            let avatar_data = client
                .get(record.fields.gravatar_url.unwrap())
                .send()
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();
            let mut filepath = "/".to_string();
            let mut filename = String::new();

            while fs::exists(filepath.clone()).unwrap() {
                let prefix = random::get_string(16, true, false, true, false);
                filename = format!("{prefix}-{filename}");
                filepath = format!("{upload_folder}/{filename}");
            }

            let mut pos = 0;
            let mut file = File::create(&filepath).unwrap();
            while pos < avatar_data.len() {
                let bytes_written = fs::File::write(&mut file, &avatar_data[pos..]).unwrap();
                pos += bytes_written
            }
            avatar_path = filename;
        }
        pool.conn(move |conn| {
            conn.execute(
                "INSERT INTO guestlog (name, email, message, active, avatar)
            VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT DO NOTHING",
                [
                    record.fields.name,
                    record.fields.email,
                    record.fields.message,
                    ternary!(record.fields.active => "1".to_string(), "0".to_string()),
                    avatar_path,
                ],
            )
        })
        .await
        .unwrap();
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct AirtableListReq {
    #[serde(rename = "offset")]
    _offset: Option<String>,
    records: Vec<AirtableRow>,
}
#[derive(serde::Deserialize)]
struct AirtableRow {
    #[serde(rename = "id")]
    _id: String,
    #[serde(rename = "createdTime")]
    _created_time: String,
    fields: GuestlogData,
}
#[derive(serde::Deserialize)]
struct GuestlogData {
    name: String,
    email: String,
    message: String,
    active: bool,
    gravatar_url: Option<String>,
}

pub struct AssetFilenameParts {
    pub filename: String,
    pub path: String,
}
