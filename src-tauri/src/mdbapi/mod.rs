use rusqlite::{params_from_iter, Params, ToSql};
use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::ops::Deref;
use std::option::Option;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use crate::get_files_by_query;
use crate::mdbapi::database::TagNode;

use self::database::{DatabaseMap, FolderMap};

mod database;

impl Context {
    // TODO: Update this to reflect changes to the database structure
    pub fn get_files_by_query(
        &mut self,
        database: DatabaseID,
        query: FileQuery,
    ) -> GUIResult<DBViewResponse> {
        let size = if self.query_cache.contains_key(&query) {
            self.query_cache.get(&query)
        } else {
            let new_size = self.get_size_of_files_by_query(database, query.clone())?;
            self.query_cache.insert(query.clone(), new_size);
            new_size
        };

        self.retrieve_query_data(database, query, size)
    }

    pub fn retrieve_query_data(
        &mut self,
        database: DatabaseID,
        query: FileQuery,
        total_size: usize, // the size of the query if it had no limitations on size (start = 0, limit = 0)
    ) -> GUIResult<DBViewResponse> {
        let db = (self.dbmap.get(database).unwrap());
        let (sql, params) = query.query_to_sql_and_params(&db.folder_map).unwrap();

        //Execute the statement
        {
            let lock = db.conn.lock().expect("Mutex is poisoned");
            let mut stmt = lock.prepare(&sql).unwrap();
            let mut rows = stmt.query(params_from_iter(params.iter())).unwrap();

            // Construct return value components
            // data
            let mut data = Vec::<FileDetails>::new();
            while let Some(row) = rows.next().unwrap() {
                let file = db.get_details_on_file(row.get(0).unwrap());
                data.push(file.unwrap());
            }

            // new_start
            let new_start = query.start.unwrap_or(0) + data.len();

            Ok(DBViewResponse {
                total_size,
                data,
                new_start,
            })
        }
    }

    /// Determines the size of a query's result if no limits are imposed on its size
    // Does this by doing the query, recording the size of the data and discarding the rest. Very wasteful
    // This function may be able to be eliminated by instead using an SQL view over the query
    pub fn get_size_of_files_by_query(
        &mut self,
        database: DatabaseID,
        query: FileQuery,
    ) -> GUIResult<usize> {
        if !query.is_valid() {
            return Err(Error::basic("Recieved a malformed SQL query.")); //Guarantees vectors contain useful data if they exist, and illegal combinations of data do not occur
        };

        let mut unlimited_query = query;
        unlimited_query.remove_limits();

        let db = (self.dbmap.get(database).unwrap());
        let (sql, params) = unlimited_query
            .query_to_sql_and_params(&db.folder_map)
            .unwrap();

        //Execute the statement
        {
            let lock = db.conn.lock().expect("Mutex is poisoned");
            let mut stmt = lock.prepare(&sql).unwrap();
            let mut rows = stmt.query(params_from_iter(params.iter())).unwrap();

            let mut data = Vec::<FileDetails>::new();
            while let Some(row) = rows.next().unwrap() {
                let file = db.get_details_on_file(row.get(0).unwrap());
                data.push(file.unwrap());
            }

            Ok(data.len())
        }
    }

    pub fn get_folders(&self, database: DatabaseID) -> GUIResult<Vec<FolderDetails>> {
        Ok(vec![
            FolderDetails {
                id: 0,
                path: PathBuf::from("C:/Users/Ben/Pictures/"),
            },
            FolderDetails {
                id: 1,
                path: PathBuf::from("G:/Users/Ben/Pictures/"),
            },
        ])
    }

    pub fn add_folder<P: AsRef<Path>>(
        &self,
        database: DatabaseID,
        location: P,
    ) -> GUIResult<FolderDetails> {
        Err(Error::basic("Not implemented!"))
    }

    pub fn del_folder(&self, database: DatabaseID, folder: FileID) -> GUIResult<()> {
        Err(Error::basic("Not implemented!"))
    }

