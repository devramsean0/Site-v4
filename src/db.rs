use async_sqlite::Pool;
use chrono::NaiveDate;
use rand::Rng;
use rusqlite::{Error, Row};

use crate::routes::admin::experience::AdminExperienceSelectProps;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const STR_LEN: usize = 15;

pub async fn create_tables(pool: &Pool) -> Result<(), async_sqlite::Error> {
    pool.conn(move |conn| {
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        // admin_users (Very bad unsecure code but you deserve my password if you get access to this DB)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS admin_user (
                email TEXT PRIMARY KEY,
                password TEXT NOT NULL,
                createdAt INTEGER  NOT NULL,
                updatedAt INTEGER  NOT NULL
            )",
            [],
        )
        .unwrap();
        // admin_session
        conn.execute(
            "CREATE TABLE IF NOT EXISTS admin_session (
                session TEXT PRIMARY KEY,
                created_at TEXT  NOT NULL,
                updated_at TEXT  NOT NULL
            )",
            [],
        )
        .unwrap();
        // experience
        conn.execute(
            "CREATE TABLE IF NOT EXISTS experience (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            company TEXT NOT NULL,
            description TEXT NOT NULL,
            start_date TEXT NOT NULL,
            end_date TEXT,
            type TEXT NOT NULL
        )",
            [],
        )
        .unwrap();
        // project
        conn.execute(
            "CREATE TABLE IF NOT EXISTS project (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            src TEXT,
            docs TEXT,
            demo TEXT,
            preview_img TEXT,
            favourite INTEGER NOT NULL,
            technologies json NOT NULL
        )",
            [],
        )
        .unwrap();
        Ok(())
    })
    .await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct AdminUser {
    pub email: String,
    pub password: String,
}

impl AdminUser {
    fn map_from_row(row: &Row) -> Result<Self, Error> {
        let email: String = row.get(0)?;
        let password: String = row.get(1)?;
        Ok(Self { email, password })
    }
    pub async fn find_by_email(
        email: String,
        pool: &Pool,
    ) -> Result<Option<Self>, async_sqlite::Error> {
        pool.conn(move |conn| {
            let mut stmt = conn.prepare("SELECT * FROM admin_user WHERE email = ?1")?;
            let user = match stmt.query_one([email], |row| Self::map_from_row(row)) {
                Ok(user) => Some(user),
                _ => None,
            };
            Ok(user)
        })
        .await
    }
}

#[derive(Debug, Clone)]
pub struct AdminSession {
    pub session: String,
    created_at: String,
    updated_at: String,
}

