use std::time::Duration as StdDuration;

use async_graphql::SimpleObject;
use bson::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, SimpleObject)]
pub struct TimeRecord {
    id: i64,
    is_running: bool,
    started: DateTime,
    ended: Option<DateTime>,
    duration: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateTimeRecord {
    pub started: DateTime,
    pub ended: Option<DateTime>,
    pub duration: Option<StdDuration>,
}

impl TimeRecord {
    /// creates a new time record for the given user
    /// sets the started time here
    pub fn new(id: i64) -> Self {
        Self {
            id,
            is_running: true,
            started: Utc::now().into(),
            ended: None,
            duration: None,
        }
    }

    pub fn get_duration(&self) -> Option<i64> {
        self.duration
    }

    /// returns the ended time
    pub fn has_ended(&self) -> bool {
        !self.is_running
    }

    /// ends the time record
    /// sets ended time here and calculates the duration
    pub fn end(&mut self) {
        let ended = Utc::now();
        let started = self.started.0;
        let dur = ended - started;
        self.ended = Some(DateTime::from(ended));
        self.duration = Some(dur.num_seconds());
    }
}
