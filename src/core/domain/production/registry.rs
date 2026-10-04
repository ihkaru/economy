use crate::core::domain::item::id::ItemId;
use super::recipe::{ProductionRecipe, RecipeIngredient};

pub struct RecipeRegistry {
    recipes: Vec<ProductionRecipe>,
}

impl RecipeRegistry {
    pub fn new(recipes: Vec<ProductionRecipe>) -> Self {
        Self { recipes }
    }

    pub fn all(&self) -> &[ProductionRecipe] {
        &self.recipes
    }

    pub fn get_recipe(&self, id: u32) -> Option<&ProductionRecipe> {
        self.recipes.iter().find(|r| r.recipe_id == id)
    }

    /// Canonical multi-tier supply chain recipe repertoire
    pub fn canonical() -> Self {
        Self::new(vec![
            // 1. Logistics Container: Woven Carrying Basket
            ProductionRecipe::new(
                1,
                "Woven Carrying Basket",
                "container_crafting",
                &[RecipeIngredient { item_id: ItemId::TIMBER, quantity: 2 }],
                &[RecipeIngredient { item_id: ItemId::WOVEN_BASKET, quantity: 1 }],
                Some(ItemId::KNOWLEDGE_BASKET_WEAVING),
                None,
                0.0,
                120.0,
            ),
            // 2. Forestry Capital Tool: Polished Stone Axe
            ProductionRecipe::new(
                2,
                "Polished Stone Axe",
                "capital_tool_production",
                &[
                    RecipeIngredient { item_id: ItemId::STONE, quantity: 1 },
                    RecipeIngredient { item_id: ItemId::TIMBER, quantity: 1 },
                ],
                &[RecipeIngredient { item_id: ItemId::STONE_AXE, quantity: 1 }],
                Some(ItemId::KNOWLEDGE_TOOL_CRAFTING),
                None,
                0.0,
                180.0,
            ),
            // 3. Aquatic Harvesting Capital Tool: Woven Fishing Net
            ProductionRecipe::new(
                3,
                "Woven Fishing Net",
                "capital_tool_production",
                &[RecipeIngredient { item_id: ItemId::TIMBER, quantity: 4 }],
                &[RecipeIngredient { item_id: ItemId::FISHING_NET, quantity: 1 }],
                Some(ItemId::KNOWLEDGE_TOOL_CRAFTING),
                None,
                0.0,
                180.0,
            ),
            // 4. Maritime Transport Capital: Timber Watercraft Raft
            ProductionRecipe::new(
                4,
                "Maritime Timber Raft",
                "capital_tool_production",
                &[RecipeIngredient { item_id: ItemId::TIMBER, quantity: 10 }],
                &[RecipeIngredient { item_id: ItemId::RAFT, quantity: 1 }],
                Some(ItemId::KNOWLEDGE_RAFT_BUILDING),
                None,
                0.0,
                500.0,
            ),
            // 5. Therapeutic Pharmacopoeia: Herbal Medicine Formulation
            ProductionRecipe::new(
                5,
                "Herbal Medicine",
                "pharmacopoeia_preparation",
                &[RecipeIngredient { item_id: ItemId::BERRIES, quantity: 3 }],
                &[RecipeIngredient { item_id: ItemId::HERBAL_MEDICINE, quantity: 1 }],
                Some(ItemId::KNOWLEDGE_HERBAL_MEDICINE),
                None,
                0.0,
                80.0,
            ),
            // 6. Food Preservation Value Chain: Salt-Cured Preserved Fish
            ProductionRecipe::new(
                6,
                "Salt-Cured Preserved Fish",
                "food_preservation",
                &[
                    RecipeIngredient { item_id: ItemId::FISH, quantity: 2 },
                    RecipeIngredient { item_id: ItemId::SALT, quantity: 1 },
                ],
                &[RecipeIngredient { item_id: ItemId::CURED_FISH, quantity: 2 }],
                Some(ItemId::KNOWLEDGE_FISH_CURING),
                None,
                0.0,
                60.0,
            ),
            // 7. Desiccation Preservation: Sun-Dried Desiccated Berries
            ProductionRecipe::new(
                7,
                "Sun-Dried Desiccated Berries",
                "food_preservation",
                &[RecipeIngredient { item_id: ItemId::BERRIES, quantity: 3 }],
                &[RecipeIngredient { item_id: ItemId::DRIED_BERRIES, quantity: 2 }],
                None, // Simple traditional sun desiccation
                None,
                0.0,
                40.0,
            ),
            // 8. Pyrotechnic Antimicrobial Smoking: Wood-Smoked Preserved Fish
            ProductionRecipe::new(
                8,
                "Wood-Smoked Preserved Fish",
                "food_preservation",
                &[
                    RecipeIngredient { item_id: ItemId::FISH, quantity: 2 },
                    RecipeIngredient { item_id: ItemId::TIMBER, quantity: 1 },
                ],
                &[RecipeIngredient { item_id: ItemId::SMOKED_FISH, quantity: 2 }],
                Some(ItemId::KNOWLEDGE_FIRE_MAKING),
                None,
                0.0,
                80.0,
            ),
            // 9. Neolithic Ceramic Pyrotechnology: Ceramic Storage Pottery Jar
            ProductionRecipe::new(
                9,
                "Ceramic Storage Pottery Jar",
                "ceramic_storage",
                &[
                    RecipeIngredient { item_id: ItemId::CLAY, quantity: 4 },
                    RecipeIngredient { item_id: ItemId::TIMBER, quantity: 1 },
                ],
                &[RecipeIngredient { item_id: ItemId::POTTERY_JAR, quantity: 1 }],
                Some(ItemId::KNOWLEDGE_POTTERY_MAKING),
                None,
                0.0,
                200.0,
            ),
            // 10. Pyrotechnic Meat Smoking: Wood-Smoked Preserved Meat
            ProductionRecipe::new(
                10,
                "Wood-Smoked Preserved Meat",
                "food_preservation",
                &[
                    RecipeIngredient { item_id: ItemId::RAW_MEAT, quantity: 2 },
                    RecipeIngredient { item_id: ItemId::TIMBER, quantity: 1 },
                ],
                &[RecipeIngredient { item_id: ItemId::SMOKED_MEAT, quantity: 2 }],
                Some(ItemId::KNOWLEDGE_FIRE_MAKING),
                None,
                0.0,
                80.0,
            ),
            // 11. Tailored Warm Leather Garment: Cold Weather Thermal Protection
            ProductionRecipe::new(
                11,
                "Warm Leather Garment",
                "clothing_tailoring",
                &[
                    RecipeIngredient { item_id: ItemId::RAW_HIDE, quantity: 2 },
                    RecipeIngredient { item_id: ItemId::TIMBER, quantity: 1 },
                ],
                &[RecipeIngredient { item_id: ItemId::LEATHER_CLOTHING, quantity: 1 }],
                Some(ItemId::KNOWLEDGE_LEATHER_WORKING),
                None,
                0.0,
                180.0,
            ),
        ])
    }
}
