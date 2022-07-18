import { Component, Inject, Injector, OnInit } from '@angular/core';
import { TuiAlertService, TuiDialogContext, TuiDialogService, TuiNotification } from '@taiga-ui/core';
import { TagDetails, TagFetchService } from 'src/service/tags/tag-fetch.service';
import { POLYMORPHEUS_CONTEXT } from '@tinkoff/ng-polymorpheus';
import { defaultEditorColors } from '@taiga-ui/addon-editor';
import { TagSelectDialogComponent } from '../tag-select-dialog/tag-select-dialog.component';
import { PolymorpheusComponent } from '@tinkoff/ng-polymorpheus';


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
        private injector: Injector,
        @Inject(POLYMORPHEUS_CONTEXT)
        private readonly context: TuiDialogContext<void, TagDetails>,
        private readonly tags: TagFetchService,
        private alertService: TuiAlertService,
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
        this.tags.updateByTag(this.tag).subscribe({
            next: () => {
                this.alertService.open("Tag has been updated!",
                    {
                        label: "Success!",
                        autoClose: true,
                        status: TuiNotification.Success,
                    }).subscribe()
                this.context.completeWith();
            }
        })
    }

    addParent() {
        this.dialogService.open<TagDetails>(new PolymorpheusComponent(TagSelectDialogComponent, this.injector)).subscribe(
            {
                next: (td) => {
                    this.tag.parents.push(td);
                }
            }
        );
    }

    delParent(p: TagDetails) {
        this.tag.parents.splice(this.tag.parents.indexOf(p), 1);
    }

    setName(newName: string) {
        this.tag.name = newName;
    }
}
