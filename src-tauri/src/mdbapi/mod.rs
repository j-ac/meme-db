use serde::{Deserialize, Serialize};
use std::option::Option;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use self::database::DatabaseMap;

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

    pub fn get_files_by_query(
        &self,
        database: DatabaseID,
        query: FileQuery,
    ) -> GUIResult<DBViewResponse> {
        Err(Error::basic("Not implemented!"))
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

    pub fn mod_tag_by_tag(&mut self, database: DatabaseID, tag: TagDetails) -> GUIResult<()> {
        Err(Error::basic("Not implemented!"))
    }

    pub fn add_tag(&self, database: DatabaseID, new_tag: TagDetails) -> GUIResult<()> {
        Err(Error::basic("Not implemented!"))
    }

    pub fn add_file_tag(
        &self,
        database: DatabaseID,
        file: FileID,
        tag: TagID,
    ) -> GUIResult<FileDetails> {
        let db = self.dbmap.get(database).ok_or(Error {
            gui_msg: std::format!("Database ID {} not recognised", database),
            err_type: ErrorType::Basic,
        })?;

        db.insert_into_tag_records(file, tag);
        db.get_details_on_file(tag)
    }

    pub fn del_file_tag(
        &self,
        database: DatabaseID,
        file: FileID,
        tag: TagID,
    ) -> GUIResult<FileDetails> {
        Err(Error::basic("Not implemented!"))
    }

    pub fn setup() -> Self {
        Self { dbmap: todo!() }
    }
}

pub type DatabaseID = usize;
pub type FileID = usize;
pub type TagID = usize;

#[derive(Debug, Serialize)]
enum FileType {
    Image,
    Text,
    Video,
}

#[derive(Debug, Serialize)]
pub struct FileDetails {
    id: FileID,
    name: String,
    folder: FileID,
    tags: Vec<TagID>,
    file_type: FileType
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
    include_strong: Option<bool>, // GROUP BY image_id HAVING COUNT(image_id) = ?
    folders_include: Option<Vec<FileID>>, // Include rows WHERE folder IN (?,?,?,...)
    names: Option<Vec<String>>, // Include rows WHERE name IN (?,?,?,...)
    limit: Option<usize>, // LIMIT ?
    start: Option<FileID>, // Include rows WHERE ROWID > ?
}

#[derive(Debug, Serialize)]
pub struct DBViewResponse {
    data: Vec<FileDetails>,
    new_start: FileID, //For pagination
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
