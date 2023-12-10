use mysql::*;
pub fn db_connect(database_url: &str) -> PooledConn {
    let pool = Pool::new(database_url).unwrap();
    //println!("Connected to {}", database_url);
    let conn = pool.get_conn().unwrap();
    conn
}
