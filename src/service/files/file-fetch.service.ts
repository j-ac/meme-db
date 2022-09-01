import { Injectable } from '@angular/core';
import { from, map, Observable, switchMap } from 'rxjs';
import { TagDetails, TagFetchService, TagID } from '../tags/tag-fetch.service';
import { API, MDBAPI } from '../util/invoke.service';

@Injectable({
    providedIn: 'root'
})
export class FileFetchService {
    files: FileDetailsNative[] = [];
    image_cache = new Map<FileID, CacheEntry>();

    constructor(
        private tagFetch: TagFetchService,
        private mdbapi: MDBAPI,
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

    private convertFromNative(native: FileDetailsNative[]): FileDetails[] {
        let ret: FileDetails[] = [];
        for (let n of native) {
            let toAdd: FileDetails = {
                folder: n.folder,
                name: n.name,
                id: n.id,
                tags: [],
                file_type: SupportedFileType.Image
            };
            for (let t of n.tags) {
                // We must ensure that tags always get updated before files do.
                toAdd.tags.push(this.tagFetch.getTagByID(t)!);
            }
            ret.push(toAdd);
        }
        return ret;
    }


    public getFilesByQuery(query: FileQuery) {
        let args = { query: query };
        return this.mdbapi.call<DBViewResponse>(API.get_files_by_query, args)
            .pipe(map((res) => {
                return this.convertFromNative(res.data);
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

        return this.mdbapi.call<LoadedImage>(API.load_image, { file: file }).pipe(switchMap((image_data) => {
            let image = new Image();
            image.onload = () => {
                this.image_cache.set(file, new CacheEntry(image));
                fulfill(image)
            }
            image.src = `data:image/${image_data.format};base64,${image_data.b64_data}`;
            return from(retval);
        }))
    }

    public getText(file: FileID): Observable<string> {
        return from([]);
    }

    public getVideo(file: FileID): string {
        return "NO RESOURCE URL SET";
    }

    public addTag(file: FileID, tag: TagID): Observable<FileDetails> {
        return this.mdbapi.call<FileDetailsNative>(API.add_file_tag, { file: file, tag: tag })
            .pipe(switchMap((native) => {
                return this.convertFromNative([native]);
            }));
    }

    public delTag(file: FileID, tag: TagID): Observable<FileDetails> {
        let args = { file: file, tag: tag };
        return this.mdbapi.call<FileDetailsNative>(API.del_file_tag, args)
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
    file_type: SupportedFileType
}

export enum SupportedFileType {
    Image = "Image",
    Text = "Text",
    Video = "Video",
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

export interface DBViewResponse {
    data: FileDetailsNative[],
    new_start: FileID, //For pagination
    total_size: FileID, //For pagination
}

class CacheEntry {
    public timestamp: number = Date.now()
    constructor(public image: HTMLImageElement) { }
}

export function cloneFlatten(tagFetch: TagFetchService, f: FileDetails): FileDetails {
    return {
        id: f.id,
        name: f.name,
        folder: f.folder,
        tags: tagFetch.getFlattened(f.tags),
        file_type: f.file_type,
    };
}