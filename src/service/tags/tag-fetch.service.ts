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

    constructor() {
        this.sample()
     }

    public sample() {
        from(invoke<TagDetailsNative[]>('get_tags')).subscribe({
            next: (tags) => {
                this.tagsNative = tags;
                this.tags = [];

                let id_lookup = new Map<TagID, TagDetails>();
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