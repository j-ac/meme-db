import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { from, Observable, Observer } from 'rxjs';

@Injectable({
    providedIn: 'root'
})
export class TagFetchService {
    observers: Observer<TagDetails[]>[] = [];
    tagsNative: TagDetailsNative[] = [];
    tags: TagDetails[] = [];
    name_map = new Map<string, TagDetails>()
    tag_map = new Map<TagID, TagDetails>()

    constructor() {
        this.sample()
    }

    public sample() {
        from(invoke<TagDetailsNative[]>('get_tags')).subscribe({
            next: (tags) => {
                this.tagsNative = tags;
                this.tags = [];
                this.name_map.clear();
                this.tag_map.clear();

                let id_lookup = new Map<TagID, TagDetails>();
                for (let tagN of tags) {
                    let tag: TagDetails = { id: tagN.id, name: tagN.name, parents: [] };
                    id_lookup.set(tagN.id, tag);
                    // Add tag to underlying datastructures
                    this.tags.push(tag);
                    this.name_map.set(tag.name, tag);
                    this.tag_map.set(tag.id, tag);
                }
                for (let tagN of this.tagsNative) {
                    let child = id_lookup.get(tagN.id)!;
                    for (let p of tagN.parents) {
                        let parent = id_lookup.get(p)!;
                        child.parents.push(parent);
                    }
                }
                this.tags = Array.from(id_lookup.values());
                for (let obs of this.observers) {
                    obs.next(this.tags);
                }
            }
        })
    }

    public getTags(): Observable<TagDetails[]> {
        return new Observable((obs: Observer<TagDetails[]>) => {
            this.observers.push(obs);
            let observers = this.observers;
            obs.next(this.tags);
            return {
                unsubscribe() {
                    observers.splice(observers.indexOf(obs, 1));
                }
            };
        })
    }

    public getIDByName(name: string): number | undefined {
        return this.name_map.get(name)?.id;
    }

    public getTagByID(id: TagID): TagDetails | undefined {
        return this.tag_map.get(id);
    }

    public getFlattened(tags: TagDetails[]): TagDetails[] {
        let ret: TagDetails[] = Array.from(tags);
        let seenTags = new Set<TagID>();
        for (let i = 0; i < ret.length; i++) {
            let t = ret[i];
            if (seenTags.has(t.id)) {
                continue;
            }
            seenTags.add(t.id);
            for (let parent of t.parents) {
                if (seenTags.has(parent.id)) {
                    continue;
                }
                seenTags.add(parent.id);
                ret.push(parent);
            }
        }
        return ret;
    }
}

export type TagID = number;

export interface TagDetailsNative {
    id: TagID;
    name: string;
    parents: TagID[];
}

export interface TagDetails {
    id: TagID;
    name: string;
    parents: TagDetails[];
    color?: string;
}