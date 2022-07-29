import { Injectable, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import { from, map, Observable, tap, throwError } from 'rxjs';
import { API, InvokeService, MDBAPI } from '../util/invoke.service';

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

    constructor(private invk: InvokeService) {
        let builtin: DatabaseDetails = { id: 0, name: "Built-in" };
        this.by_id.set(0, builtin);
        this.by_name.set(builtin.name, builtin);
    }

    ngOnInit(): void {
        this.getDatabases();
    }

    getDatabases() {
        this.invk.invoke<DatabaseDetails[]>(API.get_databases, undefined,
            {
                preHook: (dds) => {
                    this.by_id.clear();
                    this.by_name.clear();
                    for (let d of dds) {
                        this.by_id.set(d.id, d);
                        this.by_name.set(d.name, d);
                    }
                }
            });
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
        return this.invk.invoke<DatabaseDetails>(API.add_database, { name: new_name },
            {
                preHook: (dd) => {
                    this.by_id.set(dd.id, dd);
                    this.by_name.set(dd.name, dd);
                }
            });
    }

    renameDatabase(id: DatabaseID, new_name: string): Observable<DatabaseDetails> {
        return this.invk.invoke<DatabaseDetails>(API.rename_database, { id: id, new_name: new_name, },
            {
                preHook: (dd) => {
                    this.by_id.set(dd.id, dd);
                    this.by_name.set(dd.name, dd);
                }
            });
    }

    deleteDatabase(id: DatabaseID): Observable<void> {
        let deled = this.by_id.get(id);
        if (deled === undefined) {
            return throwError(() => { "Critical GUI error! Tried to delete DB that does not exist." });
        }
        return this.invk.invoke<void>(API.del_database, { id: id },
            {
                preHook: () => {
                    this.by_id.delete(id);
                    this.by_name.delete(deled!.name);
                }
            })
    }
}

export type DatabaseID = number;
export interface DatabaseDetails {
    id: DatabaseID,
    name: string,
}