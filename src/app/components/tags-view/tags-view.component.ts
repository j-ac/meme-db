import { ChangeDetectionStrategy, Component, OnInit } from '@angular/core';
import { EMPTY_ARRAY, TuiHandler } from '@taiga-ui/cdk';
import { TagDetails, TagFetchService, TagID } from 'src/service/tags/tag-fetch.service';

@Component({
    selector: 'app-tags-view',
    templateUrl: './tags-view.component.html',
    styleUrls: ['./tags-view.component.scss'],
    changeDetection: ChangeDetectionStrategy.Default,
})
export class TagsViewComponent implements OnInit {
    child_lookup = new Map<TagID, TagDetails[]>();
    tags: TagDetails[] = [];
    root: TagDetails = { id: -1, name: "Root", parents: [] };
    colors: string[] = ["silver", "maroon", "purple", "olivedrab", "navy", "darkorange", "indigo", "yellow", "teal", "turquoise", "skyblue", "seagreen", "sandybrown", "red"];

    constructor(private tagFetch: TagFetchService) { }

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
        this.tagFetch.getTags().subscribe({
            next: (tags: TagDetails[]) => {
                this.loadTags(Array.from(tags))
            }
        });
    }

    readonly handler: TuiHandler<TagDetails, readonly TagDetails[]> = item => {
        return this.child_lookup.get(item.id) || EMPTY_ARRAY;
    }
}