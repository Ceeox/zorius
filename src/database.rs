use std::path::Path;

use log::{error, info};
use sqlx::migrate::Migrator;
use sqlx::{Pool, Postgres};

use crate::{config::CONFIG, models::users::User, view::users::NewUser};

pub struct Database {
    database: Pool<Postgres>,
}

impl Database {
    pub async fn new(database: Pool<Postgres>) -> Self {
        info!("Running migrations...");
        let m = Migrator::new(Path::new("./migrations"))
            .await
            .expect("Failed to run migrates");
        m.run(&database).await.expect("Failed to run migrates");

        let _self = Self { database };

        let admin_user = NewUser {
            email: CONFIG.admin_user.email.clone(),
            password: CONFIG.admin_user.password.clone(),
            firstname: CONFIG.admin_user.firstname.clone(),
            lastname: CONFIG.admin_user.lastname.clone(),
        };

        if User::user_by_email(&_self.database, &admin_user.email)
            .await
            .is_ok()
        {
            return _self;
        }

        match User::new(&_self.database, admin_user).await {
            Ok(_) => {}
            Err(e) => error!("Failed to create admin user: {:?}", e),
        }

        _self
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.database
    }
}
