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
    folders: FolderDetails[] = []

    constructor() {
        this.sample()
    }

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
}

export interface FolderDetails {
    id: FileID;
    path: string;
}