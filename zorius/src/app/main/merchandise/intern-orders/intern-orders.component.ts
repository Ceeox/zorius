import { Component, OnInit, Inject, ViewChild, Injectable } from '@angular/core';
import { MatDialog, MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { MatPaginator } from '@angular/material/paginator';
import { MatSort } from '@angular/material/sort';
import { MatTableDataSource } from '@angular/material/table';
import { InternOrderTableDataGQL, NewInternOrderGQL } from './graphql.module';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

export interface NewInternOrder {
  merchandiseName: String;
  count: number;
  url?: String;
  purchasedOn: Date;
  articleNumber?: String,
  postage?: number;
  useCase: String;
  cost: number;
}

// export interface InternMerchandise {
//   merchandise_id: number;
//   merchandise_name: String;
//   count: number;
//   orderer: String,
//   purchased_on: Date;
//   article_number?: String,
//   postage: number;
//   cost: number;
//   status: InternMerchandiseStatus;
//   serial_number: String;
//   invoice_number: number;

//   useCase?: String;
//   arived_on?: Date;
//   url?: String;
// }

export interface InternOrderTableData {
  merchandiseName: String;
  merchandiseId: number;
  cost: number;
  ordererId: String;
  status: InternMerchandiseStatus;
  purchasedOn: Date;
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


  dataSource: MatTableDataSource<InternOrderTableData>;
  columnsToDisplay = ['count', 'merchandiseName', 'cost', 'ordererId', 'merchandiseId', 'status'];
  expandedElement: InternOrderTableData | null;

  tableData: InternOrderTableData[];

  @ViewChild(MatPaginator, { static: true }) paginator: MatPaginator;
  @ViewChild(MatSort, { static: true }) sort: MatSort;

  constructor(public dialog: MatDialog,
    private tableDataGQL: InternOrderTableDataGQL,
    private newInternOrderGQL: NewInternOrderGQL) {
  }

  ngOnInit() {
    this.loadTableData();
  }

  loadTableData() {
    this.getTableData().subscribe(data => {
      this.tableData = data;
      this.dataSource = new MatTableDataSource(this.tableData);
      this.dataSource.paginator = this.paginator;
      this.dataSource.sort = this.sort;
    });
  }

  applyFilter(event: Event) {
    const filterValue = (event.target as HTMLInputElement).value;
    this.dataSource.filter = filterValue.trim().toLowerCase();

    if (this.dataSource.paginator) {
      this.dataSource.paginator.firstPage();
    }
  }

  getTableData(): Observable<InternOrderTableData[]> {
    return this.tableDataGQL.watch({
      first: 10
    }, {
      fetchPolicy: 'network-only'
    }).valueChanges
      .pipe(
        map(result => result.data.tableData)
      );
  }

  submitNewInternOrder(newOder: NewInternOrder) {
    this.newInternOrderGQL
      .mutate({
        newInternOrder: newOder,
      })
      .subscribe();
    this.loadTableData();
  }

  openIncomingGoods(): void {
    console.log("TODO: Implement incoming goods");
  }

  openNewInternOrder(): void {
    const newOrder = undefined;
    const dialogRef = this.dialog.open(NewInternOrderDialog, {
      width: '40vw',
      hasBackdrop: true,
      disableClose: true,
      data: { newInternOrder: newOrder }
    });

    dialogRef.afterClosed().subscribe(result => {
      if (result === undefined)
        return;
      result.ordererId = "83215719-3835-4726-b356-47c33e4c74a2";
      result.purchasedOn = Date.now().toString();
      this.submitNewInternOrder(result);
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
