import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { FilesViewComponent } from './components/files-view/files-view.component';
import { FoldersViewComponent } from './components/folders-view/folders-view.component';
import { ImageViewComponent } from './components/image-view/image-view.component';
import { TagsViewComponent } from './components/tags-view/tags-view.component';

const routes: Routes = [
    {path: "", component: TagsViewComponent, outlet: "view1"},
    {path: "image", component: ImageViewComponent, outlet: "view1"},
    {path: "tags", component: TagsViewComponent, outlet: "view1"},
    {path: "files", component: FilesViewComponent, outlet: "view1"},
    {path: "folders", component: FoldersViewComponent, outlet: "view1"},
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
