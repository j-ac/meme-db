import { Injectable } from '@angular/core';
import { TuiAlertService, TuiNotification } from '@taiga-ui/core';
import { invoke } from '@tauri-apps/api';
import { from, map, Observable, Observer, of, switchMap, tap } from 'rxjs';
import { TagDetails, TagDetailsNative, TagFetchService, TagID } from '../tags/tag-fetch.service';
import { API, InvokeService } from '../util/invoke.service';
import { GUIResult } from '../util/util';

@Injectable({
    providedIn: 'root'
})
export class FileFetchService {
    files: FileDetailsNative[] = [];
    image_cache = new Map<FileID, CacheEntry>();

    constructor(
        private tagFetch: TagFetchService,
        private mdbapi: InvokeService,
    ) {
        setInterval(() => {
            const now = Date.now()
            for (let [k, v] of this.image_cache) {
                if (v.timestamp + 45_000 < now) {
                    this.image_cache.delete(k)
                }
            }
        }, 200)
    }

    public getFilesByFolderNative(folder: FileID, start: FileID, limit: number): Observable<FileDetailsNative[]> {
        const args = { folder: folder, start: start, limit: limit };
        return this.mdbapi.invoke_nores<FileDetailsNative[]>(API.get_files_by_folder, args);
    }

    public getFilesByTagNative(tag: TagID, start: FileID, limit: number): Observable<FileDetailsNative[]> {
        let args = { tag: tag, start: start, limit: limit };
        return this.mdbapi.invoke_nores<FileDetailsNative[]>(API.get_files_by_tag, args);
    }

    private convertFromNative(native: FileDetailsNative[]): FileDetails[] {
        let ret: FileDetails[] = [];
        for (let n of native) {
            let toAdd: FileDetails = { folder: n.folder, name: n.name, id: n.id, tags: [] };
            for (let t of n.tags) {
                // We must ensure that tags always get updated before files do.
                toAdd.tags.push(this.tagFetch.getTagByID(t)!);
            }
            ret.push(toAdd);
        }
        return ret;
    }

    public getFilesByFolder(folder: FileID, start: FileID, limit: number): Observable<FileDetails[]> {
        return this.getFilesByFolderNative(folder, start, limit)
            .pipe(map((native) => {
                return this.convertFromNative(native);
            }))
    }

    public getFilesByTag(tag: TagID, start: FileID, limit: number): Observable<FileDetails[]> {
        return this.getFilesByTagNative(tag, start, limit)
            .pipe(map((native) => {
                return this.convertFromNative(native);
            }))
    }

    public getFilesByQuery(query: FileQuery) {
        let args = { query: query };
        return this.mdbapi.invoke<FileDetailsNative[]>(API.get_files_by_query, args)
            .pipe(map((native) => {
                return this.convertFromNative(native);
            }));
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

        return this.mdbapi.invoke<LoadedImage>(API.load_image, { file: file }).pipe(switchMap((image_data) => {
            let image = new Image();
            image.onload = () => {
                this.image_cache.set(file, new CacheEntry(image));
                fulfill(image)
            }
            image.src = `data:image/${image_data.format};base64,${image_data.b64_data}`;
            return from(retval);
        }))
    }

    public addTag(file: FileID, tag: TagID): Observable<FileDetails> {
        return this.mdbapi.invoke<FileDetailsNative>(API.add_file_tag, { file: file, tag: tag })
            .pipe(switchMap((native) => {
                return this.convertFromNative([native]);
            }));
    }

    public delTag(file: FileID, tag: TagID): Observable<FileDetails> {
        let args = { file: file, tag: tag };
        return this.mdbapi.invoke<FileDetailsNative>(API.del_file_tag, args)
            .pipe(switchMap((native) => {
                return this.convertFromNative([native]);
            }));
    }
}

export type FileID = number;
export interface FileDetailsNative {
    id: FileID;
    name: string;
    folder: FileID;
    tags: TagID[];
}

export interface FileDetails {
    id: FileID;
    name: string;
    folder: FileID;
    tags: TagDetails[];
}

export interface LoadedImage {
    id: FileID;
    b64_data: string;
    format: string;
}

export interface FileQuery {
    tags_include?: TagID[];
    tags_exclude?: TagID[];
    folders_include?: FileID[];
    names?: string[];
}

class CacheEntry {
    public timestamp: number = Date.now()
    constructor(public image: HTMLImageElement) { }
}