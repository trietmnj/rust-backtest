// struct QueryInput {
//     dataset:,
//     statement_key:,
//     statement:,
//     suffix:,
//     bind_params:,
//     statement_appends:,
//     panic_on_err:
// }

mod super::super::transaction;

use std::{error::Error, any::Any};

use super::{transaction::Tx, query::Rows};

pub trait DataStore {
    pub fn connection();
    pub fn transaction() -> Result<Box<Tx>, Error>;
    pub fn fetch(tx:Box<Tx>, i:QueryInput, dest:Any) -> Result<(), Error>;
    pub fn fetch_rows(tx:Box<Tx>, i:QueryInput) -> Result<Rows, Error>;
    pub fn get_json(i:QueryInput, jo:JsonOpts) -> Result<(), Error>;
    pub fn get_csv(i:QueryInput, co:CsvOpts) -> Result<(), Error>;
    pub fn select(statement:String) -> Box<FluentInsert>;
    pub fn insert(ds:Dataset) -> Box<FluentInsert>;
    pub fn insert_records(ds:DataSet, records:Any, batch:bool, batch_size:u64, tx:&Tx) -> Result<(), Error>;
}
