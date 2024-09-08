use diesel::{r2d2::{self, ConnectionManager, PoolError}, PgConnection};
use ::r2d2::Pool;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> DbPool {
    println!("Connecting to database");
    let database_url = "postgres://postgres:postgres@db:5432/postgres";
    init_pool(&database_url).expect("Failed to create pool")
}
