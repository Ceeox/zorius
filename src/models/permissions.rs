use bson::oid::ObjectId;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(GraphQLInputObject, Deserialize, Serialize, Debug)]
pub struct UserPermissionUpdate {
    pub read_intern_merch: Option<bool>,
    pub update_intern_merch: Option<bool>,

    pub read_time_records: Option<bool>,
    pub update_time_records: Option<bool>,
}

#[derive(GraphQLObject, Deserialize, Serialize, Debug, Default)]
pub struct GroupPermissions {
    id: ObjectId,
    name: String,
    permited_user_ids: Vec<ObjectId>,
    pub permissions: Permissions,
}

impl GroupPermissions {
    pub fn new(name: &str, members: Vec<ObjectId>) -> Result<Self, String> {
        match name {
            "admin" => {
                if !members.is_empty() {
                    return Ok(GroupPermissions::set_admin_group_permissions(members));
                } else {
                    return Err(String::from(
                        "The permission group \"admin\" should at least have one member!",
                    ));
                }
            }
            _ => {
                let permissions = Permissions::default();

                Ok(Self {
                    id: ObjectId::new(),
                    name: name.to_owned(),
                    permited_user_ids: members,
                    permissions,
                })
            }
        }
    }

    fn set_admin_group_permissions(members: Vec<ObjectId>) -> Self {
        Self {
            id: ObjectId::new(),
            name: "admin".to_owned(),
            permited_user_ids: members,
            permissions: Permissions {
                read_users: true,
                update_users: true,

                read_intern_merch: true,
                update_intern_merch: true,

                read_time_records: true,
                update_time_records: true,
            },
        }
    }

    pub fn remove_member(&mut self, rm_id: &ObjectId) {
        let pos = self.permited_user_ids.iter().position(|id| id.eq(&rm_id));
        match pos {
            Some(r) => {
                let _ = self.permited_user_ids.remove(r);
            }
            None => {}
        }
    }
}

#[derive(GraphQLObject, Deserialize, Serialize, Debug)]
pub struct Permissions {
    pub read_users: bool,
    pub update_users: bool,

    pub read_intern_merch: bool,
    pub update_intern_merch: bool,

    pub read_time_records: bool,
    pub update_time_records: bool,
}

impl Permissions {
    pub fn update(&mut self, perm: PermissionsUpdate) {
        self.read_users = perm.read_users.unwrap_or(self.read_users);
        self.update_users = perm.update_users.unwrap_or(self.update_users);

        self.read_intern_merch = perm.read_intern_merch.unwrap_or(self.read_intern_merch);
        self.update_intern_merch = perm.update_intern_merch.unwrap_or(self.update_intern_merch);

        self.read_time_records = perm.read_time_records.unwrap_or(self.read_time_records);
        self.read_time_records = perm.read_time_records.unwrap_or(self.read_time_records);
    }

    pub fn user_update(&mut self, perm: UserPermissionUpdate) {
        self.read_intern_merch = perm.read_intern_merch.unwrap_or(self.read_intern_merch);
        self.update_intern_merch = perm.update_intern_merch.unwrap_or(self.update_intern_merch);

        self.read_time_records = perm.read_time_records.unwrap_or(self.read_time_records);
        self.read_time_records = perm.read_time_records.unwrap_or(self.read_time_records);
    }
}

#[derive(GraphQLInputObject, Deserialize, Serialize, Debug)]
pub struct PermissionsUpdate {
    pub read_users: Option<bool>,
    pub update_users: Option<bool>,

    pub read_intern_merch: Option<bool>,
    pub update_intern_merch: Option<bool>,

    pub read_time_records: Option<bool>,
    pub update_time_records: Option<bool>,
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            read_users: true,
            update_users: false,

            read_intern_merch: true,
            update_intern_merch: false,

            read_time_records: false,
            update_time_records: false,
        }
    }
}

impl Into<Permissions> for PermissionsUpdate {
    fn into(self) -> Permissions {
        Permissions {
            read_users: self.read_users.unwrap_or(true),
            update_users: self.update_users.unwrap_or(false),

            read_intern_merch: self.read_intern_merch.unwrap_or(true),
            update_intern_merch: self.update_intern_merch.unwrap_or(false),

            read_time_records: self.read_time_records.unwrap_or(false),
            update_time_records: self.update_time_records.unwrap_or(false),
        }
    }
}
