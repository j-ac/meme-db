import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { TuiAlertModule, TuiDialogModule, TuiRootModule } from '@taiga-ui/core';
import { TuiIslandModule, TuiTreeModule } from '@taiga-ui/kit';
import { TuiThemeNightModule, TuiModeModule } from '@taiga-ui/core';
import { TagsViewComponent } from './components/tags-view/tags-view.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { TuiTagModule } from '@taiga-ui/kit';
import { ImageViewComponent } from './components/image-view/image-view.component';
import { TuiTabsModule } from '@taiga-ui/kit';
import { FilesViewComponent } from './components/files-view/files-view.component';
import { FoldersViewComponent } from './components/folders-view/folders-view.component';




@NgModule({
    declarations: [
        AppComponent,
        TagsViewComponent,
        ImageViewComponent,
        FilesViewComponent,
        FoldersViewComponent,
    ],
    imports: [
        BrowserModule,
        BrowserAnimationsModule,
        AppRoutingModule,
        TuiRootModule,
        TuiDialogModule,
        TuiAlertModule,
        TuiTreeModule,
        TuiThemeNightModule,
        TuiModeModule,
        TuiTagModule,
        TuiIslandModule,
        TuiTabsModule,
    ],
    providers: [],
    bootstrap: [AppComponent]
})
export class AppModule { }
