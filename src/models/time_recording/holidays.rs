use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Holiday {
    date: DateTime<Utc>,
    name: Option<String>,
}
