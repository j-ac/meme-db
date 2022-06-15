#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{path::PathBuf, vec::Vec};
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::generate_handler;

mod mdbapi;

#[tauri::command]
async fn get_folders() -> Vec<mdbapi::FolderDetails> {
    mdbapi::get_folders()
}

#[tauri::command]
async fn get_files(folder: mdbapi::FileID, a: usize, b: usize) -> Vec<mdbapi::FileDetails> {
    mdbapi::get_files(folder, 0, 0)
}

#[tauri::command]
async fn get_tags() -> Vec<mdbapi::TagDetails> {
    mdbapi::get_tags()
}

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
        .invoke_handler(generate_handler![get_folders, get_files, get_tags])
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
