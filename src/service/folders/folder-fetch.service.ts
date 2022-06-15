import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { from, Observer } from 'rxjs';

@Injectable({
    providedIn: 'root'
})
export class FolderFetchService {
    observers: Observer<FolderDetails[]>[] = []
    folders: FolderDetails[] = []

    constructor() {  }

    public sample() {
        from(invoke<FolderDetails[]>('get_folders')).subscribe({
            next: (fd) => {
                this.folders = fd;
                for (let obs of this.observers) {
                    obs.next(fd);
                }
            }
        })
    }

    public getFolders() {
        return this.folders;
    }

    public subscribe(obs: Observer<FolderDetails[]>) {
        this.observers.push(obs);
    }

    public unsubscribe(obs: Observer<FolderDetails[]>) {
        const found = this.observers.findIndex((other, idx) => {
            return obs == other
        })
        this.observers.splice(found, 1);
    }
}

export interface FolderDetails {
    id: number;
    path: string;
}