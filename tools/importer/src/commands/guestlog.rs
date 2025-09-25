use std::io::{Error, ErrorKind};

use async_sqlite::PoolBuilder;

use crate::ternary;

pub async fn guestlog(verbose: bool, api_key: String, base_id: String) -> Result<(), Error> {
    let db_url = std::env::var("DB_URL").unwrap_or_else(|_| String::from("./db.sqlite3"));
    crate::logger::debug("Importing Guestlog from airtable", verbose);
    let client = reqwest::blocking::Client::builder()
        .user_agent(format!(
            "Sean Outram Data Importer {}",
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
        .unwrap()
        .json::<AirtableListReq>()
        .unwrap();
    crate::logger::info(format!(
        "Grabbed {} records",
        airtable_records.records.len()
    ));

    for record in airtable_records.records {
        pool.conn(move |conn| {
            conn.execute(
                "INSERT INTO guestlog (name, email, message, active, gravatar_url, avatar)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) ON CONFLICT DO NOTHING",
                [
                    record.fields.name,
                    record.fields.email,
                    record.fields.message,
                    ternary!(record.fields.active => "1".to_string(), "0".to_string()),
                    record.fields.gravatar_url.unwrap_or_default(),
                    String::new(),
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
