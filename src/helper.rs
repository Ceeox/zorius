use bson::{doc, Bson, Document};

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
