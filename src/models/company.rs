use bson::{oid::ObjectId, DateTime};
use chrono::Utc;

use super::{user::UserId, work_report::customer::CustomerId};

pub struct NewCompany {
    name: String,
    owner: UserId,
    members: Vec<UserId>,
    note: Option<String>,
}

pub type CompanyId = ObjectId;

pub struct Company {
    id: CompanyId,
    name: String,
    members: Vec<UserId>,
    customers: Vec<CustomerId>,
    note: Option<String>,
    owner: UserId,
    updated: DateTime,
    created: DateTime,
}

impl Into<Company> for NewCompany {
    fn into(self) -> Company {
        Company {
            id: CompanyId::new(),
            name: self.name,
            members: self.members,
            customers: vec![],
            note: self.note,
            owner: self.owner,
            updated: Utc::now().into(),
            created: Utc::now().into(),
        }
    }
}
