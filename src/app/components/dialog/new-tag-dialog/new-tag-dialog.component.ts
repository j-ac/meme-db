import { Component, Inject, OnInit } from '@angular/core';
import { TagDetails, TagFetchService } from 'src/service/tags/tag-fetch.service';
import { POLYMORPHEUS_CONTEXT } from '@tinkoff/ng-polymorpheus';
import { defaultEditorColors } from '@taiga-ui/addon-editor';
import { TuiAlertService, TuiDialogContext, TuiDialogService } from '@taiga-ui/core';

@Component({
    selector: 'app-new-tag-dialog',
    templateUrl: './new-tag-dialog.component.html',
    styleUrls: ['./new-tag-dialog.component.scss']
})
export class NewTagDialogComponent implements OnInit {
    tag: TagDetails = { name: "new tag", id: -1, parents: [], color: "#bbbbbb" }

    constructor(private readonly dialogService: TuiDialogService,
        @Inject(POLYMORPHEUS_CONTEXT)
        private readonly context: TuiDialogContext<void, void>,
        private readonly tags: TagFetchService,
        private alertService: TuiAlertService,) { }

    ngOnInit(): void {
    }

    submit() {

    }

    delParent(tag: TagDetails) {

    }

    addParent() {
        
    }

}
