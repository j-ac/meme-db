import { CdkDrag, CdkDragDrop } from '@angular/cdk/drag-drop';
import { Component, ElementRef, Input, OnInit, ViewChild } from '@angular/core';
import { TuiAlertService, TuiNotification } from '@taiga-ui/core';
import { switchMap } from 'rxjs';
import { FileFetchService, FileDetails, cloneFlatten } from 'src/service/files/file-fetch.service';
import { TagDetails, TagFetchService, TagID } from 'src/service/tags/tag-fetch.service';


@Component({
    selector: 'app-image-view',
    templateUrl: './image-view.component.html',
    styleUrls: ['./image-view.component.scss']
})
export class ImageViewComponent implements OnInit {
    @ViewChild("display_image") private display_image: ElementRef<HTMLImageElement> = {} as ElementRef
    image?: FileDetails
    video?: {url: string, f: FileDetails}
    text?: {val: string, f: FileDetails}

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
        this.image = cloneFlatten(this.tags, img);
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

    @Input("video") setVideo(f?: FileDetails) {
        if (f === undefined)
            return;
        this.video = {
            url: "",
            f: cloneFlatten(this.tags, f),
        }
    }

    @Input("text") setText(f?: FileDetails) {
        if (f === undefined)
            return;
        this.text = {
            val: "",
            f: cloneFlatten(this.tags, f),
        }
    }

    public drop(event: CdkDragDrop<TagDetails>) {
        if (this.image === undefined) {
            return;
        }
        if (event.container == event.previousContainer) {
            return;
        }
        this.fileFetch.addTag(this.image.id, event.item.data.id).subscribe({
            next: () => {
                this.image?.tags.push(event.item.data);
                return this.alert.open("Tag added to",
                    { label: "Success!", status: TuiNotification.Success });
            },
        });
    }

    deleted(tag: TagDetails) {
        if (this.image === undefined)
            return;
        this.fileFetch.delTag(this.image.id, tag.id).subscribe({
            next: (newFile) => {
                newFile.tags = this.tags.getFlattened(newFile.tags);
                this.image = newFile;
            }
        });

    }

    public imagePresent() {
        return this.image !== undefined;
    }

    public videoPresent() {
        return this.video !== undefined;
    }

    public textPresent() {
        return this.text !== undefined;
    }
}
