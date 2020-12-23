use chrono::{Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::time_record::TimeRecord;

#[derive(Deserialize, Serialize, Debug)]
pub enum AbsentReason {
    SickLeave,
    Holiday,
    Vacation,
    Other(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Workday {
    date: NaiveDate,
    target: String,
    time_records: Vec<TimeRecord>,
    worktime_account: String,
    absent_reason: Option<AbsentReason>,
}

impl Workday {
    /// create a new workday and automaticly start a time record
    pub fn new(target: Duration) -> Self {
        let tr = TimeRecord::new(0);
        let time_records = vec![tr];
        Self {
            date: NaiveDate::from(Utc::today().into()),
            target: target.to_string(),
            time_records,
            worktime_account: Duration::zero().to_string(),
            absent_reason: None,
        }
    }

    pub fn time_records_mut(&mut self) -> &Vec<TimeRecord> {
        &self.time_records
    }

    pub fn add_worktime_account(&mut self, dur: &Duration) {
        let wt_account: Duration = self.worktime_account.into();
        match wt_account.checked_add(dur) {
            Some(r) => self.worktime_account = r.to_string(),
            None => {}
        }
    }

    /// returns the date for the Workday
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    /// Starts a new timerecord.
    ///
    /// If a timerecord which hasn't ended yet is found it does nothing
    pub fn start_time_record(&mut self) {
        // if time_record has ended it returns Some(r), then return and do nothing
        match self.time_records.iter().last() {
            Some(r) if r.ended() => {}
            _ => return,
        }

        let next_id: usize = match self.time_records.last() {
            None => 0,
            Some(r) => r.id().saturating_add(1),
        };

        let tr = TimeRecord::new(next_id);
        self.time_records.push(tr);
    }

    /// Ends the current running time record.
    ///
    /// if all time records already ended it returns the last one
    pub fn end_time_record(&mut self) {
        match self.time_records.last().map(|tr| tr.ended()) {
            Some(r) if r == true => {}
            _ => return,
        }

        let opt_tr = self.time_records.iter_mut().find(|tr| !tr.ended());
        match opt_tr {
            Some(r) => {
                r.end();
            }
            None => {}
        }
    }
}
