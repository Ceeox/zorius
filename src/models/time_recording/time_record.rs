use bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use std::time::Duration;

use crate::models::user::UserId;

pub type TimeRecordId = ObjectId;
#[derive(Deserialize, Serialize, Debug)]
pub struct TimeRecord {
    id: TimeRecordId,
    user_id: TimeRecordId,
    started: DateTime,
    ended: Option<DateTime>,
    duration: Option<Duration>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateTimeRecord {
    pub id: TimeRecordId,
    pub started: DateTime,
    pub ended: Option<DateTime>,
}

impl TimeRecord {
    /// creates a new time record for the given user
    /// sets the started time here
    pub fn new(user_id: UserId) -> Self {
        Self {
            id: ObjectId::new(),
            user_id,
            started: Utc::now().into(),
            ended: None,
            duration: None,
        }
    }

    pub fn id(&self) -> &TimeRecordId {
        &self.id
    }

    /// returns the time were the record started
    pub fn started(&self) -> DateTime {
        self.started.clone()
    }

    /// returns the ended time
    /// None if the record hasn't conculded yet
    pub fn ended(&self) -> Option<DateTime> {
        match self.ended {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }

    /// gets the duration of the finished record
    /// None if the record hasn't conculded yet
    pub fn get_duration(&self) -> Option<Duration> {
        match self.duration {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }

    /// ends the time record
    /// sets ended time here and calculates the duration
    pub fn end(&mut self) {
        self.ended = Some(Utc::now().into());
        let dur_ms =
            self.ended.unwrap().timestamp_millis() as u64 - self.started.timestamp_millis() as u64;
        self.duration = Some(Duration::from_millis(dur_ms))
    }
}
