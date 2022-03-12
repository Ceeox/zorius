use async_graphql::CustomValidator;
use url::Url as CrateUrl;
use uuid::Uuid as CrateUuid;

pub struct Password;
const MIN_PW_LEN: usize = 8;
const MAX_PW_LEN: usize = 255;

impl CustomValidator<String> for Password {
    fn check(&self, value: &String) -> Result<(), String> {
        if value.len() >= MIN_PW_LEN && value.len() <= MAX_PW_LEN {
            Ok(())
        } else {
            Err(format!(
                "password must be longer than {MIN_PW_LEN} chars and lower than {MAX_PW_LEN} chars"
            ))
        }
    }
}
pub struct Url;

impl CustomValidator<String> for Url {
    fn check(&self, value: &String) -> Result<(), String> {
        let mut res = Err("not a valid url".to_owned());
        if CrateUrl::parse(value).is_ok() {
            res = Ok(())
        }
        res
    }
}

pub struct Uuid;

impl CustomValidator<String> for Uuid {
    fn check(&self, value: &String) -> Result<(), String> {
        let mut res = Err("not a valid uuid".to_owned());
        if CrateUuid::parse_str(value).is_ok() {
            res = Ok(());
        }
        res
    }
}
