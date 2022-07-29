import { Component, EventEmitter, Input, OnDestroy, OnInit, Output } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { TuiAlertAutoClose, TuiAlertService, TuiNotification } from '@taiga-ui/core';
import { catchError, from, map, merge, Subscription, switchMap } from 'rxjs';
import { FileDetails, FileFetchService, FileQuery } from 'src/service/files/file-fetch.service';
import { FolderDetails, FolderFetchService } from 'src/service/folders/folder-fetch.service';
import { TagFetchService } from 'src/service/tags/tag-fetch.service';

@Component({
    selector: 'app-files-view',
    templateUrl: './files-view.component.html',
    styleUrls: ['./files-view.component.scss']
})
export class FilesViewComponent implements OnInit, OnDestroy {
    private subs: Subscription[] = []
    files: FileDetails[] = []
    private internal_folders: FolderDetails[] = []
    query: FileQuery = {}
    @Output() selectedFile = new EventEmitter<FileDetails>()

    search_form = new FormGroup({
        file_name: new FormControl(''),
        folder: new FormControl(this.folders),
        tag: new FormControl(''),
    })

    constructor(
        private fileFetch: FileFetchService,
        private folderFetch: FolderFetchService,
        private tagFetch: TagFetchService,
        private alert: TuiAlertService) {
    }
    ngOnDestroy(): void {
        for (let s of this.subs) {
            s.unsubscribe();
        }
    }

    ngOnInit(): void {
        let a = this.search_form.valueChanges.pipe(map((_) => {
            // Query building
            this.query = {}
            let file_name = this.search_form.controls.file_name.value;
            let folders = this.search_form.controls.folder.value;
            let tags = this.search_form.controls.tag.value;
            if (file_name != null && file_name.length > 0) {
                this.query.names = [file_name];
            }
            if (folders != null) {
                this.query.folders_include = folders.map((folder) => {
                    return this.folderFetch.name_map.get(folder)!.id;
                })
            }
            if (tags != null && tags.length > 0) {
                let tagID = this.tagFetch.getIDByName(tags);
                if (tagID !== undefined) {
                    this.query.tags_include = [tagID];
                }
            }
            return;
        }))
        //We want to refresh files on tag changes
        let b = this.tagFetch.getTags().pipe(map((_) => {}));
        let sub1 = merge(a,b).pipe(switchMap(() => {
            return this.fileFetch.getFilesByQuery(this.query);
        }))
        .subscribe({
            next: (files) => {
                this.files = files;
            },
        });
        let sub2 = this.folderFetch.getFolders().subscribe((folders) => {
            this.internal_folders = folders;
        })
        this.subs.push(sub1, sub2);
    }

    fileClicked(id: number) {
        for (let f of this.files) {
            if (f.id == id) {
                this.selectedFile.emit(f);
            }
        }
    }

    get folders(): string[] {
        return this.internal_folders.map((f) => { return f.path });
    }
}
