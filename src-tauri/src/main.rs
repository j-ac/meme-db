#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use mdbapi::GUIResult::{Err, Ok};
use mdbapi::{DatabaseDetails, DatabaseID, FileDetails, FolderDetails, GUIResult};
use std::{fs::File, io::Read, path::PathBuf, result, vec::Vec};
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::{generate_handler, Manager};

mod mdbapi;

/* FRONT END API FUNCTIONS */
/* FRONT END TAG API */

#[tauri::command]
async fn get_tags(database: DatabaseID) -> Vec<mdbapi::TagDetails> {
    mdbapi::get_tags(database)
}

/* FRONT END TAG API END*/
/* FRONT END FILE API */

#[tauri::command]
async fn add_file_tag(
    database: DatabaseID,
    file: mdbapi::FileID,
    tag: mdbapi::TagID,
) -> mdbapi::Result<FileDetails> {
    return mdbapi::add_file_tag(database, file, tag);
}

#[tauri::command]
async fn del_file_tag(
    database: DatabaseID,
    file: mdbapi::FileID,
    tag: mdbapi::TagID,
) -> mdbapi::Result<FileDetails> {
    return mdbapi::del_file_tag(database, file, tag);
}

#[tauri::command]
async fn get_folders(database: DatabaseID) -> Vec<mdbapi::FolderDetails> {
    return mdbapi::get_folders(database);
}

#[tauri::command]
async fn add_folder(database: DatabaseID, path: String) -> mdbapi::Result<FolderDetails> {
    return mdbapi::add_folder(database, path);
}

#[tauri::command]
async fn del_folder(database: DatabaseID, folder: mdbapi::FileID) -> mdbapi::Result<()> {
    return mdbapi::del_folder(database, folder);
}

#[tauri::command]
async fn get_files_by_folder(
    database: DatabaseID,
    folder: mdbapi::FileID,
    start: mdbapi::FileID,
    limit: usize,
) -> Vec<mdbapi::FileDetails> {
    mdbapi::get_files_by_folder(database, folder, start, limit)
}

#[tauri::command]
async fn get_files_by_tag(
    database: DatabaseID,
    tag: mdbapi::TagID,
    start: mdbapi::FileID,
    limit: usize,
) -> Vec<mdbapi::FileDetails> {
    mdbapi::get_files_by_tag(database, tag, start, limit)
}

#[tauri::command]
async fn get_files_by_query(
    database: DatabaseID,
    query: mdbapi::FileQuery,
) -> mdbapi::Result<Vec<mdbapi::FileDetails>> {
    mdbapi::get_files_by_query(database, query)
}

/* FRONT END FILE API END */
/* FRONT END DATABASE API */
#[tauri::command]
async fn get_databases() -> Vec<mdbapi::DatabaseDetails> {
    return vec![mdbapi::DatabaseDetails {
        id: 0,
        name: "global".to_string(),
    }];
}

#[tauri::command]
async fn add_database(name: String) -> mdbapi::Result<DatabaseDetails> {
    return mdbapi::Error::basic_str("Not implemented!");
}

#[tauri::command]
async fn del_database(id: DatabaseID) -> mdbapi::Result<()> {
    return mdbapi::Error::basic_str("Not implemented!");
}

#[tauri::command]
async fn rename_database(id: DatabaseID, new_name: String) -> mdbapi::Result<()> {
    return mdbapi::Error::basic_str("Not implemented!");
}

/* FRONT END DATABASE API END */
/* FRONT END MISC API */

#[tauri::command]
async fn load_image(
    database: DatabaseID,
    file: mdbapi::FileID,
) -> mdbapi::Result<mdbapi::LoadedImage> {
    let mut retval = Vec::new();
    let f = match mdbapi::get_file_by_id(database, file) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    let b64_string = match File::open(f).and_then(|mut im_file: File| {
        let rd = im_file.read_to_end(&mut retval);
        return rd;
    }) {
        Result::Ok(_) => base64::encode(retval),
        Result::Err(e) => return mdbapi::Error::basic(std::format!("read_to_end failed: {e}")),
    };
    GUIResult::Ok(mdbapi::LoadedImage::new(
        file,
        b64_string,
        "jpg".to_string(),
    ))
}
/* FRONT END MISC API END */
/* FRONT END API FUNCTIONS */

/* APPLICATION FUNCTIONS */

struct BinaryConfig {
    daemon_location: PathBuf,
}

#[cfg(all(target_os = "windows", debug_assertions))]
fn get_binary_config() -> BinaryConfig {
    BinaryConfig {
        daemon_location: PathBuf::from("./target/debug/meme-db-daemon.exe"),
    }
}

#[cfg(all(target_os = "windows", not(debug_assertions)))]
fn get_binary_config() -> BinaryConfig {}

#[cfg(target_os = "macos")]
fn get_binary_config() -> BinaryConfig {}

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
fn get_binary_config() -> BinaryConfig {}

fn main() {
    enforce_daemon();
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            //TAG API
            get_tags,
            //FILE API
            get_folders,
            add_folder,
            del_folder,
            get_files_by_folder,
            get_files_by_tag,
            get_files_by_query,
            add_file_tag,
            del_file_tag,
            //DATABASE API
            get_databases,
            add_database,
            del_database,
            rename_database,
            //MISC API
            load_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn enforce_daemon() {
    let bcfg = get_binary_config();
    let sys = System::new_all();
    let memes = sys
        .processes()
        .iter()
        .filter(|(_, proc)| proc.name().starts_with("meme-db-daemon"));
    let count = memes.clone().count();
    if count == 1 {
        println!("Daemon already started.");
        return;
    }
    if count > 1 {
        println!("Too many deamons running! Killing imposters (sussy)!");
        memes.for_each(|(_, proc)| {
            proc.kill();
        });
    }

    println!("Starting daemon!");
    //TODO: FE-BE can still run without daemon, just some features won't work.
    //TODO: process will block termination waiting for the daemon to terminate
    std::process::Command::new(bcfg.daemon_location)
        .spawn()
        .expect("Can't start daemon");
}
