import { Component, Injector, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';
import { TuiAlertService, TuiDialogService, TuiNotification } from '@taiga-ui/core';
import { DatabaseID, DatabaseService } from 'src/service/database/database.service';
import { PolymorpheusComponent } from '@tinkoff/ng-polymorpheus';
import { NewDatabaseDialogComponent } from '../dialog/new-database-dialog/new-database-dialog.component';
import { from, switchMap } from 'rxjs';
import { EditFoldersDialogComponent } from '../dialog/edit-folders-dialog/edit-folders-dialog.component';


@Component({
    selector: 'app-top-bar',
    templateUrl: './top-bar.component.html',
    styleUrls: ['./top-bar.component.scss']
})
export class TopBarComponent implements OnInit {
    content_string = "A name to track files, folders, and tags. Each database works independently of eachother. The name 'Built-in' is a default database that can be used."

    database_names: string[]
    formControl: FormControl

    constructor(
        private dbService: DatabaseService,
        private dialogService: TuiDialogService,
        private injector: Injector,
        private alert: TuiAlertService) {
        this.database_names = Array.from(dbService.by_id.values()).map((db) => { return db.name; });
        this.formControl = new FormControl(this.database_names[0]);
    }

    ngOnInit(): void {
    }

    onNewDB() {
        this.dialogService
            .open<string>(new PolymorpheusComponent(NewDatabaseDialogComponent, this.injector), {})
            .pipe(switchMap((name) => {
                return this.dbService.addDatabase(name);
            }))
            .subscribe({
                next: (newDB) => {
                    this.database_names.push(newDB.name);
                    this.alert.open(`"${newDB.name}" added`).subscribe();
                },
                error: (gui_msg) => {
                    this.alert
                        .open(gui_msg,
                            { label: "Failed to add new database!", status: TuiNotification.Error, autoClose: false, })
                        .subscribe();
                }
            });
    }

    onEditFolders() {
        this.dialogService
            .open<void>(
                new PolymorpheusComponent(EditFoldersDialogComponent, this.injector),
                { data: this.dbService.getUsedDatabase() }).subscribe();
    }
}