import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { from, Observable, Observer } from 'rxjs';

@Injectable({
    providedIn: 'root'
})
export class FileFetchService {
    observers:  Observer<FileDetailsNative[]>[] = [];
    files: FileDetailsNative[] = [];

    constructor() { }

    public sample(folder: number, a: number, b: number) {
        from(invoke<FileDetailsNative[]>('get_files', {folder: folder, a: a, b: b})).subscribe({next: (files) => {
            this.files = files;
            for (let obs of this.observers) {
                obs.next(this.files);
            }
        }});
    }

    public subscribe(obs: Observer<FileDetailsNative[]>) {
        this.observers.push(obs)
    }

    public unsubscribe(obs: Observer<FileDetailsNative[]>) {
        const found = this.observers.findIndex((other, idx) => {
            return obs == other
        })
        this.observers.splice(found, 1);
    }
}

export interface FileDetailsNative {
    id: number;
    name: String;
    folder: number;
    tags: number[];
}