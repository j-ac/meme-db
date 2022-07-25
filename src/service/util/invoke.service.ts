import { Injectable } from '@angular/core';
import { TuiAlertService, TuiNotification } from '@taiga-ui/core';
import { invoke } from '@tauri-apps/api';
import { catchError, from, map, Observable } from 'rxjs';
import { DatabaseService } from '../database/database.service';
import { Error } from './util';

export interface InvokeOptions<R> {
    preHook?: ((r: R) => void);
}

export interface MDBAPIOptions<R> extends InvokeOptions<R> {
    message?: string;
    error_status?: TuiNotification;
    auto_close?: boolean
}

@Injectable({
    providedIn: 'root'
})
export class MDBAPI {

    constructor(private databaseService: DatabaseService, private alertService: TuiAlertService, private invs: InvokeService) { }

    /**
     * 
     * @param func API function to invoke
     * @param args Args to API
     * @param params Additional settings for dialogs and handling
     * @returns The result<R> from the API
     */
    public call<R>(
        func: API,
        args: any = {},
        params: MDBAPIOptions<R> = {},
    ): Observable<R> {
        args.database = this.databaseService.getUsedDatabase().id;
        return this.invs.invoke(func, args, params)
            .pipe(catchError((err: Error) => {
                this.alertService.open(err.gui_msg,
                    {
                        autoClose: params.auto_close || false,
                        label: params.message || API[func],
                        status: params.error_status || TuiNotification.Error,
                    }).subscribe();
                return [];
            }));
    }

    public call_rores<R>(func: API, args: any = {}): Observable<R> {
        args.database = this.databaseService.getUsedDatabase().id;
        return from(invoke<R>(API[func], args));
    }
}

@Injectable({
    providedIn: 'root'
})
export class InvokeService {
    public invoke<R>(func: API, args: any = {}, params: InvokeOptions<R> = {}): Observable<R> {
        return from(invoke<R>(API[func], args)
            .then((r: R) => {
                if (params.preHook)
                    params.preHook(r);
                return r;
            })
        );
    }
}

export enum API {
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
}
