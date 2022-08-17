use rusqlite::{params_from_iter, Params, ToSql};
use serde::{Deserialize, Serialize};
use std::cmp;
use std::ops::Deref;
use std::option::Option;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use crate::mdbapi::database::TagNode;

use self::database::{DatabaseMap, FolderMap};

mod database;

impl Context {
    pub fn get_files_by_folder(
        &self,
        database: DatabaseID,
        folder: FileID,
        start: FileID,
        limit: usize,
    ) -> GUIResult<Vec<FileDetails>> {
        Ok(vec![
            FileDetails {
                id: 0,
                name: "meme1.jpg".to_string(),
                folder: 0,
                tags: vec![0, 1],
                file_type: FileType::Image,
            },
            FileDetails {
                id: 1,
                name: "meme2.jpg".to_string(),
                folder: 0,
                tags: vec![1],
                file_type: FileType::Image,
            },
            FileDetails {
                id: 2,
                name: "meme3.jpg".to_string(),
                folder: 0,
                tags: vec![0],
                file_type: FileType::Image,
            },
            FileDetails {
                id: 3,
                name: "meme4.jpg".to_string(),
                folder: 0,
                tags: vec![2],
                file_type: FileType::Image,
            },
        ])
    }

    pub fn get_files_by_tag(
        &self,
        database: DatabaseID,
        tag: TagID,
        start: FileID, //Necessary?
        limit: usize,
    ) -> GUIResult<Vec<FileDetails>> {
        let db = self.dbmap.get(database).unwrap();
        db.get_files_by_tag(tag, start)
    }

    /// Temporarially broken. If names or folders_include are in the query, ALL other parts of the query will be ignored
    pub fn get_files_by_query(
        &self,
        database: DatabaseID,
        query: FileQuery,
    ) -> GUIResult<DBViewResponse> {
        if !query.is_valid() {
            return Err(Error::basic("Recieved a malformed SQL query.")); //Guarantees vectors contain useful data if they exist, and illegal combinations of data do not occur
        };

        let db = self.dbmap.get(database).unwrap();
        let mut sql: String = String::new(); // The string which will be executed over the connection
        let mut params: Vec<Box<dyn ToSql>> = Vec::new(); // The parameters passed into the SQL connection with the string

        let mut num_ands = query.count_sql_and_clauses(); // how many AND clauses are in the prompt.
        let mut need_and = false; //After a clause is written, need_and is set to true so that an AND is inserted before another clause

        //These two queries require an SQL JOIN, so they're better off isolated
        if query.names.is_some() || query.folders_include.is_some() {
            sql += "SELECT DISTINCT image.id FROM image 
            JOIN tag_records ON image.id=tag_records.image_id
            WHERE ";

            // Handles the first element without an OR clause
            sql += "image.path LIKE ?";
            params.push(Box::new(
                self.folder_map.get_folder_for_sql_like_clause(
                    (&query.folders_include)
                        .as_ref()
                        .unwrap()
                        .iter()
                        .next()
                        .unwrap()
                        .clone(),
                ),
            ));
            // Handles all subsequent elements with an OR clause prepended
            query
                .folders_include
                .as_deref()
                .unwrap()
                .iter()
                .skip(1)
                .for_each(|&x| {
                    sql += "OR image.path LIKE ?";
                    params.push(Box::new(x.clone()));
                });

            if query.names.is_some() && query.folders_include.is_some() {
                //if both are being performed, an AND is needed between them
                sql += "AND ";
            }

            //Previous algorithm repeated for the names
            // Handles the first element without an OR clause
            sql += "path LIKE ?";
            params.push(Box::new(
                crate::mdbapi::database::render_name_for_sql_like_clause(
                    (&query.names)
                        .as_ref()
                        .unwrap()
                        .iter()
                        .next()
                        .unwrap()
                        .clone(),
                ),
            ));
            // Handles all subsequent elements with an OR clause prepended
            query
                .names
                .as_deref()
                .unwrap()
                .iter()
                .skip(1)
                .for_each(|x| {
                    sql += "OR image.path LIKE ?";
                    params.push(Box::new(x.clone()));
                });
        }
        //Queries that do not require an SQL JOIN
        else {
            sql += "SELECT tag_records.image_id FROM tag_records WHERE ";
            if let Some(tags) = (&query.tags_include).as_ref() {
                sql += &database::append_tags_clause(tags, &mut params);
                need_and = true;
            }

            if num_ands > 1 && need_and {
                sql = database::append_and_clause(&mut need_and, &mut num_ands);
            }

            // Min defaults to 0 which is equivalent to not having a minimum
            sql += "rowid > ?";
            params.push(Box::new(query.start.unwrap_or(0))); //Default to 0 if no start was defined

            if query.include_strong.unwrap_or(false) {
                sql += format!(
                    " GROUP BY tag_records.image_id HAVING COUNT (image_id) = {}",
                    query.tags_include.unwrap().len()
                )
                .as_str();
            }

            if let Some(limit) = query.limit {
                sql += " LIMIT ?";
                params.push(Box::new(limit));
            }
        }

        //Execute the statement
        if let lock = db.conn.lock().expect("Mutex is poisoned") {
            let mut stmt = lock.prepare(&sql).unwrap();
            let mut rows = stmt.query(params_from_iter(params.iter())).unwrap();


            let mut data = Vec::<FileDetails>::new();
            while let Some(row) = rows.next().unwrap() {
                let file = db.get_details_on_file(row.get(0).unwrap());
                data.push(file.unwrap());
            }
            let new_start = query.start.unwrap_or(0) + data.len();
            Ok(DBViewResponse {total_size: data.len(), data, new_start, })
        }
        else {
            panic!();
        }
    }

