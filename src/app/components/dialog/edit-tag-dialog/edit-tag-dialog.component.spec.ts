import { ComponentFixture, TestBed } from '@angular/core/testing';

import { EditTagDialogComponent } from './edit-tag-dialog.component';

describe('EditTagDialogComponent', () => {
  let component: EditTagDialogComponent;
  let fixture: ComponentFixture<EditTagDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ EditTagDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(EditTagDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
