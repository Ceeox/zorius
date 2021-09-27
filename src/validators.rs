use async_graphql::{validators::InputValueValidator, Value};
use url::Url as CrateUrl;
use uuid::Uuid as CrateUuid;

pub struct Password;
const MIN_PW_LEN: usize = 8;
const MAX_PW_LEN: usize = 255;

impl InputValueValidator for Password {
    fn is_valid(&self, value: &Value) -> Result<(), String> {
        if let Value::String(s) = value {
            if s.len() >= MIN_PW_LEN && s.len() <= MAX_PW_LEN {
                Ok(())
            } else {
                Err("password must be longer than 8 chars and lower than 255 chars".to_owned())
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
        match value {
            Value::String(s) if CrateUrl::parse(s).is_ok() => res = Ok(()),
            Value::Null => res = Ok(()),
            _ => {}
        }
        res
    }
}

pub struct Uuid;

impl InputValueValidator for Uuid {
    fn is_valid(&self, value: &Value) -> Result<(), String> {
        let mut res = Err("not a valid uuid".to_owned());
        if let Value::String(s) = value {
            if CrateUuid::parse_str(s).is_ok() {
                res = Ok(());
            }
        }
        res
    }
}
