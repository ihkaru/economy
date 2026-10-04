use serde::{Deserialize, Serialize};
use super::id::ItemId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemCategory {
    Good,
    Service,
    Knowledge,
    Permit,
    Currency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemNature {
    /// Physical item: rivalrous, depleted when transferred or consumed
    RivalPhysical,
    /// Idea / Blueprint / Recipe: non-rivalrous, retained by teacher when shared
    NonRivalKnowledge,
    /// Legal permit / Institutional right / Concession
    InstitutionalRight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub category: ItemCategory,
    pub nature: ItemNature,
    pub is_perishable: bool,
    /// Flexible JSON attributes for supply chain properties (e.g. recipe, inputs, shelf-life, tier)
    pub attributes: serde_json::Value,
}

impl ItemDefinition {
    pub fn new(id: ItemId, name: impl Into<String>, category: ItemCategory, attributes: serde_json::Value) -> Self {
        let nature = match category {
            ItemCategory::Knowledge => ItemNature::NonRivalKnowledge,
            ItemCategory::Permit => ItemNature::InstitutionalRight,
            _ => ItemNature::RivalPhysical,
        };
        Self {
            id,
            name: name.into(),
            category,
            nature,
            is_perishable: false,
            attributes,
        }
    }

    pub fn with_nature(mut self, nature: ItemNature) -> Self {
        self.nature = nature;
        self
    }

    pub fn is_non_rival(&self) -> bool {
        matches!(self.nature, ItemNature::NonRivalKnowledge)
    }
}
