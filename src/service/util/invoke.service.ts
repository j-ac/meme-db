import { Injectable } from '@angular/core';
import { TuiAlertService, TuiNotification } from '@taiga-ui/core';
import { invoke } from '@tauri-apps/api';
import { catchError, from, map, Observable } from 'rxjs';
import { DatabaseService } from '../database/database.service';
import { GUIResult } from './util';

@Injectable({
    providedIn: 'root'
})
export class InvokeService {

    constructor(private databaseService: DatabaseService, private alertService: TuiAlertService) { }

    public invoke<R>(func: API, args: any = {}, message: string|undefined = undefined): Observable<R> {
        args.database = this.databaseService.getUsedDatabase().id;
        return from(invoke<GUIResult<R>>(API[func], args)).pipe(map(
            (res) => {
                if (res.Err !== undefined || res.Ok === undefined) {
                    throw res.Err?.gui_msg || "Critical backend error!";
                }
                return res.Ok;
            }
        // Conceptually this error handling should be one layer up
        )).pipe(catchError((gui_msg) => {
            if (message === undefined) {
                message = API[func]
            }
            console.log(args)
            this.alertService.open(gui_msg,
                {autoClose: false, label: message, status: TuiNotification.Error,})
                .subscribe();
            return [];
        }));
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
}
