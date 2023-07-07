#[macro_use]
extern crate rocket;

pub mod modules;

use std::path::PathBuf;
use modules::cors;

pub struct PurplePortalServer {
    username: String,
    password: String,
    port: u16,
    vault_root: PathBuf,
}

impl PurplePortalServer {
    pub fn new(
        username: String,
        password: String,
        port: u16,
        vault_root: PathBuf,
    ) -> Self {
        Self {
            username,
            password,
            port,
            vault_root,
        }
    }

    pub async fn start() -> Result<(), rocket::Error> {
        rocket::build()
            .mount("/", cors::routes::create_routes())
            .launch()
            .await?;

        Ok(())
    }
}
