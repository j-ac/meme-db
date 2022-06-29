#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use mdbapi::{DatabaseDetails, DatabaseID, GUIResult};
use std::{fs::File, io::Read, path::PathBuf, vec::Vec};
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::generate_handler;

mod mdbapi;

/* FRONT END API FUNCTIONS */
/* FRONT END TAG API */

#[tauri::command]
async fn get_tags() -> Vec<mdbapi::TagDetails> {
    mdbapi::get_tags(0)
}

/* FRONT END TAG API END*/
/* FRONT END FILE API */

#[tauri::command]
async fn add_file_tag(file: mdbapi::FileID, tag: mdbapi::TagID) -> mdbapi::Result<()> {
    if file == 0 && tag == 0 {
        mdbapi::Error::basic_str("FUCK!")
    } else {
        GUIResult::Ok(())
    }
}

#[tauri::command]
async fn get_folders() -> Vec<mdbapi::FolderDetails> {
    mdbapi::get_folders(0)
}

#[tauri::command]
async fn get_files_by_folder(
    folder: mdbapi::FileID,
    a: usize,
    b: usize,
) -> Vec<mdbapi::FileDetails> {
    mdbapi::get_files_by_folder(0, folder, 0, 0)
}

#[tauri::command]
async fn get_files_by_tag(tag: mdbapi::TagID, a: usize, b: usize) -> Vec<mdbapi::FileDetails> {
    mdbapi::get_files_by_tag(0, tag, 0, 0)
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
async fn load_image(file: mdbapi::FileID) -> mdbapi::Result<mdbapi::LoadedImage> {
    let mut retval = Vec::new();
    let f = match file {
        0 => "C:/Users/Ben/Pictures/meme1.jpg",
        1 => "C:/Users/Ben/Pictures/meme2.jpg",
        2 => "C:/Users/Ben/Pictures/meme3.jpg",
        3 => "C:/Users/Ben/Pictures/meme4.jpg",
        _ => "C:/Users/Ben/Pictures/meme1.jpg",
    };
    let b64_string = match File::open(f).and_then(|mut im_file: File| {
        let rd = im_file.read_to_end(&mut retval);
        return rd;
    }) {
        Ok(_) => base64::encode(retval),
        Err(e) => return mdbapi::Error::basic(std::format!("read_to_end failed: {e}")),
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
            get_files_by_folder,
            get_files_by_tag,
            add_file_tag,
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
