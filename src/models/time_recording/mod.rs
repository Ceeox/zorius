mod holidays;
mod time_record;

use std::time::Duration;

use bson::oid::ObjectId;
use chrono::{DateTime, Utc};

//use crate::time_recording::holidays::Holiday;
use crate::models::time_recording::holidays::Holiday;
use crate::models::time_recording::time_record::TimeRecord;

pub struct Record {
    user_id: ObjectId,
    time_records: Vec<TimeRecord>,

    worktime_account: Duration,
}

impl Record {
    pub fn new(user_id: ObjectId) -> Self {
        Self {
            user_id,
            time_records: vec![],

            worktime_account: Duration::new(0, 0),
        }
    }

    pub fn add_time_record(&mut self, time_record: TimeRecord) {
        self.worktime_account = time_record.get_duration().unwrap();
        self.time_records.push(time_record);
    }

    pub fn time_records<'a>(&'a self) -> &'a Vec<TimeRecord> {
        self.time_records.as_ref()
    }

    pub fn worktime_account<'a>(&'a self) -> &'a Duration {
        &self.worktime_account
    }
}
