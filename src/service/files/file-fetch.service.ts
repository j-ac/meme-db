import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { from, map, Observable, Observer, tap } from 'rxjs';
import { TagID } from '../tags/tag-fetch.service';

@Injectable({
    providedIn: 'root'
})
export class FileFetchService {
    files: FileDetailsNative[] = [];
    image_cache = new Map<FileID, CacheEntry>();

    constructor() {
        setInterval(() => {
            const now = Date.now()
            for (let [k, v] of this.image_cache) {
                if (v.timestamp + 45_000 < now) {
                    this.image_cache.delete(k)
                }
            }
        }, 200)
    }

    public getFilesByFolder(folder: FileID, a: number, b: number): Observable<FileDetailsNative[]> {
        return from(invoke<FileDetailsNative[]>('get_files_by_folder', { folder: folder, a: a, b: b }));
    }

    public getFilesByTag(tag: TagID, a: number, b: number): Observable<FileDetailsNative[]> {
        return from(invoke<FileDetailsNative[]>('get_files_by_tag', { tag: tag, a: a, b: b }));
    }

    public getImage(file: FileID): Observable<HTMLImageElement> {
        var fulfill: (_: HTMLImageElement | PromiseLike<HTMLImageElement>) => void
        var reject: (_: any) => void

        var retval = new Promise<HTMLImageElement>((onFulfilled, onRejected) => {
            fulfill = onFulfilled;
            reject = onRejected;
        })

        if (this.image_cache.has(file)) {
            let retImage = this.image_cache.get(file)!;
            return new Observable((obs) => {
                obs.next(retImage.image);
                retImage.timestamp = Date.now();
                return {
                    unsubscribe() {
                    }
                };
            })
        }

        invoke<LoadedImage>('load_image', { file: file }).then((image_data) => {
            let image = new Image();
            image.onload = () => {
                this.image_cache.set(file, new CacheEntry(image));
                fulfill(image)
            }
            image.src = `data:image/${image_data.format};base64,${image_data.b64_data}`;
        }).catch((reason) => reject(reason))
        return from(retval)
    }
}

export type FileID = number;
export interface FileDetailsNative {
    id: FileID;
    name: string;
    folder: FileID;
    tags: TagID[];
}

interface LoadedImage {
    id: FileID;
    b64_data: string;
    format: string;
}

class CacheEntry {
    public timestamp: number = Date.now()
    constructor(public image: HTMLImageElement) {}
}