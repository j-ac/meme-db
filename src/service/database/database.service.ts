import { Injectable, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import { from, Observable, throwError } from 'rxjs';

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
        this.getDatabases();
    }

    getDatabases() {
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
        return from(invoke<DatabaseDetails>('add_database', { name: new_name }));
    }

    renameDatabase(id: DatabaseID, new_name: string): Observable<void> {
        return from(invoke<void>('rename_database', { id: id, new_name: new_name, }));
    }

    deleteDatabase(id: DatabaseID): Observable<void> {
        if (!this.by_id.has(id)) {
            return throwError(() => { "Critical GUI error! Tried to delete DB that does not exist." });
        }
        return from(invoke<void>('del_database', { id: id }));
    }
}

export type DatabaseID = number;
export interface DatabaseDetails {
    id: DatabaseID,
    name: string,
}