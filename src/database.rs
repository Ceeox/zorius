use std::path::Path;

use log::{error, info};
use sea_orm::DatabaseConnection;
use sqlx::{migrate::Migrator, Pool, Postgres};

use crate::{
    config::CONFIG,
    models::users::{new_user, user_by_email},
    view::users::NewUser,
};

pub struct Database {
    database: DatabaseConnection,
}

impl Database {
    pub async fn new(pool: Pool<Postgres>, database: DatabaseConnection) -> Self {
        info!("Running migrations...");
        let m = Migrator::new(Path::new("./migrations"))
            .await
            .expect("Failed to run migrates");
        m.run(&pool).await.expect("Failed to run migrates");

        let _self = Self { database };

        let admin_user = NewUser {
            email: CONFIG.admin_user.email.clone(),
            password: CONFIG.admin_user.password.clone(),
            firstname: CONFIG.admin_user.firstname.clone(),
            lastname: CONFIG.admin_user.lastname.clone(),
        };

        if user_by_email(&_self.database, &admin_user.email)
            .await
            .is_ok()
        {
            return _self;
        }

        match new_user(&_self.database, admin_user).await {
            Ok(_) => {}
            Err(e) => error!("Failed to create admin user: {:?}", e),
        }

        _self
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.database
    }
}
