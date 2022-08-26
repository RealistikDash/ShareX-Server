use sqlite::{self, Connection};

fn create_base_tables(conn: &sqlite::Connection) -> sqlite::Result<()> {
    // Users Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            access_key TEXT NOT NULL
        );",
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS uploads (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
        );",
    )?;

    Ok(())
}

pub fn create_db(file: String) -> sqlite::Result<Connection> {
    let db = sqlite::open(file)?;
    create_base_tables(&db)?;
    Ok(db)
}
