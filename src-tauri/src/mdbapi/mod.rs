use serde::{Deserialize, Serialize};
use std::option::Option;
use std::path::{Path, PathBuf};
use std::vec::Vec;

impl Context {
    pub fn get_files_by_folder(
        &self,
        database: DatabaseID,
        folder: FileID,
        start: FileID,
        limit: usize,
    ) -> GUIResult<Vec<FileDetails>> {
        Success(vec![
            FileDetails {
                id: 0,
                name: "meme1.jpg".to_string(),
                folder: 0,
                tags: vec![0, 1],
            },
            FileDetails {
                id: 1,
                name: "meme2.jpg".to_string(),
                folder: 0,
                tags: vec![1],
            },
            FileDetails {
                id: 2,
                name: "meme3.jpg".to_string(),
                folder: 0,
                tags: vec![0],
            },
            FileDetails {
                id: 3,
                name: "meme4.jpg".to_string(),
                folder: 0,
                tags: vec![2],
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
        Success(vec![
            FileDetails {
                id: 0,
                name: "meme1.jpg".to_string(),
                folder: 0,
                tags: vec![0, 1],
            },
            FileDetails {
                id: 1,
                name: "meme2.jpg".to_string(),
                folder: 0,
                tags: vec![1],
            },
            FileDetails {
                id: 2,
                name: "meme3.jpg".to_string(),
                folder: 0,
                tags: vec![0],
            },
            FileDetails {
                id: 3,
                name: "meme4.jpg".to_string(),
                folder: 0,
                tags: vec![2],
            },
        ])
    }

    pub fn get_files_by_query(
        &self,
        database: DatabaseID,
        query: FileQuery,
    ) -> GUIResult<Vec<FileDetails>> {
        Error::basic_str("Not implemented!")
    }

    pub fn get_file_by_id(&self, database: DatabaseID, file: FileID) -> GUIResult<&'static Path> {
        let f = match file {
            0 => "C:/Users/Ben/Pictures/meme1.jpg",
            1 => "C:/Users/Ben/Pictures/meme2.jpg",
            2 => "C:/Users/Ben/Pictures/meme3.jpg",
            3 => "C:/Users/Ben/Pictures/meme4.jpg",
            _ => return Error::basic_str("Bad ID!"),
        };
        return Success(Path::new(f));
    }

    pub fn get_folders(&self, database: DatabaseID) -> GUIResult<Vec<FolderDetails>> {
        Success(vec![
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
        return Error::basic_str("Not implemented!");
    }

    pub fn del_folder(&self, database: DatabaseID, folder: FileID) -> GUIResult<()> {
        return Error::basic_str("Not implemented!");
    }

    pub fn get_tags(&self, database: DatabaseID) -> GUIResult<Vec<TagDetails>> {
        Success(vec![
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

    pub fn mod_tag_by_tag(&self, database: DatabaseID, tag: TagDetails) -> GUIResult<TagDetails> {
        Error::basic_str("Not implemented!")
    }

    pub fn add_file_tag(
        &self,
        database: DatabaseID,
        file: FileID,
        tag: TagID,
    ) -> GUIResult<FileDetails> {
        return Error::basic_str("Not implemented!");
    }

    pub fn del_file_tag(
        &self,
        database: DatabaseID,
        file: FileID,
        tag: TagID,
    ) -> GUIResult<FileDetails> {
        return Error::basic_str("Not implemented!");
    }

    pub fn setup() -> Self {
        return Self {};
    }
}

pub type DatabaseID = usize;
pub type FileID = usize;
pub type TagID = usize;

#[derive(Debug, Serialize)]
pub struct FileDetails {
    id: FileID,
    name: String,
    folder: FileID,
    tags: Vec<TagID>,
}

#[derive(Debug, Serialize)]
pub struct FolderDetails {
    id: FileID,
    path: PathBuf,
}
#[derive(Debug, Serialize)]
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
    tags_include: Option<Vec<TagID>>,
    tags_exclude: Option<Vec<TagID>>,
    folders_include: Option<Vec<FileID>>,
    names: Option<Vec<String>>,
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
    Filesystem,
    Database,
    SysAPI,
}

pub struct Context {}

pub type GUIResult<T> = Result<T, Error>;

impl Error {
    pub fn basic_str<T>(gui_msg: &'static str) -> GUIResult<T> {
        GUIResult::Err(Error {
            gui_msg: gui_msg.to_string(),
            err_type: ErrorType::Basic,
        })
    }

    pub fn basic<T>(gui_msg: String) -> GUIResult<T> {
        GUIResult::Err(Error {
            gui_msg: gui_msg,
            err_type: ErrorType::Basic,
        })
    }
}

pub fn Success<T>(t: T) -> GUIResult<T> {
    GUIResult::Ok(t)
}

pub mod daemon {
    // There will be more functions regarding callbacks, events, and IPC
    // fn connect() -> Result<DaemonContext>
    // fn on_file_change(cbf: fn(FileDetails))
}
