mod app;
mod config;
mod database;

#[cfg(test)]
mod test;

extern crate openssl;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
// #[macro_use]
extern crate rocket_sync_db_pools;

fn main() {
    app::server::main();
}
