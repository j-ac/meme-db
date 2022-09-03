use super::*;
use rusqlite::{Connection, MappedRows};
use serde::Serializer;
use serde_json::{Result, Value};
use std::collections::binary_heap::Iter;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, path::Path};

pub struct Database {
    pub conn: Arc<Mutex<Connection>>,
    pub taggraph: TagGraph,
    pub folder_map: FolderMap,
}

#[derive(Debug, Serialize)]
pub struct FolderMap {
    pub map: HashMap<FileID, String>,
}

impl FolderMap {
    pub fn new() -> Self {
        FolderMap {
            map: HashMap::new(),
        }
    }

    pub fn new_populated(conn: &Connection) -> Self {
        let mut ret = FolderMap::new();

        let mut stmt = conn.prepare("SELECT (id, path) FROM folder").unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let id: usize = row.get(0).unwrap();
            let path: String = row.get(1).unwrap();

            ret.map.insert(id, path);
        }

        ret
    }

    // Returns the path of the folder that will match all files inside it if used in an sql LIKE clause
    // eg. input of C://dogs/shibas -> C://dogs/shibas/% so that any file under this directory will be captured if put into a LIKE clause
    pub fn get_folder_for_sql_like_clause(&self, id: FileID) -> String {
        let mut path = self.map.get(&id).unwrap().clone();
        path.push_str("/%");
        path
    }
}

pub struct DatabaseMap {
    pub map: HashMap<DatabaseID, Database>,
    pub largest_id: usize,
}

impl DatabaseMap {
    pub fn new() -> Self {
        DatabaseMap {
            map: HashMap::new(),
            largest_id: 0,
        }
    }

    /// Given a [`HashMap<DatabaseID, PathBuf>`] relating IDs to their position on disk,
    /// constructs a DatabaseMap
    pub fn new_populated(map: HashMap<DatabaseID, PathBuf>) -> Self {
        let mut dbmap: DatabaseMap = DatabaseMap::new();
        let mut largest_id = 0;
        for (key, val) in map.clone().iter() {
            // Construct a Database's components
            let conn: Arc<Mutex<Connection>> =
                Arc::new(Mutex::new(rusqlite::Connection::open(val).unwrap()));
            let mtx = conn.lock().expect("Mutex is poisoned");
            let taggraph = TagGraph::new_populated(&mtx);
            let folder_map = FolderMap::new_populated(&mtx);
            drop(mtx);
            dbmap.map.insert(
                *key,
                Database {
                    conn,
                    taggraph,
                    folder_map,
                },
            );
        }

        // Store the greatest ID number in the map
        dbmap.largest_id = map.into_iter().fold(
            0,
            |accum, (left, _)| {
                if accum > left {
                    accum
                } else {
                    left
                }
            },
        );
        dbmap
    }

    pub fn get(&self, id: DatabaseID) -> Option<&Database> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: DatabaseID) -> Option<&mut Database> {
        self.map.get_mut(&id)
    }
}

/// Stores parent-child relationships between all tags
pub struct TagGraph {
    pub graph: HashMap<TagID, TagNode>,
}

impl TagGraph {
    //TagGraph with no data
    pub fn new() -> TagGraph {
        TagGraph {
            graph: HashMap::new(),
        }
    }

