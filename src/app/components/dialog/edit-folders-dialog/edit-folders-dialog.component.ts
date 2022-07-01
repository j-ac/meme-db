import { Component, Inject, OnInit } from '@angular/core';
import { TuiAlertService, TuiDialogContext, TuiNotification } from '@taiga-ui/core';
import { POLYMORPHEUS_CONTEXT } from '@tinkoff/ng-polymorpheus';
import { DatabaseDetails } from 'src/service/database/database.service';
import { FolderDetails, FolderFetchService } from 'src/service/folders/folder-fetch.service';
import { open } from '@tauri-apps/api/dialog';
import { from, switchMap } from 'rxjs';
import { homeDir } from '@tauri-apps/api/path';


@Component({
    selector: 'app-edit-folders-dialog',
    templateUrl: './edit-folders-dialog.component.html',
    styleUrls: ['./edit-folders-dialog.component.scss']
})
export class EditFoldersDialogComponent implements OnInit {
    goodFolder = false
    folders: FolderDetails[] = [];

    constructor(
        private folderFetchService: FolderFetchService,
        private alert: TuiAlertService,
        @Inject(POLYMORPHEUS_CONTEXT)
        private readonly context: TuiDialogContext<void, DatabaseDetails>) {
        context.data
    }

    get db_name(): string {
        return this.context.data.name;
    }

    ngOnInit(): void {
        this.folderFetchService.getFolders().subscribe((folders) => {
            this.folders = folders;
        })
    }

    addFolder() {
        from(homeDir()).pipe(switchMap((home) => {
            return from(open({
                directory: true,
                multiple: true,
                defaultPath: home,
            }))
        })).pipe(switchMap((folders) => {
            if (typeof folders == 'string') {
                return [folders];
            } else if (Array.isArray(folders)) {
                return folders;
            }
            return [];
        })).pipe(switchMap((folder) => {
            return this.folderFetchService.addFolder(folder);
        })).subscribe({
            next: (fd) => {
                this.alert.open(`${fd.path} added!`, { status: TuiNotification.Success }).subscribe();
            },
            error: (gui_msg: string) => {
                this.alert.open(gui_msg,
                    {
                        label: "Failed to add folder.",
                        status: TuiNotification.Error,
                        autoClose: false,
                    }).subscribe();
            }
        })
    }

    submit() {
        this.context.completeWith();
    }
}
