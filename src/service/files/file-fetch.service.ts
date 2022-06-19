import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { from, Observable, Observer } from 'rxjs';
import { TagID } from '../tags/tag-fetch.service';

@Injectable({
    providedIn: 'root'
})
export class FileFetchService {
    files: FileDetailsNative[] = [];

    constructor() { }

    public getFilesByFolder(folder: FileID, a: number, b: number): Observable<FileDetailsNative[]> {
        return from(invoke<FileDetailsNative[]>('get_files_by_folder', {folder: folder, a: a, b: b}));
    }

    public getFilesByTag(tag: TagID, a: number, b: number): Observable<FileDetailsNative[]> {
        return from(invoke<FileDetailsNative[]>('get_files_by_tag', {tag: tag, a: a, b: b}));
    }
}

export type FileID = number;
export interface FileDetailsNative {
    id: FileID;
    name: String;
    folder: FileID;
    tags: TagID[];
}