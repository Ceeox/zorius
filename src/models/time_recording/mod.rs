mod activity;
mod customer;
mod holidays;
mod time_record;

use std::time::Duration;

use chrono::{DateTime, Utc};
use uuid::Uuid;

//use crate::time_recording::holidays::Holiday;
use crate::models::time_recording::holidays::Holiday;
use crate::models::time_recording::time_record::TimeRecord;

pub struct Records {
    user_id: Uuid,
    date: DateTime<Utc>,
    absent_reson: Option<AbsentReason>,
    time_records: Vec<TimeRecord>,

    worktime_account: Duration,
}

#[derive(Clone)]
pub enum AbsentReason {
    Holiday(Holiday),
    Vacation,
    SickLeave,
    Other(String),
}

impl Records {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            date: Utc::now(),
            absent_reson: None,
            time_records: vec![],

            worktime_account: Duration::new(0, 0),
        }
    }

    pub fn set_absent_reason(&mut self, reason: AbsentReason) {
        self.absent_reson = Some(reason);
    }

    pub fn absent_reason(&self) -> Option<AbsentReason> {
        self.absent_reson.clone()
    }

    pub fn add_time_record(&mut self, time_record: TimeRecord) {
        self.worktime_account = time_record.get_duration().unwrap();
        self.time_records.push(time_record);
    }

    pub fn time_records(&self) -> &Vec<TimeRecord> {
        self.time_records.as_ref()
    }

    pub fn worktime_account(&self) -> &Duration {
        &self.worktime_account
    }
}
