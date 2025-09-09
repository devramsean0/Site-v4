use async_sqlite::Pool;
use rusqlite::{Error, Row};

pub async fn create_tables(pool: &Pool) -> Result<(), async_sqlite::Error> {
    pool.conn(move |conn| {
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        // admin_users (Very bad unsecure code but you deserve my password if you get access to this DB)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS admin_user (
                email TEXT PRIMARY KEY,
                password TEXT NOT NULL,
                createdAt INTEGER  NOT NULL,
                indexedAt INTEGER  NOT NULL
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
