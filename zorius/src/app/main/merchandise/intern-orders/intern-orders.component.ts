import { Component, OnInit, Inject, ViewChild } from '@angular/core';

import { MatDialog, MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { Url } from 'url';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { MatPaginator } from '@angular/material/paginator';
import { MatSort } from '@angular/material/sort';
import { MatTableDataSource } from '@angular/material/table';

export interface NewInternOrder {
  merchandise_name: String;
  count: number;
  url?: Url;
  oderer: String,
  purchased_on: Date;
  article_number?: String,
  postage?: number;
  use_case?: String;
  cost: number;
}

export interface InternMerchandise {
  merchandise_id: number;
  merchandise_name: String;
  count: number;
  orderer: String,
  purchased_on: Date;
  article_number?: String,
  postage: number;
  cost: number;
  status: InternMerchandiseStatus;
  serial_number: String;
  invoice_number: number;

  useCase?: String;
  arived_on?: Date;
  url?: Url;
}

export enum InternMerchandiseStatus {
  Ordered,
  Delivered,
  Stored,
  Used,
}


@Component({
  selector: 'app-intern-orders',
  templateUrl: './intern-orders.component.html',
  styleUrls: ['./intern-orders.component.scss'],
  animations: [
    trigger('detailExpand', [
      state('collapsed', style({ height: '0px', minHeight: '0' })),
      state('expanded', style({ height: '*' })),
      transition('expanded <=> collapsed', animate('225ms cubic-bezier(0.4, 0.0, 0.2, 1)')),
    ]),
  ],
})
export class InternOrdersComponent implements OnInit {
  dataSource: MatTableDataSource<InternMerchandise>;
  columnsToDisplay = ['BANF Nummer', 'Artikelname', 'Anzahl', 'Besteller', 'Status'];
  expandedElement: InternMerchandise | null;

  internMerchandise: InternMerchandise[];
  newOrder: NewInternOrder | null;

  @ViewChild(MatPaginator, { static: true }) paginator: MatPaginator;
  @ViewChild(MatSort, { static: true }) sort: MatSort;

  constructor(public dialog: MatDialog) {
    this.dataSource = new MatTableDataSource(this.internMerchandise);
  }

  ngOnInit() {
    this.dataSource.paginator = this.paginator;
    this.dataSource.sort = this.sort;
  }

  applyFilter(event: Event) {
    const filterValue = (event.target as HTMLInputElement).value;
    this.dataSource.filter = filterValue.trim().toLowerCase();

    if (this.dataSource.paginator) {
      this.dataSource.paginator.firstPage();
    }
  }

  openIncomingGoods(): void {
    console.log("TODO: Implement incoming goods");
  }


  openNewInternOrder(): void {
    const dialogRef = this.dialog.open(NewInternOrderDialog, {
      width: '40vw',
      hasBackdrop: true,
      disableClose: true,
      data: { newInternOrder: this.newOrder }
    });

    dialogRef.afterClosed().subscribe(result => {
      console.log(result);
      // TODO: Implement POST api/v1/merchandise/new_intern_order
    });
  }

}

@Component({
  selector: 'new-intern-order-dialog',
  templateUrl: 'new-intern-order-dialog.html',
  styleUrls: ['./new-intern-order-dialog.scss'],
})
export class NewInternOrderDialog {

  constructor(
    public dialogRef: MatDialogRef<NewInternOrderDialog>,
    @Inject(MAT_DIALOG_DATA) public data: NewInternOrder) { }

  onCancelClick(): void {
    this.dialogRef.close();
  }
}
