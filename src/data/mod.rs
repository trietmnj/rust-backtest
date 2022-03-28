mod model;
mod sql;

use barter::data::handler::historical::HistoricalDataLego;

pub mod store {
    use mysql::PooledConn;

    use super::sql;

    pub fn new_mysql_store() {
        let opts = Opts::from_url("mysql://root:@host.docker.internal:3306/stocks");
        let conn = sql::new_mysql_conn(opts).unwrap();
        let lego = HistoricalDataLego {};
    }

    struct MysqlStore {
        conn: PooledConn,
    }

    impl MysqlStore {
        fn select(&self) -> self {
        }

        fn dest<&T>(&self, var: &T) -> self {
        }

        fn params(args: &[...])
    }
}
