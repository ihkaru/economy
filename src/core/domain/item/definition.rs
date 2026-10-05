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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionRole {
    /// Essential biophysical sustenance required to preserve life and prevent starvation/illness
    Sustenance,
    /// Physical capital equipment required as an operational prerequisite for harvesting or craft
    ProductionCapital,
    /// Prestige good, status symbol, or ceremonial gift required for courtship, marriage, and alliances
    SocialStatusAndGifting,
    /// Liquid medium of exchange, promissory credit token, or warehouse claim required to settle contracts
    MediumAndCollateral,
    /// Institutional concession or legal permit required to access common pool resources
    InstitutionalConcession,
    /// Raw unshaped or intermediate material destined for transformation in multi-tier supply chains
    RawInputMaterial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub category: ItemCategory,
    pub nature: ItemNature,
    pub interaction_role: InteractionRole,
    pub weight_kg: f64,
    pub is_perishable: bool,
    /// Flexible JSON attributes for supply chain properties (e.g. recipe, inputs, shelf-life, tier, historical_era)
    pub attributes: serde_json::Value,
}

impl ItemDefinition {
    pub fn new(id: ItemId, name: impl Into<String>, category: ItemCategory, attributes: serde_json::Value) -> Self {
        let (nature, weight_kg) = match category {
            ItemCategory::Knowledge => (ItemNature::NonRivalKnowledge, 0.0),
            ItemCategory::Permit => (ItemNature::InstitutionalRight, 0.0),
            ItemCategory::Service => (ItemNature::RivalPhysical, 0.0), // Service time has 0 mass
            _ => (ItemNature::RivalPhysical, 1.0),
        };
        let interaction_role = match category {
            ItemCategory::Good => InteractionRole::RawInputMaterial,
            ItemCategory::Currency => InteractionRole::MediumAndCollateral,
            ItemCategory::Permit => InteractionRole::InstitutionalConcession,
            ItemCategory::Knowledge => InteractionRole::ProductionCapital,
            ItemCategory::Service => InteractionRole::ProductionCapital,
        };
        Self {
            id,
            name: name.into(),
            category,
            nature,
            interaction_role,
            weight_kg,
            is_perishable: false,
            attributes,
        }
    }

    pub fn with_nature(mut self, nature: ItemNature) -> Self {
        self.nature = nature;
        self
    }

    pub fn with_interaction_role(mut self, role: InteractionRole) -> Self {
        self.interaction_role = role;
        self
    }

    pub fn with_weight(mut self, weight_kg: f64) -> Self {
        self.weight_kg = weight_kg;
        self
    }

    pub fn with_perishable(mut self, perishable: bool) -> Self {
        self.is_perishable = perishable;
        self
    }

    pub fn is_non_rival(&self) -> bool {
        matches!(self.nature, ItemNature::NonRivalKnowledge)
    }

    pub fn is_sustenance(&self) -> bool {
        matches!(self.interaction_role, InteractionRole::Sustenance)
    }

    pub fn is_production_capital(&self) -> bool {
        matches!(self.interaction_role, InteractionRole::ProductionCapital)
    }

    pub fn is_social_status(&self) -> bool {
        matches!(self.interaction_role, InteractionRole::SocialStatusAndGifting)
    }

    pub fn is_medium_or_collateral(&self) -> bool {
        matches!(self.interaction_role, InteractionRole::MediumAndCollateral)
    }

    pub fn is_institutional_concession(&self) -> bool {
        matches!(self.interaction_role, InteractionRole::InstitutionalConcession)
    }

    pub fn is_raw_material(&self) -> bool {
        matches!(self.interaction_role, InteractionRole::RawInputMaterial)
    }

    /// Generates formal JSON Schema (Draft-07 compliant) for ItemDefinition
    pub fn json_schema() -> serde_json::Value {
        serde_json::json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "ItemDefinition",
            "description": "Standard Specification Schema for Economic Items, Goods, Services, Knowledge, and Permits",
            "type": "object",
            "required": ["id", "name", "category", "nature", "interaction_role", "weight_kg", "is_perishable", "attributes"],
            "properties": {
                "id": {
                    "type": "integer",
                    "minimum": 1,
                    "description": "Unique canonical identifier of the item"
                },
                "name": {
                    "type": "string",
                    "description": "Human-readable name of the item/service"
                },
                "category": {
                    "type": "string",
                    "enum": ["Good", "Service", "Knowledge", "Permit", "Currency"],
                    "description": "High-level economic category of the item"
                },
                "nature": {
                    "type": "string",
                    "enum": ["RivalPhysical", "NonRivalKnowledge", "InstitutionalRight"],
                    "description": "Ontological nature governing transferability and rivalry"
                },
                "interaction_role": {
                    "type": "string",
                    "enum": [
                        "Sustenance",
                        "ProductionCapital",
                        "SocialStatusAndGifting",
                        "MediumAndCollateral",
                        "InstitutionalConcession",
                        "RawInputMaterial"
                    ],
                    "description": "Functional prerequisite role of this item in inter-agent interactions"
                },
                "weight_kg": {
                    "type": "number",
                    "minimum": 0.0,
                    "description": "Weight in kilograms per unit (0.0 for services/knowledge)"
                },
                "is_perishable": {
                    "type": "boolean",
                    "description": "Whether this item decays/perishes over time without preservation"
                },
                "attributes": {
                    "type": "object",
                    "required": ["historical_era", "unit", "utility_type"],
                    "properties": {
                        "historical_era": {
                            "type": "string",
                            "enum": [
                                "Paleolithic_Foraging",
                                "Paleolithic_Pyrotechnology",
                                "Mesolithic_Aquatic_Revolution",
                                "Neolithic_Preservation_Storage",
                                "Neolithic_Division_Of_Labor",
                                "Proto_Historic_Currency"
                            ],
                            "description": "Phase of historical emergence in human economic evolution"
                        },
                        "unit": {
                            "type": "string",
                            "description": "Physical or accounting unit of measurement"
                        },
                        "utility_type": {
                            "type": "string",
                            "description": "Economic utility classification (Nutrition, CapitalTool, RawMaterial, MediumOfExchange, Skill, LaborTime)"
                        },
                        "recipe": {
                            "type": "object",
                            "description": "Input materials and labor required to craft this item"
                        }
                    }
                }
            }
        })
    }
}
