use std::env;

pub struct RdbmsConfig {
    db_user: String,
    db_pass: String,
    db_host: String,
    db_port: String,
    db_name: String,
    external_lib: String,
    db_driver: String,
    db_store: String,
}

pub fn rdbms_config_from_env() -> RdbmsConfig {
    return RdbmsConfig {
        db_user: env::var("DBUSER"),
        db_pass: env::var("DBPASS"),
        db_host: env::var("DBHOST"),
        db_port: env::var("DBPORT"),
        db_name: env::var("DBNAME"),
        external_lib: env::var("EXTERNAL_LIB"),
        db_driver: env::var("DBDRIVER"),
        db_store: env::var("DBSTORE"),
    }
}
