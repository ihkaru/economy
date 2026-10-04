use std::collections::BTreeMap;
use crate::core::domain::item::id::ItemId;
use crate::core::domain::ledger::entry::LedgerEntry;
use crate::core::ports::ledger_store::LedgerStorePort;

#[derive(Default)]
pub struct MemoryLedgerStore {
    entries: Vec<LedgerEntry>,
    item_counts: BTreeMap<ItemId, u64>,
    bilateral_trade_count: usize,
    bilateral_item_counts: BTreeMap<ItemId, u64>,
}

impl MemoryLedgerStore {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            item_counts: BTreeMap::new(),
            bilateral_trade_count: 0,
            bilateral_item_counts: BTreeMap::new(),
        }
    }

    fn index_entry(&mut self, entry: &LedgerEntry) {
        for it in &entry.items_from_a {
            *self.item_counts.entry(it.item_id).or_insert(0) += 1;
        }
        for it in &entry.items_from_b {
            *self.item_counts.entry(it.item_id).or_insert(0) += 1;
        }

        if !entry.items_from_a.is_empty() && !entry.items_from_b.is_empty() {
            self.bilateral_trade_count += 1;
            for it in &entry.items_from_a {
                *self.bilateral_item_counts.entry(it.item_id).or_insert(0) += 1;
            }
            for it in &entry.items_from_b {
                *self.bilateral_item_counts.entry(it.item_id).or_insert(0) += 1;
            }
        }
    }
}

impl LedgerStorePort for MemoryLedgerStore {
    fn record(&mut self, entry: LedgerEntry) -> Result<(), String> {
        self.index_entry(&entry);
        self.entries.push(entry);
        Ok(())
    }

    fn record_batch(&mut self, mut entries: Vec<LedgerEntry>) -> Result<(), String> {
        for e in &entries {
            self.index_entry(e);
        }
        self.entries.append(&mut entries);
        Ok(())
    }

    fn total_records(&self) -> usize {
        self.entries.len()
    }

    fn all_entries(&self) -> &[LedgerEntry] {
        &self.entries
    }

    fn drain_all(&mut self) -> Vec<LedgerEntry> {
        self.item_counts.clear();
        self.bilateral_trade_count = 0;
        self.bilateral_item_counts.clear();
        std::mem::take(&mut self.entries)
    }

    fn item_transaction_counts(&self) -> &BTreeMap<ItemId, u64> {
        &self.item_counts
    }

    fn bilateral_market_trades_count(&self) -> usize {
        self.bilateral_trade_count
    }

    fn bilateral_trade_item_counts(&self) -> &BTreeMap<ItemId, u64> {
        &self.bilateral_item_counts
    }
}

