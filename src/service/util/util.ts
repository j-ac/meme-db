export class Util {
}

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