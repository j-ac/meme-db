import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NewTagDialogComponent } from './new-tag-dialog.component';

describe('NewTagDialogComponent', () => {
  let component: NewTagDialogComponent;
  let fixture: ComponentFixture<NewTagDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ NewTagDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(NewTagDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
