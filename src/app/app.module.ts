import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { TuiAlertModule, TuiButtonModule, TuiDataListModule, TuiDialogModule, TuiErrorModule, TuiGroupModule, TuiHintModule, TuiPrimitiveTextfieldModule, TuiRootModule, TuiScrollbarModule, TuiTextfieldControllerModule, TuiTooltipModule } from '@taiga-ui/core';
import { TuiDataListWrapperModule, TuiInputModule, TuiIslandModule, TuiSelectModule, TuiTextAreaModule, TuiTreeModule } from '@taiga-ui/kit';
import { TuiThemeNightModule, TuiModeModule } from '@taiga-ui/core';
import { TagsViewComponent } from './components/tags-view/tags-view.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { TuiTagModule } from '@taiga-ui/kit';
import { ImageViewComponent } from './components/image-view/image-view.component';
import { TuiTabsModule } from '@taiga-ui/kit';
import { FilesViewComponent } from './components/files-view/files-view.component';
import { FoldersViewComponent } from './components/folders-view/folders-view.component';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { DragDropModule } from '@angular/cdk/drag-drop';
import { TopBarComponent } from './components/top-bar/top-bar.component';
import { NewDatabaseDialogComponent } from './components/dialog/new-database-dialog/new-database-dialog.component';





@NgModule({
    declarations: [
        AppComponent,
        TagsViewComponent,
        ImageViewComponent,
        FilesViewComponent,
        FoldersViewComponent,
        TopBarComponent,
        NewDatabaseDialogComponent,
    ],
    imports: [
        //Angular built-ins
        BrowserModule,
        BrowserAnimationsModule,
        AppRoutingModule,
        ReactiveFormsModule,
        DragDropModule,
        FormsModule,
        //Tui
        TuiRootModule,
        TuiDialogModule,
        TuiAlertModule,
        TuiTreeModule,
        TuiThemeNightModule,
        TuiModeModule,
        TuiTagModule,
        TuiIslandModule,
        TuiTabsModule,
        TuiGroupModule,
        TuiInputModule,
        TuiSelectModule,
        TuiDataListModule,
        TuiDataListWrapperModule,
        TuiTextAreaModule,
        TuiPrimitiveTextfieldModule,
        TuiTextfieldControllerModule,
        TuiButtonModule,
        TuiDialogModule,
        TuiTooltipModule,
        TuiHintModule,
        TuiErrorModule,
        TuiScrollbarModule,
    ],
    providers: [],
    bootstrap: [AppComponent]
})
export class AppModule { }
