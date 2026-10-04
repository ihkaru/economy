use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

use super::catalog::build_canonical_items;
use super::definition::ItemDefinition;
use super::id::ItemId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRegistry {
    items: BTreeMap<ItemId, ItemDefinition>,
}

impl ItemRegistry {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, def: ItemDefinition) {
        self.items.insert(def.id, def);
    }

    pub fn get(&self, id: ItemId) -> Option<&ItemDefinition> {
        self.items.get(&id)
    }

    pub fn all(&self) -> Vec<&ItemDefinition> {
        self.items.values().collect()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Canonical master item catalog reflecting historical economic emergence
    pub fn canonical() -> Self {
        let mut reg = Self::new();
        for item in build_canonical_items() {
            reg.register(item);
        }
        reg
    }
}

impl Default for ItemRegistry {
    fn default() -> Self {
        Self::canonical()
    }
}
