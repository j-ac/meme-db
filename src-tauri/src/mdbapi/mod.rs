
use std::path::PathBuf;
use std::vec::Vec;
use serde::Serialize;
//Stubs go here

pub fn get_files_by_folder(folder: FileID, a: usize, b: usize) -> Vec<FileDetails> {
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

pub fn get_files_by_tag(tag: TagID, a: usize, b: usize) -> Vec<FileDetails> {
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

pub fn get_folders() -> Vec<FolderDetails> {
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

pub fn get_tags() -> Vec<TagDetails> {
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
// fn post_tag(name: String, parents: Optional<Vec<TagID>>) -> Result<TagDetails>
// fn delete_tag(tag: TagID) -> bool
// fn patch_tag_add_parents(tag: TagID, parents: Vec<TagID>) -> Result<TagDetails>
// fn patch_tag_del_parents(tag: TagID, parents: Vec<TagID>) -> Result<TagDetails>
// fn patch_tag_name(tag: TagID, name: String) -> Result<TagDetails>

// fn patch_file_tags(file: FileID, tags_add: Vec<TagID>) -> Result<FileDetails>
// fn delete_file_tags(file: FileID, tags_removed: Vec<TagID>) -> Result<FileDetails>
// fn purge_file_tags(file: FileID) -> Result<FileDetails>
// fn put_file_tags(file: FileID, tags: Vec<TagID>) -> Result<FileDetails>

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
    id: TagID,
    name: String,
    parents: Vec<TagID>,
}

pub mod daemon {
    // There will be more functions regarding callbacks, events, and IPC
    // fn connect() -> Result<DaemonContext>
    // fn on_file_change(cbf: fn(FileDetails))
}
