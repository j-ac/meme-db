#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused)]

use mdbapi::*;
use serde::Serialize;
use std::{
    borrow::BorrowMut, cell::Cell, fs::File, io::Read, ops::DerefMut, path::PathBuf, sync::{Mutex, Arc},
    vec::Vec,
};
use tauri::{generate_handler, Manager, State};

mod mdbapi;

struct MDBAPIState {
    ctx: Mutex<Context>,
}

impl MDBAPIState {
    fn exec<T, F>(&self, body: F) -> GUIResult<T>
    where
        T: Serialize,
        F: FnOnce(&mut Context) -> GUIResult<T>,
    {
        let mut ctx = self.ctx.lock().expect("State mutex poisoned.");
        body(&mut ctx)
    }
}

/* FRONT END API FUNCTIONS */
/* FRONT END TAG API */

#[tauri::command]
async fn get_tags(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
) -> GUIResult<Vec<TagDetails>> {
    state.exec(|ctx| ctx.get_tags(database))
}

#[tauri::command]
async fn mod_tag(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
    tag: TagDetails,
) -> GUIResult<()> {
    state.exec(|ctx| ctx.mod_tag_by_tag(database, tag))
}

#[tauri::command]
async fn add_tag(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
    new_tag: TagDetails,
) -> GUIResult<()> {
    state.exec(|ctx| ctx.add_tag(database, new_tag))
}

/* FRONT END TAG API END*/
/* FRONT END FILE API */

#[tauri::command]
async fn add_file_tag(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
    file: FileID,
    tag: TagID,
) -> GUIResult<FileDetails> {
    state.exec(|ctx| ctx.add_file_tag(database, file, tag))
}

#[tauri::command]
async fn del_file_tag(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
    file: FileID,
    tag: TagID,
) -> GUIResult<FileDetails> {
    state.exec(|ctx| ctx.del_file_tag(database, file, tag))
}

#[tauri::command]
async fn get_folders(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
) -> GUIResult<Vec<FolderDetails>> {
    state.exec(|ctx| ctx.get_folders(database))
}

#[tauri::command]
async fn add_folder(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
    path: String,
) -> GUIResult<FolderDetails> {
    state.exec(|ctx| ctx.add_folder(database, path))
}

#[tauri::command]
async fn del_folder(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
    folder: FileID,
) -> GUIResult<()> {
    state.exec(|ctx| ctx.del_folder(database, folder))
}

#[tauri::command]
async fn get_files_by_query(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
    query: FileQuery,
) -> GUIResult<DBViewResponse> {
    state.exec(|ctx| ctx.get_files_by_query(database, query))
}

/* FRONT END FILE API END */
/* FRONT END DATABASE API */
#[tauri::command]
async fn get_databases(state: State<'_, MDBAPIState>) -> GUIResult<Vec<DatabaseDetails>> {
    state.exec(|ctx| {
        Ok(vec![DatabaseDetails {
            id: 0,
            name: "Built-in".to_string(),
        }])
    })
}

#[tauri::command]
async fn add_database(state: State<'_, MDBAPIState>, name: String) -> GUIResult<DatabaseDetails> {
    state.exec(|ctx| Err(Error::basic("Not implemented!")))
}

#[tauri::command]
async fn del_database(state: State<'_, MDBAPIState>, id: DatabaseID) -> GUIResult<()> {
    state.exec(|ctx| Err(Error::basic("Not implemented!")))
}

#[tauri::command]
async fn rename_database(
    state: State<'_, MDBAPIState>,
    id: DatabaseID,
    new_name: String,
) -> GUIResult<()> {
    state.exec(|ctx| Err(Error::basic("Not implemented!")))
}

/* FRONT END DATABASE API END */
/* FRONT END MISC API */

#[tauri::command]
async fn load_image(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
    file: FileID,
) -> GUIResult<LoadedImage> {
    state.exec(|ctx| {
        let mut retval = Vec::new();
        let f = ctx.get_file_by_id(database, file)?;
        let b64_string = match File::open(f)
            .and_then(|mut im_file: File| im_file.read_to_end(&mut retval))
        {
            Result::Ok(_) => base64::encode(retval),
            Result::Err(e) => return Err(Error::basic(std::format!("read_to_end failed: {e}"))),
        };
        Ok(LoadedImage::new(file, b64_string, "jpg".to_string()))
    })
}

#[tauri::command]
async fn load_text(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
    file: FileID,
) -> GUIResult<String> {
    state.exec(|ctx| {
        let mut retval = String::new();
        let f = ctx.get_file_by_id(database, file)?;
        File::open(f)
            .and_then(|mut text_file: File| text_file.read_to_string(&mut retval))
            .or_else(|_| Err(Error::filesystem("Failed to read the selected file")))?;
        Ok(retval)
    })
}

#[tauri::command]
async fn load_video(
    state: State<'_, MDBAPIState>,
    database: DatabaseID,
    file: FileID,
) -> GUIResult<String> {
    /**
     * The thesis of this function is to create a soft link to the existing file and send that soft link.
     *
     * The reason why we have to do this is because Tauri does not allow you to read
     * arbitrary files from the GUI. This way the file is always in the same location.
     */
    Err(Error::basic("Not implemented!"))
}

/* FRONT END MISC API END */
/* FRONT END API FUNCTIONS */

/* APPLICATION FUNCTIONS */

struct BinaryConfig {}

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
            load_text,
            load_video,
        ])
        .setup(|app| {
            let ctx = Mutex::new(Context::setup());
            app.manage(ctx);
            std::result::Result::Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
