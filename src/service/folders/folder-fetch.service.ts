import { NgIfContext } from '@angular/common';
import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { from, Observable, Observer } from 'rxjs';
import { FileID } from '../files/file-fetch.service';

@Injectable({
    providedIn: 'root'
})
export class FolderFetchService {
    observers: Observer<FolderDetails[]>[] = []
    obs_map: Observer<Map<FileID, FolderDetails>>[] = []
    folders: FolderDetails[] = []
    folder_map = new Map<FileID, FolderDetails>()
    name_map = new Map<string, FolderDetails>()

    constructor() {
        this.sample()
    }

    public sample() {
        from(invoke<FolderDetails[]>('get_folders')).subscribe({
            next: (fd) => {
                this.folders = fd;
                this.folder_map.clear();
                this.name_map.clear();
                for (let f of fd) {
                    this.folder_map.set(f.id, f);
                    this.name_map.set(f.path, f);
                }
                for (let obs of this.observers) {
                    obs.next(fd);
                }
                for (let obs of this.obs_map) {
                    obs.next(this.folder_map);
                }
            }
        })
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