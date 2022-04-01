use std::any::Any;

use super::{store::DataStore, transaction::Tx};

struct FluentInsert {
    store: DataStore,
    ds: DataSet,
    batch: bool,
    batch_size: u64,
    tx: &Tx,
    records: Any,
}

const DEFAULT_BATCHSIZE: u32 = 100;

impl FluentInsert {
    pub fn tx(&self, tx:&Transaction) -> Self {
        self.tx = tx;
        return self
    }

    pub fn batch(&self, tx:&Transaction) -> Self {
        self.tx = tx;
        return self
    }

    pub fn batch_size(&self, bs:u32) -> Self {
        self.tx = tx;
        return self
    }

    pub fn records(&self, recs:&Transaction) -> Self {
        self.records = recs;
        return self
    }

    pub fn execute(&self, tx:&Transaction) -> Self {
        return self.store.insert_records(self.ds)
    }
}
