import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { from, map, Observable } from 'rxjs';
import { DatabaseService } from '../database/database.service';
import { GUIResult } from './util';

@Injectable({
    providedIn: 'root'
})
export class InvokeService {

    constructor(private databaseService: DatabaseService) { }

    public invoke<R>(func: API, args: any = {}): Observable<R> {
        args.database = this.databaseService.getUsedDatabase().id;
        return from(invoke<GUIResult<R>>(API[func], args)).pipe(map(
            (res) => {
                if (res.Err !== undefined || res.Ok === undefined) {
                    throw res.Err?.gui_msg || "Critical backend error!";
                }
                return res.Ok;
            }
        ));
    }

    public invoke_nores<R>(func: API, args: any = {}): Observable<R> {
        args.database = this.databaseService.getUsedDatabase().id;
        return from(invoke<R>(API[func], args));
    }
}

export enum API {
    //TAG API
    get_tags,
    //FILE API
    get_folders,
    add_folder,
    del_folder,
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
}
