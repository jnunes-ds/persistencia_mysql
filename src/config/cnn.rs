use std::env;
use dotenv::dotenv;
use mysql::{Opts, Pool, PooledConn};

pub fn get_connection() -> Result<PooledConn, mysql::Error> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:root@localhost/clients_rust_db".to_string());

    let opts = Opts::from_url(&database_url)?;
    let pool = Pool::new(opts)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}