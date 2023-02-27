use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;
// use rocket_sync_db_pools::diesel;

// pub struct Db(diesel::PgConnection);
#[database("questions")]
pub struct Db(diesel::PgConnection);

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("src/database/migrations");

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}
