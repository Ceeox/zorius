import { Injectable } from '@angular/core';
import { Query, Mutation } from 'apollo-angular';
import gql from 'graphql-tag';

import { InternOrderTableData } from './intern-orders.component';


export interface Response {
  tableData: InternOrderTableData[];
}

@Injectable({
  providedIn: 'root',
})
export class InternOrderTableDataGQL extends Query<Response> {
  document = gql`
    query TableData {
      tableData {
        count
        merchandiseName
        merchandiseId
        cost
        ordererId
        status
      }
    }
  `;
}

@Injectable({
  providedIn: 'root',
})
export class NewInternOrderGQL extends Mutation {
  document = gql`
    mutation new_intern_order($newInternOrder: NewInternOrder!) {
      newInternOrder(newInternOrder: $newInternOrder) {
        Id
      }
    }`;
}