    // Make a new TagGraph and fill its HashMap with the data from an SQL database
    pub fn new_populated(conn: &Connection) -> TagGraph {
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

    pub fn insert(&mut self, tag: TagDetails) {
        self.graph.insert(tag.id, tag.into());
    }

    //given a TagID return all ancestors. (Parents, Grandparents ...)
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

    // Return a list of parent IDs,
    pub fn get_parent_ids(&self, id: TagID) -> Vec<TagID> {
        self.graph.get(&id).unwrap().parents.clone()
    }
}

/// Nodes in a [TagGraph]
// Should this even be distinguished from a TagDetails? they've become the same thing over time
pub struct TagNode {
    id: TagID,
    pub parents: Vec<TagID>,
    pub name: String,
    pub colour: usize,
}

impl From<TagDetails> for TagNode {
    fn from(tag: TagDetails) -> Self {
        TagNode {
            id: (tag.id),
            parents: (tag.parents),
            name: (tag.name),
            colour: (tag.colour),
        }
    }
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
            colour: 0,
        }
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
            "CREATE TABLE IF NOT EXISTS folder (
                id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
                path STRING UNIQUE NOT NULL)

            CREATE TABLE IF NOT EXISTS image (
                id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
                folder REFERENCES folder (id),
                name STRING NOT NULL);
                
            CREATE TABLE IF NOT EXISTS tag (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
                name TEXT UNIQUE,
                colour STRING);

            CREATE TABLE IF NOT EXISTS tag_records (
                image_id REFERENCES image (id),
                tag_id REFERENCES tag (id), UNIQUE (tag_id, image_id) ON CONFLICT IGNORE);
            
            CREATE TABLE IF NOT EXISTS child_to_parent
                parent REFERENCES image (id),
                child REFERENCES image (id);",
        );

        Self {
            taggraph: TagGraph::new_populated(&connection),
            folder_map: FolderMap::new_populated(&connection),
            conn: Arc::new(Mutex::new(connection)),
        }

        //TODO dbmap.map.insert(dbmap.largest_id + 1, ret);  //Probably better to do this in another function to avoid self-referentials
    }

    //Invoked by mdbapi::add_tag_to_file()
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

        let folder = path.parent(); //TODO: needed?

        let tags = self.taggraph.get_ancestor_ids(id);
        Ok(FileDetails {
            id,
            path: path.into_os_string().into_string().unwrap(),
            file_type: FileType::Image,
            folder: 0,
            tags,
        }) //TODO: remove the hardcoded 0 for the folder parameter, and file_type as Image
    }

    pub fn new_tag(&self, tag: &TagDetails) -> Option<i64> {
        let mtx = self.conn.lock().expect("Mutex is poisoned");
        let mut stmt = mtx
            .prepare(
                "INSERT OR IGNORE INTO tags (id, name, colour) 
        VALUES (NULL, ?, ?)
        ",
            )
            .unwrap();

        let mut params: Vec<Box<dyn ToSql>> = Vec::new();
        params.push(Box::new(&tag.name));
        params.push(Box::new(tag.colour));
        stmt.execute(params_from_iter(params)).unwrap();

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
                    colour: row.get(2)?,
                })
            })
            .unwrap();

        Vec::from_iter(tag_iter.map(|tag| tag.unwrap()))
    }

    // Given a file details, queries the database for its full path
    pub fn get_file_path(&self, info: &FileDetails) -> PathBuf {
        let lock = self.conn.lock().expect("Mutex is poisoned");
        let stmt = lock.prepare("SELECT (folder.path || image.name) FROM folder RIGHT JOIN image ON folder.id=image.folder");

        let ret: String = stmt.unwrap().query_row([], |row| row.get(0)).unwrap();

        ret.into()
    }

    //Invoked by mdbapi::add_tag_to_file()
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
            "DELETE from tag_records WHERE tag_records.image_id = ? AND tag_records.tag_id = ?",
            [file, tag],
        );
    }
}

/// Based on the extension of a file, checks against the lists of locally stored filetypes and
/// returns an appropriate [FileType]
pub fn path_to_filetype(path: &PathBuf) -> FileType {
    

    todo!();
}

//======= get_files_by_query() helpers=======
pub fn append_tags_clause(tags: &Vec<usize>, params: &mut Vec<Box<dyn ToSql>>) -> String {
    let mut sql = String::new();
    sql += "tag_records.tag_id IN(";

    for tag in tags.iter() {
        sql += "?,";
        params.push(Box::new(tag.clone()));
    }
    sql.pop(); //Delete the trailing comma from the previous for loop
    sql.push(')');

    sql
}

/// Writes the name of the file as a path so that any file with this substring in the name matches in a sql LIKE clause
/// eg: input of dog -> %/%dog%.%
pub fn render_name_for_sql_like_clause(name: String) -> String {
    if name.contains("%") {
        // This would cause very unexpected behaviors because % is an SQL wildcard
        panic!();
    }

    "%/%".to_string() + &name + "%.%"
}

#[cfg(test)]
mod tests {
    use super::render_name_for_sql_like_clause;

    #[test]
    fn test1_render_name_for_sql_like_clause() {
        assert_eq!(
            "%/%dog%.%",
            render_name_for_sql_like_clause("dog".to_string())
        );
    }

    fn test2_render_name_for_sql_like_clause() {
        assert_eq!("%/%x%.%", render_name_for_sql_like_clause("x".to_string()));
    }

    fn test3_render_name_for_sql_like_clause() {
        assert_eq!("%/%%.%", render_name_for_sql_like_clause("".to_string()));
    }
}
