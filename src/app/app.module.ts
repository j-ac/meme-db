import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { TuiAlertModule, TuiButtonModule, TuiColorModule, TuiDataListModule, TuiDialogModule, TuiDropdownControllerModule, TuiErrorModule, TuiGroupModule, TuiHintModule, TuiPrimitiveTextfieldModule, TuiRootModule, TuiScrollbarModule, TuiSvgModule, TuiTextfieldControllerModule, TuiTooltipModule } from '@taiga-ui/core';
import { TuiDataListWrapperModule, TuiInputModule, TuiIslandModule, TuiMarkerIconModule, TuiMultiSelectModule, TuiSelectModule, TuiTextAreaModule, TuiTreeModule } from '@taiga-ui/kit';
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
import { EditFoldersDialogComponent } from './components/dialog/edit-folders-dialog/edit-folders-dialog.component';
import { NewTagDialogComponent } from './components/dialog/new-tag-dialog/new-tag-dialog.component';
import { EditTagDialogComponent } from './components/dialog/edit-tag-dialog/edit-tag-dialog.component';
import { TuiColorPickerModule, TuiEditorModule, TuiEditorSocketModule, TuiPaletteModule, TuiInputColorModule } from '@taiga-ui/addon-editor'



@NgModule({
    declarations: [
        AppComponent,
        TagsViewComponent,
        ImageViewComponent,
        FilesViewComponent,
        FoldersViewComponent,
        TopBarComponent,
        NewDatabaseDialogComponent,
        EditFoldersDialogComponent,
        NewTagDialogComponent,
        EditTagDialogComponent,
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
        TuiMarkerIconModule,
        TuiSvgModule,
        TuiMultiSelectModule,
        TuiDropdownControllerModule,
        TuiColorModule,
        TuiColorPickerModule,
        TuiEditorModule,
        TuiEditorSocketModule,
        TuiPaletteModule,
        TuiInputColorModule,
    ],
    providers: [],
    bootstrap: [AppComponent]
})
export class AppModule { }