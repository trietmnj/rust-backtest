mod super::super::config;
use super::config;

pub fn new_rdbms_store(c: &config::RdbmsConfig) -> Result<, Error> {
    match c.db_store {
        "pgx" =>
    }
    Ok(())
}
