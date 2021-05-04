use bson::{doc, to_bson, Bson, Document};
use serde::Serialize;

pub trait Update<U, T> {
    fn update(u: U) -> T;
}

pub trait NullKeyRemover {
    fn remove_null_keys(self) -> Vec<Document>;
}

impl NullKeyRemover for Document {
    fn remove_null_keys(self) -> Vec<Document> {
        let mut res = vec![];
        for item in self.into_iter() {
            if item.1 != Bson::Null {
                res.push(doc! { "$set": { item.0.as_str(): item.1 }});
            }
        }
        println!("{:?}", res);
        res
    }
}

pub struct AggregateBuilder {
    docs: Vec<Document>,
}

impl AggregateBuilder {
    pub fn new() -> Self {
        Self { docs: Vec::new() }
    }

    pub fn matching<T>(mut self, fields: (&str, T)) -> Self
    where
        T: Into<Bson>,
    {
        let mut doc = Document::new();
        let mut inner = Document::new();
        inner.insert(fields.0, fields.1);
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
        let doc = doc! { "$skip": limit };
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
                if let Ok(_) = CrateUrl::parse(s) {
                    res = Ok(());
                }
            }
            res
        }
    }
}
