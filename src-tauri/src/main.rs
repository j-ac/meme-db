#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use mdbapi::{GeneralResult, LoadedImage};
use std::{fs::File, io::Read, path::PathBuf, vec::Vec};
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::generate_handler;

mod mdbapi;

/* FRONT END API FUNCTIONS */

#[tauri::command]
async fn get_folders() -> Vec<mdbapi::FolderDetails> {
    mdbapi::get_folders()
}

#[tauri::command]
async fn get_files_by_folder(
    folder: mdbapi::FileID,
    a: usize,
    b: usize,
) -> Vec<mdbapi::FileDetails> {
    mdbapi::get_files_by_folder(folder, 0, 0)
}

#[tauri::command]
async fn get_files_by_tag(tag: mdbapi::TagID, a: usize, b: usize) -> Vec<mdbapi::FileDetails> {
    mdbapi::get_files_by_tag(tag, 0, 0)
}

#[tauri::command]
async fn get_tags() -> Vec<mdbapi::TagDetails> {
    mdbapi::get_tags()
}

#[tauri::command]
async fn add_file_tag(file: mdbapi::FileID, tag: mdbapi::TagID) -> mdbapi::GeneralResult {
    if file == 0 && tag == 0 {
        GeneralResult {
            res: -1,
            res_str: "FUCK!".to_string(),
        }
    } else {
        GeneralResult {
            res: 0,
            res_str: "All good my G.".to_string(),
        }
    }
}

#[tauri::command]
async fn load_image(file: mdbapi::FileID) -> mdbapi::LoadedImage {
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
        Err(_) => String::new(),
    };
    mdbapi::LoadedImage::new(file, b64_string, "jpg".to_string())
}

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
            get_folders,
            get_files_by_folder,
            get_files_by_tag,
            get_tags,
            add_file_tag,
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
