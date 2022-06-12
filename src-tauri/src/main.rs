#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::vec::Vec;
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::generate_handler;

#[tauri::command]
async fn get_folders() -> Vec<String> {
    println!("GET FOLDERS!");
    return std::vec!["./".to_string(), "././".to_string()];
}

#[tauri::command]
async fn get_files(folder: String, a: usize, b: usize) -> Vec<String> {
    println!("GET FILES!");
    return std::vec!["Test.txt".to_string(), "ab.png".to_string()];
}

fn main() {
    enforce_daemon();
    tauri::Builder::default()
        .invoke_handler(generate_handler![get_folders, get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn enforce_daemon() {
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
    //TODO: not portable
    //TODO: process will block termination waiting for the daemon to terminate
    std::process::Command::new("./target/debug/meme-db-daemon.exe")
        .spawn()
        .expect("Can't start daemon");
}