impl AdminSession {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let rand_str: String = (0..STR_LEN)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        let now = chrono::Local::now().timestamp().to_string();
        Self {
            session: rand_str,
            created_at: now.clone(),
            updated_at: now,
        }
    }
    fn map_from_row(row: &Row) -> Result<Self, Error> {
        Ok(Self {
            session: row.get(0)?,
            created_at: row.get(1)?,
            updated_at: row.get(2)?,
        })
    }
    pub async fn insert(self, pool: &Pool) -> Result<(), async_sqlite::Error> {
        pool.conn(move |conn| {
            let mut stmt = conn.prepare(
                "INSERT INTO admin_session (session, created_at, updated_at) VALUES (?1, ?2, ?3);",
            ).unwrap();

            stmt.execute([self.session, self.created_at, self.updated_at])?;
            Ok(())
        })
        .await?;
        Ok(())
    }
    pub async fn verify(pool: &Pool, cookie_session: String) -> Result<bool, async_sqlite::Error> {
        pool.conn(move |conn| {
            let mut stmt = conn.prepare("SELECT * FROM admin_session WHERE session = ?1")?;
            let session =
                match stmt.query_one([cookie_session.clone()], |row| Self::map_from_row(row)) {
                    Ok(session) => session,
                    Err(e) => {
                        log::error!("SQL error: {e}");
                        AdminSession {
                            session: String::new(),
                            created_at: String::new(),
                            updated_at: String::new(),
                        }
                    }
                };
            log::debug!(
                "DB Session ID: {} (cookie: {cookie_session})",
                session.session
            );
            return Ok(session.session == cookie_session);
        })
        .await
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Experience {
    pub id: Option<i64>,
    pub name: String,
    pub company: String,
    pub description: String,
    pub start_date: String,
    pub end_date: String,
    pub e_type: String,
}
impl Experience {
    fn map_from_row(row: &Row, convert_dates: bool) -> Result<Self, Error> {
        let start_date: String = row.get(4)?;
        let end_date: String = row.get(5)?;

        let mut start_date_formatted = start_date.clone();
        let mut end_date_formatted = end_date.clone();
        if convert_dates {
            // Format the dates
            start_date_formatted = Self::format_date(&start_date);
            end_date_formatted = if end_date.is_empty() {
                String::new()
            } else {
                Self::format_date(&end_date)
            };
        }

        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            company: row.get(2)?,
            description: row.get(3)?,
            start_date: start_date_formatted.clone().to_owned(),
            end_date: end_date_formatted.clone().to_owned(),
            e_type: row.get(6)?,
        })
    }
    fn format_date(date_str: &str) -> String {
        match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => date.format("%B %Y").to_string(),
            Err(_) => date_str.to_string(),
        }
    }
    pub async fn insert(pool: &Pool, data: Experience) -> Result<(), async_sqlite::Error> {
        pool.conn(|conn| {
            let mut stmt = conn.prepare(
                "INSERT INTO experience (name, company, description, start_date, end_date, type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
            ).unwrap();
            stmt.execute([data.name, data.company, data.description, data.start_date, data.end_date, data.e_type])?;
            Ok(())
        })
        .await?;
        Ok(())
    }
    pub async fn all(pool: &Pool, convert_date: bool) -> Result<Vec<Self>, async_sqlite::Error> {
        pool.conn(move |conn| {
            let mut stmt = conn.prepare("SELECT * FROM experience")?;
            let status_iter = stmt
                .query_map([], |row| Ok(Self::map_from_row(row, convert_date).unwrap()))
                .unwrap();

            let mut statuses = Vec::new();
            for status in status_iter {
                statuses.push(status?);
            }
            Ok(statuses)
        })
        .await
    }
    pub async fn get_by_id(id: String, pool: &Pool) -> Result<Option<Self>, async_sqlite::Error> {
        pool.conn(move |conn| {
            let mut stmt = conn.prepare("SELECT * FROM experience WHERE id = ?1")?;
            let user = match stmt.query_one([id], |row| Self::map_from_row(row, false)) {
                Ok(user) => Some(user),
                _ => None,
            };
            Ok(user)
        })
        .await
    }
    pub async fn delete(
        pool: &Pool,
        data: AdminExperienceSelectProps,
    ) -> Result<(), async_sqlite::Error> {
        pool.conn(move |conn| {
            let mut stmt = conn
                .prepare("DELETE FROM experience WHERE id = ?1")
                .unwrap();
            stmt.execute([data.id.to_owned()])?;
            Ok(())
        })
        .await?;
        Ok(())
    }
    pub async fn update(pool: &Pool, data: Experience) -> Result<(), async_sqlite::Error> {
        pool.conn(move |conn| {
            let mut stmt = conn.prepare(
                "UPDATE experience SET name = ?1, company = ?2, description = ?3, start_date = ?4, end_date = ?5, type = ?6 WHERE id = ?7"
            ).unwrap();
            stmt.execute([data.name, data.company, data.description, data.start_date, data.end_date, data.e_type, data.id.unwrap().to_string()])?;
            Ok(())
        })
        .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Project {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub src: Option<String>,
    pub docs: Option<String>,
    pub demo: Option<String>,
    pub preview_img: Option<String>,
    pub favourite: bool,
    pub technologies: Vec<String>,
}

impl Project {
    fn map_from_row(row: &Row) -> Result<Self, Error> {
        let technologies_json: String = row.get(8)?;

        // Parse the JSON string back into a Vec<String>
        let technologies: Vec<String> =
            serde_json::from_str(&technologies_json).unwrap_or_else(|_| Vec::new());
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            src: row.get(3)?,
            docs: row.get(4)?,
            demo: row.get(5)?,
            preview_img: row.get(6)?,
            favourite: row.get(7)?,
            technologies: technologies,
        })
    }
    pub async fn all(pool: &Pool) -> Result<Vec<Self>, async_sqlite::Error> {
        pool.conn(move |conn| {
            let mut stmt = conn.prepare("SELECT * FROM project")?;
            let status_iter = stmt
                .query_map([], |row| Ok(Self::map_from_row(row).unwrap()))
                .unwrap();

            let mut statuses = Vec::new();
            for status in status_iter {
                statuses.push(status?);
            }
            Ok(statuses)
        })
        .await
    }
    pub async fn insert(pool: &Pool, data: Project) -> Result<(), async_sqlite::Error> {
        let mut technolgies_string = "json('[".to_string();
        for tech in data.technologies {
            technolgies_string.push_str(format!("\"{tech}\",").as_str());
        }
        technolgies_string.push_str("]')");
        pool.conn(move |conn| {
            let mut stmt = conn.prepare("INSERT INTO project (name, description, src, docs, demo, preview_img, favourite, technologies) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)").unwrap();
            stmt.execute([data.name, data.description, data.src.unwrap(), data.docs.unwrap(), data.demo.unwrap(), data.preview_img.unwrap(), data.favourite.to_string(), technolgies_string])?;
            Ok(())
        })
        .await?;
        Ok(())
    }
    pub fn get_src(&self) -> &str {
        self.src.as_deref().unwrap()
    }
    pub fn get_docs(&self) -> &str {
        self.docs.as_deref().unwrap()
    }
    pub fn get_demo(&self) -> &str {
        self.demo.as_deref().unwrap()
    }
    pub fn get_preview_img(&self) -> &str {
        self.preview_img.as_deref().unwrap()
    }
}
