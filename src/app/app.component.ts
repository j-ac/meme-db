import { Component, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { flatMap, from, generate, mergeMap, Observable } from 'rxjs';

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
    title = 'meme-db';
    files: string[] = [];

    ngOnInit(): void {
        this.title += " initialized"
        invoke<string[]>('get_folders').then(() => {console.log("AAAAA")});
        //Create a pipeline from invoking 'get_folders'
        from(invoke<string[]>('get_folders'))
            //Flatten the array of folders to elements in the pipe
            .pipe(mergeMap((folders: string[], idx) => {
                return from(folders);
            }))
            //Query for some files in the folder
            .pipe(mergeMap((folder: string, idx) => {
                return invoke<string[]>('get_files', {folder: folder, a: 0, b: 100});
            }))
            //Flatten the array of files to elements in the pipe
            .subscribe({next: (files: string[]) => {
                files.forEach((f) => this.files.push(f));
            }})
    }
}
