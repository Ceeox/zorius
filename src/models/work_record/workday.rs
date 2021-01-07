use async_graphql::SimpleObject;
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::time_record::TimeRecord;

#[derive(Deserialize, Serialize, Debug, SimpleObject)]
pub struct Workday {
    date: NaiveDate,
    time_records: Vec<TimeRecord>,
    worktarget_secs: i64,
    worktime_secs: i64,
    absent_reason: Option<String>,
}

impl Workday {
    /// create a new workday and automaticly start a time record
    pub fn new(worktarget_secs: i64) -> Self {
        let tr = TimeRecord::new(0);
        let worktime_secs = worktarget_secs * -1;
        let time_records = vec![tr];
        Self {
            date: Utc::today().naive_utc(),
            worktarget_secs,
            time_records,
            worktime_secs,
            absent_reason: None,
        }
    }

    pub fn get_date(&self) -> &NaiveDate {
        &self.date
    }

    /// Starts a new timerecord.
    ///
    /// If a timerecord which hasn't ended yet is found it does nothing
    pub fn start_time_record(&mut self) {
        // check if there is a running time record
        let tr = self.time_records.iter_mut().find(|tr| tr.has_ended());
        if tr.is_some() {
            return;
        }
        let tr = TimeRecord::new((self.time_records.len() + 1) as i64);
        self.time_records.push(tr);
    }

    /// Ends the current running time record.
    ///
    /// if all time records already ended it returns the last one
    pub fn end_time_record(&mut self) {
        let tr = self.time_records.iter_mut().find(|tr| !tr.has_ended());
        if tr.is_none() {
            return;
        }

        let tr = tr.unwrap();
        tr.end();
        let dur = tr.get_duration().unwrap();
        self.worktime_secs += dur;
    }
}
