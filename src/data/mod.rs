mod model;
mod sql;
mod query;

use barter::data::handler::historical::HistoricalDataLego;

pub mod store {
    use std::error::Error;

    use mysql::{PooledConn, Transaction};

    use super::sql;

    pub fn new_mysql_store() {
        let opts = Opts::from_url("mysql://root:@host.docker.internal:3306/stocks");
        let conn = sql::new_mysql_conn(opts).unwrap();
        let lego = HistoricalDataLego {
            exchange: String::from(""),
            symbol: String::from("")
        };
    }

    pub struct MysqlStore {
        conn: PooledConn,
    }

    impl MysqlStore {
        pub fn fetch(&self, tx &Transaction, query ) -> Result<(), Error> {
            get_select_statement(query.dataset, query.statement_key, query.statement, query.suffix, query.statement_appends)
        }

        pub fn select(&self) -> self {
        }

        pub fn dest<&T>(&self, var: &T) -> self {
        }

        pub fn params(args: &[...]) -> self {
        }
    }

    fn get_select_statement(dataset: _, statement_key: String, statement: String, suffix: String, appends: _) -> Result<String, dyn Error> {
        if (key != "") {
        }
        if (statement != "") {
        }
    }
}