    pub fn get_tags(&self, database: DatabaseID) -> GUIResult<Vec<TagDetails>> {
        let sql = "SELECT * FROM tag";
        let db = (self.dbmap.get(database).unwrap());

        //Execute the statement
        {
            let lock = db.conn.lock().expect("Mutex is poisoned");
            let mut stmt = lock.prepare(&sql).unwrap();
            let mut rows = stmt.query([]).unwrap();

            // Construct return value components
            // data
            let mut data = Vec::<TagDetails>::new();
            while let Some(row) = rows.next().unwrap() {
                let id = row.get(0).unwrap();
                let tag_info = TagDetails {
                    id,
                    name: (row.get(1).unwrap()),
                    parents: (db.taggraph.get_parent_ids(id)),
                    colour: (row.get(2).unwrap()),
                };
                data.push(tag_info);
            }

            Ok(data)
        }
    }

    /// Without changing the [TagID], updates a tag to have the data supplied in a [TagDetails].
    /// Does not change any tags other than the one supplied. If calling this function would introuce an inconsistency in the tag tree, it should be called additional times to rectify it
    pub fn mod_tag_by_tag(&mut self, database: DatabaseID, tag: TagDetails) -> GUIResult<()> {
        let db = self.dbmap.get_mut(database).ok_or(Error {
            gui_msg: std::format!("Database ID {} not recognised", database),
            err_type: ErrorType::Basic,
        })?;

        let target = db.taggraph.graph.get_mut(&tag.id).unwrap();
        target.parents = tag.parents;
        target.name = tag.name;

        GUIResult::Ok(())
    }

    // Add a tag to the database, and record its data in the TagGraph
    pub fn add_tag(&mut self, database: DatabaseID, new_tag: TagDetails) -> GUIResult<()> {
        let db = self.dbmap.get_mut(database).ok_or(Error {
            gui_msg: std::format!("Database ID {} not recognised", database),
            err_type: ErrorType::Basic,
        })?;

        db.taggraph.insert(new_tag.clone());
        db.new_tag(&new_tag);

        Ok(())
    }

    pub fn add_tag_to_file(
        &self,
        database: DatabaseID,
        file: FileID,
        tag: TagID,
    ) -> GUIResult<FileDetails> {
        let db = self.dbmap.get(database).ok_or(Error {
            gui_msg: std::format!("Database ID {} not recognised", database),
            err_type: ErrorType::Basic,
        })?;

        // For now, inserting a tag inserts all its parents as well. Probably the most wise implementation
        let mut tag_and_ancestors = db.taggraph.get_ancestor_ids(tag);
        tag_and_ancestors.push(tag);

        for element in tag_and_ancestors {
            db.insert_into_tag_records(file, tag);
        }

        db.get_details_on_file(file)
    }

    pub fn del_file_tag(
        &self,
        database: DatabaseID,
        file: FileID,
        tag: TagID,
    ) -> GUIResult<FileDetails> {
        let db = self.dbmap.get(database).ok_or(Error {
            gui_msg: std::format!("Database ID {} not recognised", database),
            err_type: ErrorType::Basic,
        })?;

        db.delete_from_tag_records(file, tag);
        db.get_details_on_file(file)
    }

    pub fn setup() -> Self {
        Self {
            dbmap: DatabaseMap::new_initialized(),
            //folder_map: todo!(),
            query_cache: QuerySizeCache::new(),
        }
    }
}

pub type DatabaseID = usize;
pub type FileID = usize;
pub type TagID = usize;

#[derive(Debug, Serialize)]
pub enum FileType {
    Image,
    Text,
    Video,
    Pdf,
}

#[derive(Debug, Serialize)]
pub struct FileDetails {
    id: FileID,
    path: String,
    folder: FileID,
    tags: Vec<TagID>,
    file_type: FileType,
}

