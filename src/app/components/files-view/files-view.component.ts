import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { from, switchMap } from 'rxjs';
import { FileDetails, FileFetchService } from 'src/service/files/file-fetch.service';
import { FolderFetchService } from 'src/service/folders/folder-fetch.service';
import { TagFetchService } from 'src/service/tags/tag-fetch.service';

@Component({
    selector: 'app-files-view',
    templateUrl: './files-view.component.html',
    styleUrls: ['./files-view.component.scss']
})
export class FilesViewComponent implements OnInit {
    files: FileDetails[] = []
    @Output() selectedFile = new EventEmitter<FileDetails>();

    search_form = new FormGroup({
        file_name: new FormControl(''),
        folder: new FormControl(''),
        tag: new FormControl(''),
        range_a: new FormControl(1),
        range_b: new FormControl(100),
    })

    constructor(
        private fileFetch: FileFetchService,
        private folderFetch: FolderFetchService,
        private tagFetch: TagFetchService) {
    }

    ngOnInit(): void {
        this.search_form.valueChanges.pipe(switchMap((_) => {
            let range_a = this.search_form.value.range_a || 0;
            let range_b = this.search_form.value.range_a || (range_a + 100);
            if (this.search_form.value.folder && this.search_form.value.folder !== "") {
                let folder = this.folderFetch.getIDByName(this.search_form.value.folder);
                if (folder !== undefined)
                    return this.fileFetch.getFilesByFolder(folder, range_a, range_b);
            }
            if (this.search_form.value.tag && this.search_form.value.tag !== "") {
                let tag = this.tagFetch.getIDByName(this.search_form.value.tag);
                if (tag !== undefined) {
                    return this.fileFetch.getFilesByTag(tag, range_a, range_b);
                }
            }
            return from([null]);
        })).subscribe((files) => {
            if (files === null) {
                this.files = [];
                return;
            }
            this.files = files;
        })
    }

    fileClicked(id: number) {
        for (let f of this.files) {
            if (f.id == id) {
                this.selectedFile.emit(f);
            }
        }
    }
}
