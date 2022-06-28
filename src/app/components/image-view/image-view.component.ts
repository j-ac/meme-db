import { CdkDrag, CdkDragDrop } from '@angular/cdk/drag-drop';
import { Component, ElementRef, Input, OnInit, ViewChild } from '@angular/core';
import { TuiAlertService, TuiNotification } from '@taiga-ui/core';
import { switchMap } from 'rxjs';
import { FileFetchService, FileDetails } from 'src/service/files/file-fetch.service';
import { TagDetails, TagFetchService, TagID } from 'src/service/tags/tag-fetch.service';


@Component({
    selector: 'app-image-view',
    templateUrl: './image-view.component.html',
    styleUrls: ['./image-view.component.scss']
})
export class ImageViewComponent implements OnInit {
    @ViewChild("display_image") private display_image: ElementRef<HTMLImageElement> = {} as ElementRef
    image: FileDetails | undefined;

    constructor(
        private fileFetch: FileFetchService,
        private tags: TagFetchService,
        private alert: TuiAlertService,
    ) {

    }

    ngOnInit(): void {
    }

    @Input("image") set setImage(img: FileDetails | undefined) {
        if (img === undefined)
            return;
        this.image = {
            folder: img.folder,
            id: img.id,
            name: img.name,
            tags: this.tags.getFlattened(img.tags),
        };
        this.fileFetch.getImage(this.image.id).subscribe({
            next: (image) => {
                this.display_image.nativeElement.src = image.src;
            },
            error: (err) => {
                this.alert.open(err,
                    {
                        label: "Failed to load image!",
                        status: TuiNotification.Error, autoClose: false,
                    }).subscribe();
            }
        })
    }

    public drop(event: CdkDragDrop<TagDetails>) {
        if (this.image === undefined) {
            return;
        }
        if (event.container == event.previousContainer) {
            return;
        }
        this.fileFetch.addTag(this.image.id, event.item.data.id).pipe(switchMap(
            (res) => {
                if (res.Err !== undefined) {
                    return this.alert.open(res.Err.gui_msg,
                        { label: "Failed to add tag!", status: TuiNotification.Error, autoClose: false, });
                }
                this.image?.tags.push(event.item.data);
                return this.alert.open("Tag added to",
                    { label: "Success!", status: TuiNotification.Success });
            }
        )).subscribe();
    }

    public imagePresent() {
        return this.image !== undefined;
    }
}
