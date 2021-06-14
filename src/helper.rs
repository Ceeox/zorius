use std::fmt::Display;

use bson::{doc, Bson, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SortOrder {
    ASCE = 1,
    DESC = -1,
}

pub struct AggregateBuilder {
    docs: Vec<Document>,
}

impl AggregateBuilder {
    pub fn new() -> Self {
        Self { docs: Vec::new() }
    }

    pub fn sort(mut self, fields: Vec<(&str, SortOrder)>) -> Self {
        let mut doc = Document::new();
        let mut inner = Document::new();
        for field in fields {
            match field.1 {
                SortOrder::ASCE => {
                    inner.insert(field.0, field.1);
                }
                SortOrder::DESC => {
                    inner.insert(field.0, -1);
                }
            }
        }
        doc.insert("$sort", inner);
        self.docs.push(doc);
        self
    }

    pub fn matching<T>(mut self, fields: Vec<(&str, T)>) -> Self
    where
        T: Into<Bson>,
    {
        let mut doc = Document::new();
        let mut inner = Document::new();
        for field in fields {
            inner.insert(field.0, field.1);
        }
        doc.insert("$match", inner);
        self.docs.push(doc);
        self
    }

    pub fn lookup(mut self, from: &str, local_field: &str, foreign_field: &str, _as: &str) -> Self {
        let doc = doc! { "$lookup": {
            "from": from,
            "localField": local_field,
            "foreignField": foreign_field,
            "as": _as
        }};
        self.docs.push(doc);
        self
    }

    pub fn unwind(
        mut self,
        path: &str,
        include_array_index: Option<&str>,
        preserve_null_and_empty_arrays: Option<bool>,
    ) -> Self {
        let doc = if include_array_index.is_some() {
            doc! { "$unwind": {
                "path": path,
                "includeArrayIndex": include_array_index.unwrap(),
                "preserveNullAndEmptyArrays": preserve_null_and_empty_arrays.unwrap_or(false),
            }}
        } else {
            doc! { "$unwind": {
                "path": path,
                "preserveNullAndEmptyArrays": preserve_null_and_empty_arrays.unwrap_or(false),
            }}
        };

        self.docs.push(doc);
        self
    }

    pub fn skip(mut self, skip: i64) -> Self {
        let doc = doc! { "$skip": skip };
        self.docs.push(doc);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        let doc = doc! { "$limit": limit };
        self.docs.push(doc);
        self
    }

    pub fn build(self) -> Vec<Document> {
        self.docs
    }
}

pub mod validators {
    use async_graphql::{validators::InputValueValidator, Value};
    use url::Url as CrateUrl;

    pub struct Password;

    impl InputValueValidator for Password {
        fn is_valid(&self, value: &Value) -> Result<(), String> {
            if let Value::String(s) = value {
                if s.len() >= 8 && s.len() <= 64 {
                    Ok(())
                } else {
                    Err("password must be longer than 8 chars and lower than 64 chars".to_owned())
                }
            } else {
                Ok(())
            }
        }
    }
    pub struct Url;

    impl InputValueValidator for Url {
        fn is_valid(&self, value: &Value) -> Result<(), String> {
            let mut res = Err("not a valid url".to_owned());
            if let Value::String(s) = value {
                if CrateUrl::parse(s).is_ok() {
                    res = Ok(());
                }
            }
            res
        }
    }
}
