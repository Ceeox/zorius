import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { InternOrdersComponent } from './intern-orders.component';

describe('InternOrdersComponent', () => {
  let component: InternOrdersComponent;
  let fixture: ComponentFixture<InternOrdersComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ InternOrdersComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(InternOrdersComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