    pub fn get_file_by_id(&self, database: DatabaseID, file: FileID) -> GUIResult<PathBuf> {
        match file {
            n @ 0..=3 => Ok(format!("C:/Users/Ben/Pictures/meme{}.jpg", n + 1).into()),
            _ => Err(Error::basic("Bad ID!")),
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
        Ok(vec![
            TagDetails {
                id: 0,
                name: "TagA".to_string(),
                parents: vec![],
            },
            TagDetails {
                id: 1,
                name: "TagB".to_string(),
                parents: vec![],
            },
            TagDetails {
                id: 2,
                name: "TagC".to_string(),
                parents: vec![1, 0],
            },
            TagDetails {
                id: 3,
                name: "TagC1".to_string(),
                parents: vec![2],
            },
            TagDetails {
                id: 4,
                name: "TagC2".to_string(),
                parents: vec![2],
            },
            TagDetails {
                id: 5,
                name: "TagC3".to_string(),
                parents: vec![2],
            },
            TagDetails {
                id: 6,
                name: "TagC31".to_string(),
                parents: vec![5],
            },
        ])
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

    pub fn add_tag(&self, database: DatabaseID, new_tag: TagDetails) -> GUIResult<()> {
        Err(Error::basic("Not implemented!"))
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
            dbmap: todo!(),
            folder_map: todo!(),
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
    PDF,
}

#[derive(Debug, Serialize)]
pub struct FileDetails {
    id: FileID,
    name: String,
    folder: FileID,
    tags: Vec<TagID>,
    file_type: FileType,
}

#[derive(Debug, Serialize)]
pub struct FolderDetails {
    id: FileID,
    path: PathBuf,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TagDetails {
    pub id: TagID,
    pub name: String,
    pub parents: Vec<TagID>,
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

#[derive(Debug, Deserialize)]
pub struct FileQuery {
    tags_include: Option<Vec<TagID>>, // Include rows WHERE tag_id IN (?,?,?,...)
    include_strong: Option<bool>,     // GROUP BY image_id HAVING COUNT(image_id) = ?
    folders_include: Option<Vec<FileID>>, // Include rows WHERE folder IN (?,?,?,...)
    names: Option<Vec<String>>,       // Include rows WHERE name IN (?,?,?,...)
    limit: Option<usize>,             // LIMIT ?
    start: Option<FileID>,            // Include rows WHERE ROWID > ?
}

impl FileQuery {
    /// Returns a boolean value indicating if this FileQuery can be converted into valid SQL
    fn is_valid(&self) -> bool {
        let tags_vec = self.tags_include.as_ref();
        let folders_vec = self.folders_include.as_ref();
        let name_vec = self.names.as_ref();

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

        // If an empty name vector is supplied
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

    /// Returns the number of AND clauses needed for this query to be well-formed SQL
    fn count_sql_and_clauses(&self) -> usize {
        let mut count = -1;

        if self.tags_include.is_some() {
            count += 1;
        }

        if self.folders_include.is_some() {
            count += 1;
        }

        if self.start.is_some() {
            count += 1;
        }

        if self.names.is_some() {
            count += 1;
        }

        // no AND clause is used for a LIMIT statement
        // no AND clause is used for a HAVING COUNT statement

        cmp::max(count, 0) as usize
    }
}
#[derive(Debug, Serialize)]
pub struct DBViewResponse {
    data: Vec<FileDetails>,
    new_start: FileID,  //For pagination
    total_size: FileID, //For pagination
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
    dbmap: DatabaseMap,
    folder_map: FolderMap,
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
