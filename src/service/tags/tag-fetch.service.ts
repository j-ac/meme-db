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

    constructor() { }

    public sample() {
        from(invoke<TagDetailsNative[]>('get_tags')).subscribe({
            next: (tags) => {
                this.tagsNative = tags;
                this.tags = [];

                let id_lookup = new Map<number, TagDetails>();
                for (let tagN of tags) {
                    let tag: TagDetails = { id: tagN.id, name: tagN.name, parents: [] };
                    id_lookup.set(tagN.id, tag);
                    this.tags.push(tag);
                }
                for (let tagN of this.tagsNative) {
                    let child = id_lookup.get(tagN.id)!;
                    for (let p of tagN.parents) {
                        let parent = id_lookup.get(p)!;
                        child.parents.push(parent);
                    }
                }
            }
        })
    }

    public getTags(): TagDetails[] {
        return this.tags;
    }

    public subscribe(obs: Observer<TagDetails[]>) {
        this.observers.push(obs)
    }

    public unsubscribe(obs: Observer<TagDetails[]>) {
        const found = this.observers.findIndex((other, idx) => {
            return obs == other
        })
        this.observers.splice(found, 1);
    }
}

export interface TagDetailsNative {
    id: number;
    name: String;
    parents: number[];
}

export interface TagDetails {
    id: number;
    name: String;
    parents: TagDetails[];
}