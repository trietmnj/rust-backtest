use std::error::Error;

use mysql::{conn::pool::Pool, PooledConn, Opts};

use super::model;

pub fn new_mysql_conn(opts: Opts) -> Result<PooledConn, Error> {
    let url = Opts::from_url("mysql://root:@host.docker.internal:3306/stocks");
    let pool = Pool::new(url)?;
    pool.check_health(check_health);
    Ok(pool.get_conn()?);
}
