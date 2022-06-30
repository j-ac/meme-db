import { Injectable } from '@angular/core';
import { TuiAlertService, TuiNotification } from '@taiga-ui/core';
import { invoke } from '@tauri-apps/api';
import { from, map, Observable, Observer, of, tap } from 'rxjs';
import { TagDetails, TagFetchService, TagID } from '../tags/tag-fetch.service';
import { GUIResult } from '../util/util';

@Injectable({
    providedIn: 'root'
})
export class FileFetchService {
    files: FileDetailsNative[] = [];
    image_cache = new Map<FileID, CacheEntry>();

    constructor(private tagFetch: TagFetchService) {
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
        return from(invoke<FileDetailsNative[]>('get_files_by_folder', { folder: folder, start: start, limit: limit }));
    }

    public getFilesByTagNative(tag: TagID, start: FileID, limit: number): Observable<FileDetailsNative[]> {
        return from(invoke<FileDetailsNative[]>('get_files_by_tag', { tag: tag, start: start, limit: limit }));
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

        invoke<GUIResult<LoadedImage>>('load_image', { file: file }).then((image_data) => {
            if (image_data.Err !== undefined) {
                throw image_data.Err.gui_msg;
            }
            let image = new Image();
            image.onload = () => {
                this.image_cache.set(file, new CacheEntry(image));
                fulfill(image)
            }
            image.src = `data:image/${image_data.Ok!.format};base64,${image_data.Ok!.b64_data}`;
        }).catch((reason) => reject(reason))
        return from(retval)
    }

    public addTag(file: FileID, tag: TagID): Observable<GUIResult<any>> {
        return from(invoke<GUIResult<any>>('add_file_tag', { file: file, tag: tag }))
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
    folders_exclude?: FileID[];
    names?: string[];
    ids?: FileID[];
}

class CacheEntry {
    public timestamp: number = Date.now()
    constructor(public image: HTMLImageElement) { }
}