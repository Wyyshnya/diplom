pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use dotenv::dotenv;

embed_migrations!();

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn run_migrations(conn: &PgConnection) {
    let _ = diesel_migrations::run_pending_migrations(&*conn);
}

pub fn establish_connection() -> DbPool {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("URL db doesn't set");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        Pool::builder().build(manager).expect("Failed to create db pool")

}