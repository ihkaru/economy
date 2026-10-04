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
    pub const HERBAL_MEDICINE: ItemId = ItemId(110);
    pub const WOVEN_BASKET: ItemId = ItemId(111);
    pub const CURED_FISH: ItemId = ItemId(112);
    pub const DRIED_BERRIES: ItemId = ItemId(113);
    pub const SMOKED_FISH: ItemId = ItemId(114);
    pub const CLAY: ItemId = ItemId(115);
    pub const POTTERY_JAR: ItemId = ItemId(116);

    // Non-rival Knowledge / Recipes
    pub const KNOWLEDGE_RAFT_BUILDING: ItemId = ItemId(201);
    pub const KNOWLEDGE_FISH_CURING: ItemId = ItemId(202);
    pub const KNOWLEDGE_FIRE_MAKING: ItemId = ItemId(203);
    pub const KNOWLEDGE_TOOL_CRAFTING: ItemId = ItemId(204);
    pub const KNOWLEDGE_HERBAL_MEDICINE: ItemId = ItemId(205);
    pub const KNOWLEDGE_BASKET_WEAVING: ItemId = ItemId(206);
    pub const KNOWLEDGE_POTTERY_MAKING: ItemId = ItemId(207);

    // Institutional Permits / Concessions
    pub const PERMIT_FISHING_RIGHT: ItemId = ItemId(301);
    pub const PERMIT_FORESTRY_RIGHT: ItemId = ItemId(302);

    // Services & Labor Time (Intangible Services / Man-Hours)
    pub const SERVICE_LABOR: ItemId = ItemId(401);
    pub const SERVICE_EDUCATION: ItemId = ItemId(402);
    pub const SERVICE_TRANSPORT: ItemId = ItemId(403);
    pub const SERVICE_MEDICAL: ItemId = ItemId(404);

    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn is_good(&self) -> bool {
        (100..=199).contains(&self.0)
    }

    pub fn is_knowledge(&self) -> bool {
        (200..=299).contains(&self.0)
    }

    pub fn is_permit(&self) -> bool {
        (300..=399).contains(&self.0)
    }

    pub fn is_service(&self) -> bool {
        (400..=499).contains(&self.0)
    }

    pub fn is_tool(&self) -> bool {
        matches!(*self, Self::RAFT | Self::STONE_AXE | Self::FISHING_NET | Self::WOVEN_BASKET | Self::POTTERY_JAR)
    }

    pub fn default_weight_kg(&self) -> f64 {
        match *self {
            Self::TIMBER => 5.0,
            Self::FISH => 0.5,
            Self::GRAIN => 1.0,
            Self::BERRIES => 0.2,
            Self::SALT => 0.5,
            Self::RAFT => 45.0,
            Self::SHELLS => 0.05,
            Self::STONE_AXE => 2.5,
            Self::FISHING_NET => 1.5,
            Self::HERBAL_MEDICINE => 0.1,
            Self::WOVEN_BASKET => 0.5,
            Self::CURED_FISH => 0.4,
            Self::DRIED_BERRIES => 0.1,
            Self::SMOKED_FISH => 0.4,
            Self::CLAY => 0.5,
            Self::POTTERY_JAR => 4.0,
            _ => 0.0,
        }
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
