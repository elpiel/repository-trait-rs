//use r2d2::Pool;
//use r2d2_postgres::PostgresConnectionManager;
//use postgres::NoTls;

pub mod channel;

//pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;
pub type DbPool = bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>;