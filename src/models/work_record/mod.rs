pub(crate) mod time_record;
pub(crate) mod workday;

use async_graphql::SimpleObject;
use bson::oid::ObjectId;
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::user::UserId;

use self::workday::Workday;

pub type WorkAccountId = ObjectId;

#[derive(Deserialize, Serialize, Debug, SimpleObject)]
pub struct WorkAccount {
    #[serde(rename = "_id")]
    id: WorkAccountId,
    user_id: UserId,
    worktime_secs: i64,
    workdays: Vec<Workday>,
    default_work_target: Option<i64>,
    disabled: bool,
}

impl WorkAccount {
    pub fn new(user_id: UserId, default_work_target: Option<i64>) -> Self {
        Self {
            id: WorkAccountId::new(),
            user_id,
            worktime_secs: 0,
            workdays: vec![],
            default_work_target,
            disabled: false,
        }
    }

    pub fn get_id(&self) -> &WorkAccountId {
        &self.id
    }

    /// starts a new workday
    ///
    /// skips cration if a workday already exists for today
    pub fn start_workday(&mut self) {
        let today = Utc::today().naive_utc();

        if self.find_workday_mut(&today).is_some() {
            return;
        }

        let target = match self.default_work_target {
            Some(r) => r,
            None => 0,
        };

        let wd = Workday::new(target);
        self.workdays.push(wd);
    }

    pub fn pause(&mut self) {
        let today = Utc::today().naive_utc();
        let wd = match self.find_workday_mut(&today) {
            Some(r) => r,
            None => return,
        };
        wd.end_time_record();
    }

    pub fn resume_work(&mut self) {
        let today = Utc::today().naive_utc();
        let wd = self.find_workday_mut(&today);

        match wd {
            Some(r) => r.start_time_record(),
            None => {}
        }
    }

    fn find_workday_mut(&mut self, date: &NaiveDate) -> Option<&mut Workday> {
        self.workdays
            .iter_mut()
            .find(|item| item.get_date().eq(date))
    }
}
