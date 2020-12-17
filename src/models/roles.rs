use bson::{oid::ObjectId, DateTime};

#[derive(PartialEq, Clone)]
pub struct Role {
    id: ObjectId,
    user_ids: Vec<ObjectId>,
    name: String,
    permissions: Vec<Permission>,
}

#[derive(PartialEq, Clone)]
pub enum Permission {
    CanReadInternMerch(bool),
    UpdateInternMerch(bool),
    DeleteInternMerch(bool),

    ReadTimeRecords(bool),
    UpdateTimeRechords(bool),
    DeleteTimeRecords(bool),
}
pub struct RoleSearch {
    ident: RoleSearchIdent,
    before: DateTime,
    after: DateTime,
    start_at: usize,
    count: usize,
}

pub enum RoleSearchIdent {
    ById(ObjectId),
    ByName(String),
    ByIds(Vec<ObjectId>),
    ByNames(Vec<String>),
}

impl Role {
    /// creates a new Role wit the given  user ids and name
    pub fn new(user_ids: Vec<ObjectId>, name: &str) -> Self {
        Self {
            id: ObjectId::new(),
            user_ids,
            name: name.to_owned(),
            permissions: vec![],
        }
    }

    /// gets the `role id`
    pub fn id(&self) -> &ObjectId {
        &self.id
    }

    /// gets the name of the Role
    pub fn name(&self) -> &str {
        &self.name
    }

    /// changes the name of the role
    pub fn update_name(&mut self, new_name: &str) {
        self.name = new_name.to_owned();
    }

    /// adds a new user
    pub fn add_user(&mut self, new_user: &ObjectId) {
        match self.user_ids.iter().position(|id| id == new_user) {
            None => self.user_ids.push(new_user.to_owned()),
            Some(_) => {}
        }
    }

    /// Searchs for the given `user id` and returns the `user id`.
    /// If the `user id` isn't in the role it returns `None`.
    pub fn remove_user(&mut self, user: &ObjectId) -> Option<ObjectId> {
        let index = self.user_ids.iter().position(|id| id == user);
        match index {
            Some(r) => Some(self.user_ids.remove(r)),
            None => None,
        }
    }

    /// adds the new `Permission` to role.
    pub fn add_permission(&mut self, new_permission: Permission) {
        self.permissions.push(new_permission)
    }

    pub fn update_permission(mut self, new_permission: Permission) {}

    pub fn remove_permission(&mut self, permission: Permission) -> Option<Permission> {
        let index = self.permissions.iter().position(|id| id == &permission);
        match index {
            Some(r) => Some(self.permissions.remove(r)),
            None => None,
        }
    }
}
