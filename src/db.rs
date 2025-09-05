use async_sqlite::Pool;

pub async fn create_tables(pool: &Pool) -> Result<(), async_sqlite::Error> {
    pool.conn(move |conn| {
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        Ok(())
    })
    .await?;
    Ok(())
}
