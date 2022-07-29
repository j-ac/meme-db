import { Component, OnInit } from '@angular/core';
import { FolderDetails, FolderFetchService } from 'src/service/folders/folder-fetch.service';

@Component({
    selector: 'app-folders-view',
    templateUrl: './folders-view.component.html',
    styleUrls: ['./folders-view.component.scss']
})
export class FoldersViewComponent implements OnInit {
    folders: FolderDetails[] = [];
    constructor(private folderFetch: FolderFetchService) { }

    ngOnInit(): void {
        this.folderFetch.getFolders().subscribe({next: (folders => {
            this.folders = Array.from(folders);
            this.folders.sort((a,b) => {return a.id - b.id});
        })})
    }
}
