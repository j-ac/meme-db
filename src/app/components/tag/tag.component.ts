import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { TagDetails } from 'src/service/tags/tag-fetch.service';

@Component({
    selector: 'app-tag',
    templateUrl: './tag.component.html',
    styleUrls: ['./tag.component.scss']
})
export class TagComponent implements OnInit {
    @Input()
    tag: TagDetails = {id: -1, name: "UNDEFINED", parents: []}
    @Input()
    edit: boolean = false
    @Input()
    hover: boolean = true
    @Input()
    size: "s" | "m" | "l" = 'm'

    @Output()
    tag_clicked = new EventEmitter<TagDetails>()
    @Output()
    tag_edit = new EventEmitter<string>()

    constructor() { }
    ngOnInit(): void {
    }

    onClick() {
        this.tag_clicked.emit(this.tag);
    }

    handleTagEdited(newName: string) {
        this.tag_edit.emit(newName)
    }

}
