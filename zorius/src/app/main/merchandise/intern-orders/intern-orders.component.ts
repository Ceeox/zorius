import { Component, OnInit, Inject, ViewChild, Injectable } from '@angular/core';

import { MatDialog, MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { Url } from 'url';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { MatPaginator } from '@angular/material/paginator';
import { MatSort } from '@angular/material/sort';
import { MatTableDataSource } from '@angular/material/table';

import { Apollo } from 'apollo-angular';
import gql from 'graphql-tag';
import { url } from 'inspector';

export interface NewInternOrder {
  merchandise_name: String;
  count: number;
  url?: String;
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
  url?: String;
}

export enum InternMerchandiseStatus {
  Ordered,
  Delivered,
  Stored,
  Used,
}


@Injectable()
class InternOrderService {
  mutNewInternOrder = gql`mutation new_intern_order {
    newInternOrder(newInternOrder: $newInternOrder) {
      Id
    }
  }`;

  constructor(private apollo: Apollo) {}

  submitNewInternOrder(order: NewInternOrder) {
    return this.apollo.mutate({
      mutation: this.mutNewInternOrder,
      variables: {
        newInternOrder: order
      }
    });
  }
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

  constructor(public dialog: MatDialog, private internOrderService: InternOrderService) {
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

  submitNewInternOrder() {
    var order: NewInternOrder =  {
      merchandise_name: "Test",
      count: 42,
      url: "sndfjdshouhusiof",
      oderer: "mw",
      purchased_on: new Date(),
      article_number: "aksjk3",
      postage: 2.345,
      use_case: "slkjdkldjfk",
      cost: 345.34,
    };

    this.internOrderService.submitNewInternOrder(order).subscribe(({data}) => {
      console.log('go data: ' + data);
      return data;
    }, (error) => {
      console.log('there was an error: ' + error);
      return;
    });
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
      // this.submitNewInternOrder();
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
