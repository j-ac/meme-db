import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { TuiAlertModule, TuiDialogModule, TuiRootModule } from '@taiga-ui/core';
import { TuiTreeModule } from '@taiga-ui/kit';
import { TuiThemeNightModule, TuiModeModule } from '@taiga-ui/core';
import { TagsViewComponent } from './components/tags-view/tags-view.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { TuiTagModule } from '@taiga-ui/kit';



@NgModule({
    declarations: [
        AppComponent,
        TagsViewComponent
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
    ],
    providers: [],
    bootstrap: [AppComponent]
})
export class AppModule { }
