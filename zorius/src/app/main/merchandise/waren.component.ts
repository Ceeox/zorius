import { Component, OnInit } from '@angular/core';
import { Observable, Observer } from 'rxjs';

export interface ExampleTab {
  label: string;
  content: string;
}

@Component({
  selector: 'app-waren',
  templateUrl: './waren.component.html',
  styleUrls: ['./waren.component.scss']
})
export class WarenComponent implements OnInit {

  asyncTabs: Observable<ExampleTab[]>;

  constructor() {
    this.asyncTabs = new Observable((observer: Observer<ExampleTab[]>) => {
      setTimeout(() => {
        observer.next([
          { label: 'Interner Wareneingang', content: 'Content 1' },
          { label: 'Externer Wareneingang', content: 'Content 2' },
          { label: 'Lager', content: 'Content 3' },
        ]);
      }, 1000);
    });
  }

  ngOnInit(): void {
  }

}
