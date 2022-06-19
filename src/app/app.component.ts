import { Component, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { from, generate, mergeMap, Observable } from 'rxjs';
import { FolderFetchService } from 'src/service/folders/folder-fetch.service';
@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
    title = 'meme-db';
    files: string[] = [];

    constructor (private fs: FolderFetchService) {
    }

    ngOnInit(): void {
        this.title += " initialized"
    }
}
