use serde::{Deserialize, Serialize};
use super::id::{ItemId, ItemInstanceId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemInstance {
    pub instance_id: Option<ItemInstanceId>,
    pub item_id: ItemId,
    pub quantity: u32,
    /// Flexible JSON payload for instance-specific metadata (e.g. lot number, durability, quality score, origin)
    pub metadata: serde_json::Value,
}

impl ItemInstance {
    pub fn new(item_id: ItemId, quantity: u32, metadata: serde_json::Value) -> Self {
        Self {
            instance_id: None,
            item_id,
            quantity,
            metadata,
        }
    }

    pub fn with_instance_id(mut self, id: ItemInstanceId) -> Self {
        self.instance_id = Some(id);
        self
    }
}
