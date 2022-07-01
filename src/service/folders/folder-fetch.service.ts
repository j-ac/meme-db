import { NgIfContext } from '@angular/common';
import { Injectable } from '@angular/core';
import { from, map, Observable, Observer } from 'rxjs';
import { DatabaseService } from '../database/database.service';
import { FileID } from '../files/file-fetch.service';
import { API, InvokeService } from '../util/invoke.service';
import { GUIResult } from '../util/util';

@Injectable({
    providedIn: 'root'
})
export class FolderFetchService {
    observers: Observer<FolderDetails[]>[] = []
    obs_map: Observer<Map<FileID, FolderDetails>>[] = []
    folders: FolderDetails[] = []
    folder_map = new Map<FileID, FolderDetails>()
    name_map = new Map<string, FolderDetails>()

    constructor(private mdbapi: InvokeService) {
        this.sample()
    }

    public sample() {
        this.mdbapi.invoke_nores<FolderDetails[]>(API.get_folders).subscribe({
            next: (fd) => {
                this.folders = fd;
                this.folder_map.clear();
                this.name_map.clear();
                for (let f of fd) {
                    this.folder_map.set(f.id, f);
                    this.name_map.set(f.path, f);
                }
                this.sendFolders();
            }
        })
    }

    private sendFolders() {
        for (let obs of this.observers) {
            obs.next(this.folders);
        }
        for (let obs of this.obs_map) {
            obs.next(this.folder_map);
        }
    }

    public getFolders(): Observable<FolderDetails[]> {
        return new Observable((obs: Observer<FolderDetails[]>) => {
            this.observers.push(obs);
            let observers = this.observers;
            obs.next(this.folders)
            return {
                unsubscribe() {
                    observers.splice(observers.indexOf(obs, 1));
                }
            };
        })
    }

    public addFolder(path: string): Observable<FolderDetails> {
        let args = {
            path: path,
        };
        return this.mdbapi.invoke<FolderDetails>(API.add_folder, args).pipe(map((fd) => {
            this.folders.push(fd);
            this.sendFolders();
            return fd;
        }))
    }

    public delFolder(f: FolderDetails): Observable<void> {
        let args = {
            folder: f.id,
        };
        return this.mdbapi.invoke<null>(API.del_folder, args).pipe(map(() => {
            this.folders.splice(this.folders.findIndex((v) => { v.id === f.id }), 1);
            this.sendFolders();
            return;
        }))
    }

    public getFolderMap(): Observable<Map<FileID, FolderDetails>> {
        return new Observable((obs) => {
            this.obs_map.push(obs);
            let observers = this.obs_map;
            obs.next(this.folder_map);
            return {
                unsubscribe() {
                    observers.splice(observers.indexOf(obs, 1));
                }
            }
        })
    }

    public getIDByName(name: string): FileID | undefined {
        return this.name_map.get(name)?.id;
    }
}

export interface FolderDetails {
    id: FileID;
    path: string;
}