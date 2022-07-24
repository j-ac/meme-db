#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#![allow(unused)]

use mdbapi::*;
use std::{fs::File, io::Read, path::PathBuf, vec::Vec};
use tauri::{generate_handler, Manager, State};

mod mdbapi;

/* FRONT END API FUNCTIONS */
/* FRONT END TAG API */

#[tauri::command]
async fn get_tags(
    ctx: State<'_, Context>,
    database: DatabaseID,
) -> GUIResult<Vec<TagDetails>> {
    ctx.get_tags(database)
}

#[tauri::command]
async fn mod_tag(
    ctx: State<'_, Context>,
    database: DatabaseID,
    tag: TagDetails,
) -> GUIResult<()> {
    ctx.mod_tag_by_tag(database, tag)
}

#[tauri::command]
async fn add_tag(
    ctx: State<'_, Context>,
    database: DatabaseID,
    new_tag: TagDetails,
) -> GUIResult<()> {
    ctx.add_tag(database, new_tag)
}

/* FRONT END TAG API END*/
/* FRONT END FILE API */

#[tauri::command]
async fn add_file_tag(
    ctx: State<'_, Context>,
    database: DatabaseID,
    file: FileID,
    tag: TagID,
) -> GUIResult<FileDetails> {
    ctx.add_file_tag(database, file, tag)
}

#[tauri::command]
async fn del_file_tag(
    ctx: State<'_, Context>,
    database: DatabaseID,
    file: FileID,
    tag: TagID,
) -> GUIResult<FileDetails> {
    ctx.del_file_tag(database, file, tag)
}

#[tauri::command]
async fn get_folders(
    ctx: State<'_, Context>,
    database: DatabaseID,
) -> GUIResult<Vec<FolderDetails>> {
    ctx.get_folders(database)
}

#[tauri::command]
async fn add_folder(
    ctx: State<'_, Context>,
    database: DatabaseID,
    path: String,
) -> GUIResult<FolderDetails> {
    ctx.add_folder(database, path)
}

#[tauri::command]
async fn del_folder(
    ctx: State<'_, Context>,
    database: DatabaseID,
    folder: FileID,
) -> GUIResult<()> {
    ctx.del_folder(database, folder)
}

#[tauri::command]
async fn get_files_by_query(
    ctx: State<'_, Context>,
    database: DatabaseID,
    query: FileQuery,
) -> GUIResult<Vec<FileDetails>> {
    ctx.get_files_by_query(database, query)
}

/* FRONT END FILE API END */
/* FRONT END DATABASE API */
#[tauri::command]
async fn get_databases(ctx: State<'_, Context>) -> GUIResult<Vec<DatabaseDetails>> {
    Ok(vec![DatabaseDetails {
        id: 0,
        name: "Built-in".to_string(),
    }])
}

#[tauri::command]
async fn add_database(ctx: State<'_, Context>, name: String) -> GUIResult<DatabaseDetails> {
    Err(Error::basic("Not implemented!"))
}

#[tauri::command]
async fn del_database(ctx: State<'_, Context>, id: DatabaseID) -> GUIResult<()> {
    Err(Error::basic("Not implemented!"))
}

#[tauri::command]
async fn rename_database(
    ctx: State<'_, Context>,
    id: DatabaseID,
    new_name: String,
) -> GUIResult<()> {
    Err(Error::basic("Not implemented!"))
}

/* FRONT END DATABASE API END */
/* FRONT END MISC API */

#[tauri::command]
async fn load_image(
    ctx: State<'_, Context>,
    database: DatabaseID,
    file: FileID,
) -> GUIResult<LoadedImage> {
    let mut retval = Vec::new();
    let f = match ctx.get_file_by_id(database, file) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    let b64_string = match File::open(f).and_then(|mut im_file: File| {
        im_file.read_to_end(&mut retval)
    }) {
        Result::Ok(_) => base64::encode(retval),
        Result::Err(e) => return Err(Error::basic(std::format!("read_to_end failed: {e}"))),
    };
    Ok(LoadedImage::new(
        file,
        b64_string,
        "jpg".to_string(),
    ))
}
/* FRONT END MISC API END */
/* FRONT END API FUNCTIONS */

/* APPLICATION FUNCTIONS */

struct BinaryConfig {
}

#[cfg(all(target_os = "windows", debug_assertions))]
fn get_binary_config() -> BinaryConfig {
    BinaryConfig {}
}

#[cfg(all(target_os = "windows", not(debug_assertions)))]
fn get_binary_config() -> BinaryConfig {}

#[cfg(target_os = "macos")]
fn get_binary_config() -> BinaryConfig {}

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
fn get_binary_config() -> BinaryConfig {}

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            //TAG API
            get_tags,
            mod_tag,
            add_tag,
            //FILE API
            get_folders,
            add_folder,
            del_folder,
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
        .setup(|app| {
            let ctx = Context::setup();
            app.manage(ctx);
            std::result::Result::Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
