import { Component, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { from, generate, mergeMap, Observable } from 'rxjs';
import { FileDetails } from 'src/service/files/file-fetch.service';
import { FolderFetchService } from 'src/service/folders/folder-fetch.service';
@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
    title = 'meme-db'
    selectedFile: FileDetails | undefined

    constructor (private fs: FolderFetchService) {
    }

    ngOnInit(): void {
        this.title += " initialized"
    }

    fileSelectEvent(file: FileDetails) {
        this.selectedFile = file;
    }
}