#[derive(Debug, Serialize)]
pub struct FolderDetails {
    id: FileID,
    path: PathBuf,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagDetails {
    pub id: TagID,
    pub name: String,
    pub parents: Vec<TagID>,
    pub colour: usize, //Representation in the GUI. From 0x0 to 0xFFFFFF.
}

#[derive(Debug, Serialize)]
pub struct DatabaseDetails {
    pub id: DatabaseID,
    pub name: String,
    //Others that may be needed
}
#[derive(Debug, Serialize)]
pub struct LoadedImage {
    id: FileID,
    b64_data: String,
    format: String,
}

impl LoadedImage {
    pub fn new(id: FileID, b64_data: String, format: String) -> LoadedImage {
        LoadedImage {
            id: (id),
            b64_data: (b64_data),
            format: (format),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct FileQuery {
    tags_include: Option<Vec<TagID>>, // Include rows WHERE tag_id IN (?,?,?,...)
    include_strong: Option<bool>,     // GROUP BY image_id HAVING COUNT(image_id) = ?
    folders_include: Option<Vec<FileID>>, // Include rows WHERE folder IN (?,?,?,...)
    files_include: Option<Vec<FileID>>,
    names: Option<Vec<String>>, // Include rows WHERE name IN (?,?,?,...)
    limit: Option<usize>,       // LIMIT ?
    start: Option<FileID>,      // Include rows WHERE ROWID > ?
}

impl FileQuery {
    /// Creates a new FileQuery where all field are [None]
    pub fn new() -> FileQuery {
        FileQuery {
            tags_include: None,
            include_strong: None,
            folders_include: None,
            files_include: None,
            names: None,
            limit: None,
            start: None,
        }
    }

    pub fn set_files_include(mut self, files_include: Vec<FileID>) -> Self{
        self.files_include = Some(files_include);
        self
    }

    /// Returns a boolean value indicating if this FileQuery can be converted into valid SQL
    fn is_valid(&self) -> bool {
        let tags_vec = self.tags_include.as_ref();
        let folders_vec = self.folders_include.as_ref();
        let name_vec = self.names.as_ref();
        let files_vec = self.files_include.as_ref();

        // If an empty tag vector is supplied
        if tags_vec.is_some() && tags_vec.unwrap().len() == 0 {
            //didn't use unwrap_and() because it's a nightly feature
            return false;
        }

        // If an empty folder vector is supplied
        if folders_vec.is_some() && folders_vec.unwrap().len() == 0 {
            //didn't use unwrap_and() because it's a nightly feature
            return false;
        }

        // If an empty fileID vector is supplied
        if files_vec.is_some() && files_vec.unwrap().len() == 0 {
            //didn't use unwrap_and() because it's a nightly feature
            return false;
        }

        // If an empty names vector is supplied
        if name_vec.is_some() && folders_vec.unwrap().len() == 0 {
            //didn't use unwrap_and() because it's a nightly feature
            return false;
        }

        // If strong tag inclusion was enabled but no tags were supplied
        if (tags_vec.is_none()) && self.include_strong.unwrap_or(false) {
            return false;
        }

        true
    }

    /// Given a [FileQuery], returns the [Result] of a tuple of its SQL string representation, and a paramters Vector stored in [Box]es of types implementing [ToSql].
    pub fn query_to_sql_and_params(
        &self,
        folder_map: &FolderMap,
    ) -> (Result<(String, Vec<Box<dyn ToSql>>), Error>) {
        if !self.is_valid() {
            return Err(Error::basic("Recieved a malformed SQL query.")); //Guarantees vectors contain useful data if they exist, and illegal combinations of data do not occur
        };

        let mut sql: String = String::new(); // The string which will be executed over the connection
        let mut params: Vec<Box<dyn ToSql>> = Vec::new(); // The parameters passed into the SQL connection with the string
        let mut conditions = vec![]; // Stores the conditions as they're encountered so that they can be added to sql later with AND clauses in between each one

        // JOINs are not strictly neccesarry for all cases, but for generality, all queries will use them
        sql += "SELECT DISTINCT image.id FROM image
        JOIN tag_records ON image.id=tag_records.image_id
        WHERE ";

        if self.folders_include.is_some() {
            // Handles the first element without an OR clause
            let mut cond = "image.path LIKE ?".to_string();
            params.push(Box::new(
                folder_map.get_folder_for_sql_like_clause(
                    *(&self.folders_include)
                        .as_ref()
                        .unwrap()
                        .iter()
                        .next()
                        .unwrap(),
                ),
            ));
            // Handles all subsequent elements with an OR clause prepended
            self.folders_include
                .as_deref()
                .unwrap()
                .iter()
                .skip(1)
                .for_each(|&x| {
                    cond += "OR image.path LIKE ?";
                    params.push(Box::new(x));
                });

            conditions.push(cond);
        }

        if self.folders_include.is_some() {
            // Same algorithm used on folders_include redone for names
            // Handles the first element without an OR clause
            let mut cond = "path LIKE ?".to_string();
            params.push(Box::new(
                crate::mdbapi::database::render_name_for_sql_like_clause(
                    (&self.names)
                        .as_ref()
                        .unwrap()
                        .iter()
                        .next()
                        .unwrap()
                        .clone(),
                ),
            ));
            // Handles all subsequent elements with an OR clause prepended
            self.names.as_deref().unwrap().iter().skip(1).for_each(|x| {
                cond += "OR image.path LIKE ?";
                params.push(Box::new(x.clone()));
            });

            conditions.push(cond);
        }

        if let Some(tags) = (&self.tags_include).as_ref() {
            conditions.push(database::append_tags_clause(tags, &mut params));
        }

        // For each condition insert it into the SQL query
        {
            sql += conditions.iter().next().unwrap(); // Add the first condition without an AND
            conditions.iter().skip(1).for_each(|x| {
                // For each subsequent condition add it with AND prepended
                sql += " AND ";
                sql += x;
            })
        }

        // ==== ALL FOLLOWING CLAUSES DO NOT REQUIRE ANDS BETWEEN THEM ====
        if self.include_strong.unwrap_or(false) {
            sql += format!(
                " GROUP BY tag_records.image_id HAVING COUNT (image_id) = {}",
                self.tags_include.clone().unwrap().len()
            )
            .as_str();
        }

        if let Some(limit) = self.limit {
            sql += " LIMIT ?";
            params.push(Box::new(limit));

            if let Some(offset) = self.start {
                sql += " OFFSET ?";
                params.push(Box::new(offset));
            }
        }

        Ok((sql, params))
    }

    // Queries with the limits removed return an exaustive list of files meeting the criteria without truncation
    pub fn remove_limits(&mut self) {
        self.limit = Some(0);
        self.start = Some(0);
    }
}
/// A cache storing the known size of various queries
/// The size of the query is calculated by doing the query and discarding the result (expensive!), so caching them is important
/// The cache will be flushed any time the database's data changes, as the data may no longer be current
pub struct QuerySizeCache {
    map: HashMap<FileQuery, usize>,
}
impl QuerySizeCache {
    pub fn new() -> Self {
        QuerySizeCache { map: HashMap::new() }
    }

    pub fn insert(&mut self, query: FileQuery, size: usize) {
        self.map.insert(query, size);
    }

    pub fn contains_key(&self, query: &FileQuery) -> bool {
        let mut q = query.clone();
        q.limit = Some(0);
        q.start = Some(0);

        self.map.contains_key(query)
    }

    pub fn get(&self, key: &FileQuery) -> usize {
        self.map.get(key).unwrap().clone()
    }
}

#[derive(Debug, Serialize)]
pub struct DBViewResponse {
    pub data: Vec<FileDetails>,
    new_start: FileID,  //For pagination
    total_size: FileID, //For pagination. The number of results if it were queried with no limit.
}

#[derive(Debug, Serialize)]
pub struct Error {
    pub gui_msg: String,
    pub err_type: ErrorType,
}

#[derive(Debug, Serialize)]
pub enum ErrorType {
    Basic,
    Arguments,
    Internal,
    Logical,
    Filesystem,
    Database,
    SysAPI,
}

pub struct Context {
    pub dbmap: DatabaseMap,
    //folder_map: FolderMap,
    query_cache: QuerySizeCache,
}

pub type GUIResult<T> = Result<T, Error>;

impl Error {
    pub fn basic<S: AsRef<str>>(gui_msg: S) -> Self {
        Error {
            gui_msg: gui_msg.as_ref().to_string(),
            err_type: ErrorType::Basic,
        }
    }

    pub fn filesystem<S: AsRef<str>>(gui_msg: S) -> Self {
        Error {
            gui_msg: gui_msg.as_ref().to_string(),
            err_type: ErrorType::Filesystem,
        }
    }
}

pub mod daemon {
    // There will be more functions regarding callbacks, events, and IPC
    // fn connect() -> Result<DaemonContext>
    // fn on_file_change(cbf: fn(FileDetails))
}
