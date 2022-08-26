use sqlite::{Connection, State};

pub struct User {
    pub id: i64,
    pub username: String,
    pub access_key: String,
}

pub fn fetch_user_key(conn: &Connection, access_key: String) -> Option<User> {
    let mut user_db = conn.prepare("SELECT id, username, access_key FROM users WHERE access_key = ? LIMIT 1")
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
