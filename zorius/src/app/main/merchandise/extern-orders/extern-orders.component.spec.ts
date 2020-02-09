import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ExternOrdersComponent } from './extern-orders.component';

describe('ExternOrdersComponent', () => {
  let component: ExternOrdersComponent;
  let fixture: ComponentFixture<ExternOrdersComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ExternOrdersComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ExternOrdersComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
