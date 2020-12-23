use std::time::Duration as StdDuration;

use bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use log::error;
use serde::{Deserialize, Serialize};

pub type TimeRecordId = ObjectId;
#[derive(Deserialize, Serialize, Debug)]
pub struct TimeRecord {
    id: usize,
    started: DateTime,
    ended: Option<DateTime>,
    duration: Option<StdDuration>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateTimeRecord {
    pub started: Option<DateTime>,
    pub ended: Option<DateTime>,
    pub duration: Option<StdDuration>,
}

impl TimeRecord {
    /// creates a new time record for the given user
    /// sets the started time here
    pub fn new(id: usize) -> Self {
        Self {
            id,
            started: Utc::now().into(),
            ended: None,
            duration: None,
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }

    /// returns the time were the record started
    pub fn started(&self) -> DateTime {
        self.started.clone()
    }

    /// returns the ended time
    /// None if the record hasn't conculded yet
    pub fn ended(&self) -> bool {
        match self.ended {
            Some(_) => false,
            None => true,
        }
    }

    /// gets the duration of the finished record
    /// None if the record hasn't conculded yet
    pub fn duration(&self) -> Option<&StdDuration> {
        match self.duration {
            Some(ref r) => Some(r),
            None => None,
        }
    }

    /// ends the time record
    /// sets ended time here and calculates the duration
    pub fn end(&mut self) {
        let ended = Utc::now();
        let started = self.started.0;
        let dur = ended - started;
        self.ended = Some(DateTime::from(ended));
        match dur.to_std() {
            Ok(r) => self.duration = Some(r),
            Err(e) => {
                error!("Time Record Duration was < 0\n{:?}", e);
                self.duration = None;
            }
        }
    }
}
