import { Injectable, OnInit } from '@angular/core';
import { Data } from '@angular/router';
import { invoke } from '@tauri-apps/api/tauri';
import { from, map, Observable, throwError } from 'rxjs';
import { GUIResult } from '../util/util';

/**
 * For holding GUI selected database info
 */

@Injectable({
    providedIn: 'root'
})
export class DatabaseService implements OnInit {
    private selected_database: number = 0
    public by_id = new Map<DatabaseID, DatabaseDetails>()
    public by_name = new Map<string, DatabaseDetails>()

    constructor() {
        let builtin: DatabaseDetails = { id: 0, name: "Built-in" };
        this.by_id.set(0, builtin);
        this.by_name.set(builtin.name, builtin);
    }

    ngOnInit(): void {
        invoke<DatabaseDetails[]>('get_databases').then((dds) => {
            this.by_id.clear();
            this.by_name.clear();
            for (let d of dds) {
                this.by_id.set(d.id, d);
                this.by_name.set(d.name, d);
            }
        })
    }

    useDatabase(id: DatabaseID) {
        if (this.by_id.has(id)) {
            this.selected_database = id;
        }
    }

    getUsedDatabase(): DatabaseDetails {
        return this.by_id.get(this.selected_database) || { id: 0, name: "Built-in" };
    }

    addDatabase(new_name: string): Observable<DatabaseDetails> {
        return from(invoke<GUIResult<DatabaseDetails>>('add_database', { name: new_name })).pipe(map(
            (res) => {
                if (res.Err !== undefined || res.Ok === undefined) {
                    throw res?.Err?.gui_msg || "Critical backend failure";
                }
                this.by_id.set(res.Ok.id, res.Ok);
                this.by_name.set(res.Ok.name, res.Ok);
                return res.Ok;
            }
        ))
    }

    renameDatabase(id: DatabaseID, new_name: string): Observable<void> {
        return from(invoke<GUIResult<unknown>>('rename_database', { id: id, new_name: new_name, }))
            .pipe(map((res) => {
                if (res.Err !== undefined) {
                    throw res.Err.gui_msg;
                }
            }));
    }

    deleteDatabase(id: DatabaseID): Observable<void> {
        if (!this.by_id.has(id)) {
            return throwError(() => { "Critical GUI error! Tried to delete DB that does not exist." });
        }
        return from(invoke<GUIResult<unknown>>('del_database', { id: id }))
            .pipe(map((res) => {
                if (res.Err !== undefined || res.Ok === undefined) {
                    throw res?.Err?.gui_msg || "Critical backend failure";
                }
                return;
            }));
    }
}

export type DatabaseID = number;
export interface DatabaseDetails {
    id: DatabaseID,
    name: string,
}