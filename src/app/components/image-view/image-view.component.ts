import { Component, OnInit } from '@angular/core';
import { map, merge, tap } from 'rxjs';
import { FileFetchService, FileDetailsNative } from 'src/service/files/file-fetch.service';
import { FolderDetails, FolderFetchService } from 'src/service/folders/folder-fetch.service';
import { DomSanitizer, SafeUrl } from '@angular/platform-browser';
import { convertFileSrc } from '@tauri-apps/api/tauri';


@Component({
    selector: 'app-image-view',
    templateUrl: './image-view.component.html',
    styleUrls: ['./image-view.component.scss']
})
export class ImageViewComponent implements OnInit {
    image_src?: SafeUrl
    images: FileDetailsNative[] = []
    folders: FolderDetails[] = []

    constructor(
        private fileFetch: FileFetchService,
        private folderFetch: FolderFetchService,) { }

    ngOnInit(): void {
        let imChange = this.fileFetch.getFilesByFolder(0, 0, 100).pipe(map((images) => {
            this.images = images;
        }))
        let fChange = this.folderFetch.getFolders().pipe(map((folders) => {
            this.folders = folders;
        }))
        merge(imChange, fChange).subscribe({
            next: () => {
                if (this.images.length > 0 && this.folders.length > 0) {
                    const path = this.folders[0].path + this.images[0].name;
                    this.image_src = convertFileSrc(path);
                }
            }
        })
    }
}
