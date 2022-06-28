export class Util {
}

//mdbapi::Result
export type GUIResult<T> = Result<T, Error>

//std::result::Result
export interface Result<T, E> {
    Ok?: T,
    Err?: E,
}

//mdbapi::Error
export interface Error {
    gui_msg: string,
    err_type: any | undefined, //May not be relevant to GUI
}