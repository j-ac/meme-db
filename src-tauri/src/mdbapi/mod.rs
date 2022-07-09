use std::path::{PathBuf, Path};
use std::option::Option;
use std::vec::Vec;
use serde::{Serialize, Deserialize};
//Stubs go here

pub fn get_files_by_folder(database: DatabaseID, folder: FileID, start: FileID, limit: usize) -> Vec<FileDetails> {
    vec![
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
    ]
}

pub fn get_files_by_tag(database: DatabaseID, tag: TagID, start: FileID, limit: usize) -> Vec<FileDetails> {
    vec![
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
    ]
}

pub fn get_files_by_query(database: DatabaseID, query: FileQuery) -> Result<Vec<FileDetails>> {
    Error::basic_str("Not implemented!")
}

pub fn get_file_by_id(database: DatabaseID, file: FileID) -> Result<&'static Path> {
    let f = match file {
        0 => "C:/Users/Ben/Pictures/meme1.jpg",
        1 => "C:/Users/Ben/Pictures/meme2.jpg",
        2 => "C:/Users/Ben/Pictures/meme3.jpg",
        3 => "C:/Users/Ben/Pictures/meme4.jpg",
        _ => return Error::basic_str("Bad ID!"),
    };
    return Result::Ok(Path::new(f));
}

pub fn get_folders(database: DatabaseID) -> Vec<FolderDetails> {
    vec![
        FolderDetails {
            id: 0,
            path: PathBuf::from("C:/Users/Ben/Pictures/"),
        },
        FolderDetails {
            id: 1,
            path: PathBuf::from("G:/Users/Ben/Pictures/"),
        },
    ]
}

pub fn add_folder<P: AsRef<Path>>(database: DatabaseID, location: P) -> Result<FolderDetails> {
    return Error::basic_str("Not implemented!");
}

pub fn del_folder(database: DatabaseID, folder: FileID) -> Result<()> {
    return Error::basic_str("Not implemented!")
}

pub fn get_tags(database: DatabaseID) -> Vec<TagDetails> {
    vec![
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
    ]
}

pub fn add_file_tag(database: DatabaseID, file: FileID, tag: TagID) -> Result<FileDetails> {
    return Error::basic_str("Not implemented!");
}

pub fn del_file_tag(database: DatabaseID, file: FileID, tag: TagID) -> Result<FileDetails> {
    return Error::basic_str("Not implemented!");
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
        LoadedImage { id: (id), b64_data: (b64_data), format: (format) }
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

//Serde's serialization implementation for std::Result flattens (this does not)
#[derive(Debug, Serialize)]
pub enum GUIResult<T,E> {
    Ok(T),
    Err(E),
}

pub type Result<T> = GUIResult<T, Error>;

impl Error {
    pub fn basic_str<T>(gui_msg: &'static str) -> Result<T> {
        Result::Err(Error { gui_msg: gui_msg.to_string(), err_type: ErrorType::Basic })
    }

    pub fn basic<T>(gui_msg: String) -> Result<T> {
        Result::Err(Error { gui_msg: gui_msg, err_type: ErrorType::Basic })
    }
}

pub mod daemon {
    // There will be more functions regarding callbacks, events, and IPC
    // fn connect() -> Result<DaemonContext>
    // fn on_file_change(cbf: fn(FileDetails))
}
