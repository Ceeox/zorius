use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ProjectId = Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct DBProject {
    pub id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct NewProject {
    pub name: String,
    pub description: Option<String>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct UpdateProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl DBProject {
    pub fn new(new: NewProject) -> Self {
        Self {
            id: ProjectId::new_v4(),
            name: new.name,
            description: new.description,
            note: new.note,
        }
    }

    pub fn get_id(&self) -> &ProjectId {
        &self.id
    }
}
