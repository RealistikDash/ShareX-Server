use crate::sharex::utils;
use sqlite::{Connection, State};

pub struct User {
    pub id: i64,
    pub username: String,
    pub access_key: String,
}

pub fn fetch_user_key(conn: &Connection, access_key: String) -> Option<User> {
    let mut user_db = conn
        .prepare("SELECT id, username, access_key FROM users WHERE access_key = ? LIMIT 1")
        .unwrap()
        .bind(1, access_key.as_str())
        .unwrap();

    if let State::Row = user_db.next().unwrap() {
        Some(User {
            id: user_db.read(0).unwrap(),
            username: user_db.read(1).unwrap(),
            access_key: user_db.read(2).unwrap(),
        })
    } else {
        None
    }
}

const ACCESS_KEY_LEN: usize = 16;

pub fn register_user(username: String, conn: &Connection) -> User {
    let access_key = utils::random_string(ACCESS_KEY_LEN);
    conn.prepare("INSERT INTO users (username, access_key) VALUES (?, ?)")
        .unwrap()
        .bind(1, username.as_str())
        .unwrap()
        .bind(2, access_key.as_str())
        .unwrap()
        .next()
        .unwrap();

    // TODO: Is there seriously no lastrowid?
    let mut last_row_stmt = conn.prepare("SELECT last_insert_rowid()").unwrap();

    last_row_stmt.next().unwrap();

    User {
        id: last_row_stmt.read(0).unwrap(),
        username: username,
        access_key: access_key,
    }
}
