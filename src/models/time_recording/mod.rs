pub(crate) mod time_record;
pub(crate) mod workday;

use bson::oid::ObjectId;
use chrono::{Date, Duration, Utc};
use time_record::TimeRecord;

use self::workday::Workday;

use super::user::UserId;

pub type WorkAccountId = ObjectId;

pub struct WorkAccount {
    id: WorkAccountId,
    user_id: UserId,
    account: Duration,
    workdays: Vec<Workday>,
    default_work_target: Option<Duration>,
}

impl WorkAccount {
    pub fn new(user_id: UserId, default_work_target: Option<Duration>) -> Self {
        Self {
            id: WorkAccountId::new(),
            user_id,
            account: Duration::hours(8),
            workdays: vec![],
            default_work_target,
        }
    }

    pub fn id(&self) -> &WorkAccountId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    /// starts a new workday
    ///
    /// skips cration if a workday already exists for today
    pub fn start_workday(&mut self, target: Option<Duration>) {
        let today = Utc::today();

        if self.find_workday_mut(&today).is_some() {
            return;
        }

        let target = match target {
            Some(r) => r,
            None => match self.default_work_target {
                Some(r) => r,
                None => Duration::zero(),
            },
        };

        let wd = Workday::new(target);
        self.workdays.push(wd);
    }

    pub fn pause(&mut self) {
        let today = Utc::today();
        let wd = match self.find_workday_mut(&today) {
            Some(r) => r,
            None => return,
        };
        wd.end_time_record();
    }

    pub fn resume_work(&mut self) {
        let today = Utc::today();
        let wd = self.find_workday_mut(&today);

        match wd {
            Some(r) => r.start_time_record(),
            None => {}
        }
    }

    fn find_workday_mut(&mut self, date: &Date<Utc>) -> Option<&mut Workday> {
        self.workdays.iter_mut().find(|item| item.date().eq(date))
    }
}
