import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { WarenComponent } from './waren.component';

describe('WarenComponent', () => {
  let component: WarenComponent;
  let fixture: ComponentFixture<WarenComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ WarenComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(WarenComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
