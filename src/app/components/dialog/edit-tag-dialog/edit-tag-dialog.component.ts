import { Component, Inject, OnInit } from '@angular/core';
import { TuiDialogContext, TuiDialogService } from '@taiga-ui/core';
import { TagDetails, TagFetchService } from 'src/service/tags/tag-fetch.service';
import { POLYMORPHEUS_CONTEXT } from '@tinkoff/ng-polymorpheus';
import { defaultEditorColors } from '@taiga-ui/addon-editor';


@Component({
    selector: 'app-edit-tag-dialog',
    templateUrl: './edit-tag-dialog.component.html',
    styleUrls: ['./edit-tag-dialog.component.scss']
})
export class EditTagDialogComponent implements OnInit {
    tag: TagDetails
    readonly palette = defaultEditorColors


    constructor(
        private readonly dialogService: TuiDialogService,
        @Inject(POLYMORPHEUS_CONTEXT)
        private readonly context: TuiDialogContext<void, TagDetails>,
        private readonly tags: TagFetchService,
    ) {
        let t = this.context.data;
        this.tag = {
            id: t.id,
            name: t.name,
            parents: Array.from(t.parents),
            color: t.color,
        };
    }

    ngOnInit(): void {
    }

    submit(): void {
    }

    addParent() {

    }

    delParent(p: TagDetails) {
        this.tag.parents.splice(this.tag.parents.indexOf(p), 1);
    }
}
