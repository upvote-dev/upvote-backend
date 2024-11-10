use diesel_migrations::MigrationHarness;

pub mod models;

pub mod routes;

pub mod schema;

#[cfg(test)]
mod tests;

pub const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations");

pub fn db_init() {
    log::info!("Initializing DB");
    lazy_static::initialize(&rust_actix_diesel_auth_scaffold::POOL);
    let mut connection = rust_actix_diesel_auth_scaffold::establish_connection()
        .expect("Failed to create connection");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}
