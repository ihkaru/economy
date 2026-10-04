use crate::core::domain::ledger::entry::LedgerEntry;
use crate::core::ports::ledger_store::LedgerStorePort;

#[derive(Default)]
pub struct MemoryLedgerStore {
    entries: Vec<LedgerEntry>,
}

impl MemoryLedgerStore {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl LedgerStorePort for MemoryLedgerStore {
    fn record(&mut self, entry: LedgerEntry) -> Result<(), String> {
        self.entries.push(entry);
        Ok(())
    }

    fn record_batch(&mut self, mut entries: Vec<LedgerEntry>) -> Result<(), String> {
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
        std::mem::take(&mut self.entries)
    }
}
