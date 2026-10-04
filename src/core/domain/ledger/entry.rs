use serde::{Deserialize, Serialize};
use crate::core::domain::agent::id::AgentId;
use crate::core::domain::item::instance::ItemInstance;
use crate::core::domain::time::{RunId, Tick};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// Atomic transaction identifier binding all transferred items in this economic event
    pub trx_id: u64,
    pub run_id: RunId,
    pub tick_id: Tick,
    pub party_a: AgentId,
    pub party_b: AgentId,
    /// Bundle of items, services, knowledge, or permits transferred from Party A to Party B
    pub items_from_a: Vec<ItemInstance>,
    /// Bundle of items, services, knowledge, or permits transferred from Party B to Party A
    pub items_from_b: Vec<ItemInstance>,
    /// Flexible JSON column for arbitrary transaction metadata (e.g. discovery trigger, exchange ratio, surplus)
    pub metadata: serde_json::Value,
}

impl LedgerEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trx_id: u64,
        run_id: RunId,
        tick_id: Tick,
        party_a: AgentId,
        party_b: AgentId,
        items_from_a: Vec<ItemInstance>,
        items_from_b: Vec<ItemInstance>,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            trx_id,
            run_id,
            tick_id,
            party_a,
            party_b,
            items_from_a,
            items_from_b,
            metadata,
        }
    }

    /// Convenience constructor for single item or unilateral transfers
    #[allow(clippy::too_many_arguments)]
    pub fn single(
        trx_id: u64,
        run_id: RunId,
        tick_id: Tick,
        party_a: AgentId,
        party_b: AgentId,
        item_from_a: Option<ItemInstance>,
        item_from_b: Option<ItemInstance>,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            trx_id,
            run_id,
            tick_id,
            party_a,
            party_b,
            items_from_a: item_from_a.into_iter().collect(),
            items_from_b: item_from_b.into_iter().collect(),
            metadata,
        }
    }

    pub fn entry_id(&self) -> u64 {
        self.trx_id
    }
}
