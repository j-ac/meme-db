use super::*;
use rusqlite::{Connection, MappedRows};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, path::Path};

pub struct Database<'a> {
    conn: Arc<Mutex<Connection>>,
    taggraph: TagGraph<'a>,
}

pub struct DatabaseMap<'a> {
    pub map: HashMap<DatabaseID, Database<'a>>,
    largest_id: usize,
}

impl DatabaseMap<'_> {
    pub fn get(&self, id: DatabaseID) -> Option<&Database> {
        self.map.get(&id)
    }
}

/// Stores parent-child relationships between all tags
struct TagGraph<'a> {
    graph: HashMap<TagID, TagNode<'a>>,
}

/// Nodes in a [TagGraph]
struct TagNode<'a> {
    parents: Vec<&'a TagNode<'a>>,
    id: TagID,
    name: String,
}

impl TagGraph<'_> {
    //given a TagID return all ancestors
    pub fn get_ancestor_ids(&self, id: TagID) -> Vec<TagID> {
        let mut child = self.graph.get(&id);
        let mut nodes: Vec<&'_ TagNode<'_>> = Vec::new();
        nodes.extend_from_slice(&child.unwrap().parents); //Initialize the parent array with the child's immidiate parents

        let mut ret = HashSet::new();

        //Did not use an iterator here since it can't work on a vector expanding on each iteration
        //let mut i = 0;
        //while i < nodes.len() {
        for i in 0..nodes.len() {
            if ret.insert(nodes[i].id) {
                nodes.extend_from_slice(&nodes[i].parents);
            }
            //i += 1;
        }

        ret.into_iter().collect::<Vec<_>>()

        /*
        //Expand the array which each parents' parents, until its all been exausted
        let i = 0;
        while i < nodes.len() {
            nodes.append(&mut nodes[i].parents);
        }

        for node in nodes {
            nodes.append(&mut node.parents);
        }

        //Extract just the TagIDs from the previous vector
        let ret: Vec<TagID>;
        for node in nodes {
            ret.push(node.id);
        };

        ret */
    }
}

struct Image {
    id: i32,
    path: String,
}

impl<'a> Database<'a> {
    //TODO: Replace with Connection::execute_batch()
    fn open<P: AsRef<Path>>(path: P, mut dbmap: DatabaseMap<'a>) -> () {
        let connection = Connection::open(path).unwrap();

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS image (
                id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE)
                path TEXT UNIQUE",
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

        let ret = Self {
            conn: Arc::new(Mutex::new(connection)),
            taggraph: TagGraph {
                graph: HashMap::new(),
            },
        };

        dbmap.map.insert(dbmap.largest_id + 1, ret);
    }

    //Invoked by mdbapi::add_file_tag()
    pub fn get_details_on_file(&self, id: FileID) -> GUIResult<FileDetails> {
        let path: PathBuf = self
            .conn
            .lock()
            .expect("The mutex was poisoned")
            .query_row("SELECT path FROM image WHERE id = ?", [id], |row| {
                row.get(0).map(|x: String| x.into())
            })
            .map_err(|e| Error {
                gui_msg: "invalid file ID".to_string(),
                err_type: ErrorType::Arguments,
            })?;

        let folder = path.parent();

        //Convert an ostr to String, and do not include the path.
        let name: String = path
            .file_name()
            .map(|x| x.to_str())
            .flatten()
            .map(|x| x.to_string())
            .ok_or(Error {gui_msg: "Encountered malformed path entry in DB".to_string(), err_type: ErrorType::Logical})?;

        let tags = self.taggraph.get_ancestor_ids(id);
        Ok(FileDetails {
            id: id,
            name: name,
            folder: 0,
            tags: tags,
        }) //TODO! remove the hardcoded 0 for the folder parameter.
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
        let mtx = self.conn.lock().expect("Mutex is poisoned");
        let mut stmt = mtx
            .prepare(
                "INSERT OR IGNORE INTO tags (id, name) 
        VALUES (NULL, ?)
        ",
            )
            .unwrap();

        stmt.execute(&[name.as_ref()]).unwrap();

        Some(
            self.conn
                .lock()
                .expect("Mutex is poisoned")
                .last_insert_rowid(),
        ) //UI needs this
    }

    fn get_tags(&self) -> Vec<TagDetails> {
        let mtx = self.conn.lock().expect("Mutex is poisoned");
        let mut query = mtx
            .prepare("SELECT * from tag")
            .unwrap();
        let tag_iter = query
            .query_map([], |row| {
                Ok(TagDetails {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    parents: vec![], //TODO: implement this
                })
            })
            .unwrap();

        Vec::from_iter(tag_iter.map(|tag| tag.unwrap()))
    }

    //Invoked by mdbapi::add_file_tag()
    pub fn insert_into_tag_records(&self, file: FileID, tag: TagID) {
        //TODO, make this actually use the DatabaseID to select the appropriate one
        self.conn
            .lock()
            .expect("Mutex is poisoned")
            .execute(
                "INSERT INTO tag_records
        (image_id, tag_id) VALUES (?1, ?2)",
                [file, tag],
            )
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}
}
