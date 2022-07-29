use super::*;
use rusqlite::{Connection, MappedRows};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, path::Path};

pub struct Database {
    conn: Arc<Mutex<Connection>>,
    taggraph: TagGraph,
}

pub struct DatabaseMap {
    pub map: HashMap<DatabaseID, Database>,
    largest_id: usize,
}

impl DatabaseMap {
    pub fn get(&self, id: DatabaseID) -> Option<&Database> {
        self.map.get(&id)
    }
}

/// Stores parent-child relationships between all tags
struct TagGraph {
    graph: HashMap<TagID, TagNode>,
}

impl TagGraph {
    //TagGraph with no data
    fn new() -> TagGraph {
        TagGraph {
            graph: HashMap::new(),
        }
    }

    // Make a new TagGraph and fill its HashMap with the data from an SQL database
    fn new_populated(conn: Connection) -> TagGraph {
        let mut graph = TagGraph::new();
        let mut stmt = conn.prepare("SELECT * FROM child_to_parent").unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let child_id: TagID = row.get_unwrap(1);
            let parent_id: TagID = row.get_unwrap(0);

            // If the child node discovered is not yet in the DB, place it
            if !graph.graph.contains_key(&child_id) {
                graph
                    .graph
                    .insert(child_id, TagNode::new_isolated_node(child_id, &conn));
            }

            // Same as above for the parent node
            if !graph.graph.contains_key(&parent_id) {
                graph
                    .graph
                    .insert(parent_id, TagNode::new_isolated_node(parent_id, &conn));
            }

            /*
            let child = graph.graph.get_mut(&childID).unwrap();
            let parentID: TagID = row.get_unwrap(0);
            let parent = graph.graph.get(&parentID).unwrap();

            child.parents.push(parent);
            */

            // Make an edge between child and parent in the graph
            graph
                .graph
                .get_mut(&child_id)
                .unwrap()
                .parents
                .push(parent_id); //Why no compiler error?? parents wants a TagID but I supplied a TagNode.
        }
        graph
    }
}

/// Nodes in a [TagGraph]
struct TagNode {
    parents: Vec<TagID>,
    id: TagID,
    name: String,
}

impl TagNode {
    //Queries the database for the name associated with an ID and makes a node with NO parents listed
    fn new_isolated_node(id: TagID, conn: &Connection) -> Self {
        let tag_name: String = conn
            .query_row("SELECT name FROM tag WHERE id = ?", [id], |name| {
                name.get(0)
            })
            .unwrap();
        TagNode {
            parents: vec![],
            id,
            name: tag_name,
        }
    }
}

impl TagGraph {
    //given a TagID return all ancestors
    pub fn get_ancestor_ids(&self, id: TagID) -> Vec<TagID> {
        let mut child = self.graph.get(&id);
        let mut nodes: Vec<TagID> = Vec::new();
        nodes.extend_from_slice(&child.unwrap().parents); //Initialize the parent array with the child's immidiate parents

        let mut ret = HashSet::new();

        //Did not use an iterator here since it can't work on a vector expanding on each iteration
        //let mut i = 0;
        //while i < nodes.len() {
        for i in 0..nodes.len() {
            if ret.insert(nodes[i]) {
                nodes.extend_from_slice(&[nodes[i]]);
            }
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

impl Database {
    fn open<P: AsRef<Path>>(path: P) -> Self {
        let connection = Connection::open(path).unwrap();
        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS image (
                id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE)
                path TEXT UNIQUE;
                
            CREATE TABLE IF NOT EXISTS tag (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE ,
                name TEXT UNIQUE);

            CREATE TABLE IF NOT EXISTS tag_records (
                image_id REFERENCES image (id),
                tag_id REFERENCES tag (id), UNIQUE (tag_id, image_id) ON CONFLICT IGNORE);
            
            CREATE TABLE IF NOT EXISTS child_to_parent
                parent REFERENCES image (id)
                child REFERENCES image (id);
                ",
        );

        let ret = Self {
            conn: Arc::new(Mutex::new(connection)),
            taggraph: TagGraph {
                graph: HashMap::new(),
            },
        };

        //dbmap.map.insert(dbmap.largest_id + 1, ret);  //Probably better to do this in another function to avoid self-referentials

        ret
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
            .ok_or(Error {
                gui_msg: "Encountered malformed path entry in DB".to_string(),
                err_type: ErrorType::Logical,
            })?;

        let tags = self.taggraph.get_ancestor_ids(id);
        Ok(FileDetails {
            id: id,
            name: name,
            folder: 0,
            tags: tags,
        }) //TODO! remove the hardcoded 0 for the folder parameter.
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
        let mut query = mtx.prepare("SELECT * from tag").unwrap();
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

    pub fn delete_from_tag_records(&self, file: FileID, tag: TagID) {
        self.conn.lock().expect("Mutex is poisoned").execute(
            "DELETE from tag_records WHERE image_id = ? AND tag_id = ?",
            [file, tag],
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {}
}
