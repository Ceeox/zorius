use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;

pub struct TimeRecord {
    started: DateTime<Utc>,
    ended: Option<DateTime<Utc>>,
    duration: Option<Duration>,
}

impl TimeRecord {
    /// creates a new time record for the given user
    /// sets the started time here
    pub fn new(user_id: Uuid) -> Self {
        Self {
            started: Utc::now(),
            ended: None,
            duration: None,
        }
    }

    /// returns the time were the record started
    pub fn started(&self) -> DateTime<Utc> {
        self.started.clone()
    }

    /// returns the ended time
    /// None if the record hasn't conculded yet
    pub fn ended(&self) -> Option<DateTime<Utc>> {
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
        self.ended = Some(Utc::now());
        let dur_ms =
            self.ended.unwrap().timestamp_millis() as u64 - self.started.timestamp_millis() as u64;
        self.duration = Some(Duration::from_millis(dur_ms))
    }
}
