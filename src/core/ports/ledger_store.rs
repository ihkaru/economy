use std::collections::BTreeMap;
use crate::core::domain::item::id::ItemId;
use crate::core::domain::ledger::entry::LedgerEntry;

pub trait LedgerStorePort: Send + Sync {
    fn record(&mut self, entry: LedgerEntry) -> Result<(), String>;
    fn record_batch(&mut self, entries: Vec<LedgerEntry>) -> Result<(), String>;
    fn total_records(&self) -> usize;
    fn all_entries(&self) -> &[LedgerEntry];
    fn drain_all(&mut self) -> Vec<LedgerEntry>;
    fn item_transaction_counts(&self) -> &BTreeMap<ItemId, u64>;
}

