use rusqlite::{Connection, MappedRows};
use std::{collections::HashMap, path::Path};

use super::{TagDetails, TagID};

pub mod prelude {}
struct Database<'a> {
    conn: Connection,
    taggraph: TagGraph<'a>,
}

/// Stores parent-child relationships between all tags
struct TagGraph<'a> {
    graph: HashMap<TagID, TagNode<'a>>,
}
/// Nodes in a [TagGraph]
struct TagNode<'a> {
    parents: Vec<&'a TagNode<'a>>,
    tag: Tag,
}

struct Image {
    id: i32,
}

struct Tag {
    id: i32,
    name: String,
}

impl Database<'_> {
    fn open<P: AsRef<Path>>(path: P) -> Self {
        let connection = Connection::open(path).unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS image (
                id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE)",
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
                image_id REFERENCES image (id),
                tag_id REFERENCES tag (id), UNIQUE (tag_id, image_id) ON CONFLICT IGNORE)",
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

        Self {
            conn: connection,
            taggraph: TagGraph {
                graph: HashMap::new(),
            },
        }
    }

    ///Retrieve rows from child_to_parent table, construct a [TagGraph]
    fn create_tag_tree() {
        //make tag graph
        //SELECT * FROM child_to_parent
        //For each row in child to parent
        //if there is no TagNode.tag matching in the graph already, then create it, and add the parent
        //else add the parent
    }

    fn new_tag<S: AsRef<str>>(&self, name: S) -> Option<i64> {
        let mut stmt = self
            .conn
            .prepare(
                "INSERT OR IGNORE INTO tags (id, name) 
        VALUES (NULL, ?)
        ",
            )
            .unwrap();

        stmt.execute(&[name.as_ref()]).unwrap();

        Some(self.conn.last_insert_rowid()) //UI needs this
    }

    fn get_tags(&self) -> Vec<TagDetails> {
        let mut query = self.conn.prepare("SELECT * from tag").unwrap();
        let tag_iter = query
            .query_map([], |row| {
                Ok(TagDetails {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    parents: vec![],
                })
            })
            .unwrap();

        Vec::from_iter(tag_iter.map(|tag| tag.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}
}
