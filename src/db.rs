use async_sqlite::Pool;
use rand::Rng;
use rusqlite::{Error, Row};

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
                created_at INTEGER  NOT NULL,
                updated_at INTEGER  NOT NULL
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
}
