import { Component, Inject, OnDestroy, OnInit } from '@angular/core';
import { EMPTY_ARRAY, TuiHandler } from '@taiga-ui/cdk';
import { TuiDialogContext } from '@taiga-ui/core';
import { Subscription } from 'rxjs';
import { TagDetails, TagFetchService, TagID } from 'src/service/tags/tag-fetch.service';
import { POLYMORPHEUS_CONTEXT } from '@tinkoff/ng-polymorpheus';


@Component({
    selector: 'app-tag-select-dialog',
    templateUrl: './tag-select-dialog.component.html',
    styleUrls: ['./tag-select-dialog.component.scss'],
})
export class TagSelectDialogComponent implements OnInit, OnDestroy {
    child_lookup = new Map<TagID, TagDetails[]>();
    tags: TagDetails[] = [];
    root: TagDetails = { id: -1, name: "Root", parents: [] };
    colors: string[] = ["silver", "maroon", "purple", "olivedrab", "navy", "darkorange", "indigo", "yellow", "teal", "turquoise", "skyblue", "seagreen", "sandybrown", "red"];
    subscription?: Subscription

    constructor(
        private tagFetch: TagFetchService,
        @Inject(POLYMORPHEUS_CONTEXT)
        private readonly context: TuiDialogContext<TagDetails, void>,) {

    }

    /**
     * Translate bottum up architecture to top down (parents, child_lookup)
     */
    loadTags(tags: TagDetails[]) {
        let child_lookup = new Map<TagID, TagDetails[]>();
        let parents: TagDetails[] = [];
        for (let tag of tags) {
            if (!child_lookup.has(tag.id)) {
                child_lookup.set(tag.id, []);
            }
            if (tag.parents.length === 0) {
                parents.push(tag);
            }
            if (!tag.color) {
                tag.color = this.colors[tag.id % this.colors.length];
            }
            for (let parent of tag.parents) {
                let children: TagDetails[] = []
                if (!child_lookup.has(parent.id)) {
                    child_lookup.set(parent.id, children);
                } else {
                    children = child_lookup.get(parent.id)!;
                }
                children.push(tag);
            }
        }
        this.root = { id: -1, name: "Root", parents: parents };
        child_lookup.set(this.root.id, parents);
        this.tags = tags;
        this.child_lookup = child_lookup;
    }

    ngOnInit(): void {
        this.subscription = this.tagFetch.getTags().subscribe({
            next: (tags: TagDetails[]) => {
                this.loadTags(Array.from(tags))
            }
        });
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe()
    }

    readonly handler: TuiHandler<TagDetails, readonly TagDetails[]> = item => {
        return this.child_lookup.get(item.id) || EMPTY_ARRAY;
    }

    tagSelected(tag: TagDetails) {
        this.context.completeWith(tag);
    }

    falsePred(): boolean {
        return false;
    }
}
