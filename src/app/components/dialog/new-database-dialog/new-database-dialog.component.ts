import { Component, Inject, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';
import { TuiValidationError } from '@taiga-ui/cdk';
import { TuiDialogContext } from '@taiga-ui/core';
import { POLYMORPHEUS_CONTEXT } from '@tinkoff/ng-polymorpheus';
import { DatabaseService } from 'src/service/database/database.service';


@Component({
    selector: 'app-new-database-dialog',
    templateUrl: './new-database-dialog.component.html',
    styleUrls: ['./new-database-dialog.component.scss']
})
export class NewDatabaseDialogComponent implements OnInit {
    nameValue = ""
    goodName = false

    constructor(
        private dbService: DatabaseService,
        @Inject(POLYMORPHEUS_CONTEXT)
        private readonly context: TuiDialogContext<string, undefined>) { }

    ngOnInit(): void {
    }

    submit() {
        if (this.goodName) {
            this.context.completeWith(this.nameValue);
        }
    }

    error1 = new TuiValidationError("Name required")
    error2 = new TuiValidationError("Unique name required")
    get error(): TuiValidationError | null {
        this.goodName = false;
        if (this.nameValue.length === 0)
            return this.error1;
        if (this.dbService.by_name.has(this.nameValue))
            return this.error2;
        this.goodName = true;
        return null;
    }
}