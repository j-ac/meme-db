use std::path::Path;

use rusqlite::Connection;

pub mod prelude {}
struct Database {
    conn: Connection,
}

impl Database {
    fn open<P: AsRef<Path>>(path: P) -> Self {
        let connection = Connection::open(path).unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS image (
                id INTEGER PRIMARY KEY UNIQUE AUTOINCREMENT)",
                [],
            )
            .unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS tag (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE ,
                name TEXT UNIQUE)",
                [],
            )
            .unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS tag_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
                image_id REFERENCES image (id),
                tag_id REFERENCES tag (id))",
                [],
            )
            .unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS child_to_parent
                parent REFERENCES image (id)
                child REFERENCES image (id)",
                [],
            )
            .unwrap();

        Self { conn: connection }
    }

    fn new_tag<S: AsRef<str>>(&self, name: S) -> Option<i64>{
        let mut stmt = self.conn.prepare(
        "INSERT OR IGNORE INTO tags (id, name) 
        VALUES (NULL, ?)
        "
        ).unwrap();

        stmt.execute(&[name.as_ref()]).unwrap();

        Some(self.conn.last_insert_rowid()) //UI needs this

    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_one(){}


}