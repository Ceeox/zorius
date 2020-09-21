use uuid::Uuid;

use std::time::Duration;

pub struct Activity {
    id: Uuid,
    desc: String,
    budget: f32,
    time_budget: Duration,
    customer: Option<Uuid>,
    project: Option<Uuid>,
}
