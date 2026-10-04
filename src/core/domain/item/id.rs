use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ItemId(pub u64);

impl ItemId {
    pub const TIMBER: ItemId = ItemId(101);
    pub const FISH: ItemId = ItemId(102);
    pub const GRAIN: ItemId = ItemId(103);
    pub const BERRIES: ItemId = ItemId(104);
    pub const SALT: ItemId = ItemId(105);
    pub const RAFT: ItemId = ItemId(106);
    pub const SHELLS: ItemId = ItemId(107);
    pub const STONE_AXE: ItemId = ItemId(108);
    pub const FISHING_NET: ItemId = ItemId(109);

    // Non-rival Knowledge / Recipes
    pub const KNOWLEDGE_RAFT_BUILDING: ItemId = ItemId(201);
    pub const KNOWLEDGE_FISH_CURING: ItemId = ItemId(202);
    pub const KNOWLEDGE_FIRE_MAKING: ItemId = ItemId(203);
    pub const KNOWLEDGE_TOOL_CRAFTING: ItemId = ItemId(204);

    // Institutional Permits / Concessions
    pub const PERMIT_FISHING_RIGHT: ItemId = ItemId(301);
    pub const PERMIT_FORESTRY_RIGHT: ItemId = ItemId(302);

    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn is_knowledge(&self) -> bool {
        (200..=299).contains(&self.0)
    }

    pub fn is_permit(&self) -> bool {
        (300..=399).contains(&self.0)
    }

    pub fn is_tool(&self) -> bool {
        matches!(*self, Self::RAFT | Self::STONE_AXE | Self::FISHING_NET)
    }
}

impl std::fmt::Display for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item-{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ItemInstanceId(pub u64);

impl ItemInstanceId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}
