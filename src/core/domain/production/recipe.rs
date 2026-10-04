use crate::core::domain::item::id::ItemId;

/// Fixed-proportion ingredient in a Leontief production recipe
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecipeIngredient {
    pub item_id: ItemId,
    pub quantity: u32,
}

/// Data-driven specification of a multi-tier manufacturing supply chain recipe
#[derive(Debug, Clone)]
pub struct ProductionRecipe {
    pub recipe_id: u32,
    pub name: &'static str,
    pub category: &'static str,
    pub inputs: &'static [RecipeIngredient],
    pub outputs: &'static [RecipeIngredient],
    pub required_knowledge: Option<ItemId>,
    pub required_tool: Option<ItemId>,
    pub tool_wear_probability: f64,
    pub labor_calorie_cost: f64,
}

impl ProductionRecipe {
    pub const fn new(
        recipe_id: u32,
        name: &'static str,
        category: &'static str,
        inputs: &'static [RecipeIngredient],
        outputs: &'static [RecipeIngredient],
        required_knowledge: Option<ItemId>,
        required_tool: Option<ItemId>,
        tool_wear_probability: f64,
        labor_calorie_cost: f64,
    ) -> Self {
        Self {
            recipe_id,
            name,
            category,
            inputs,
            outputs,
            required_knowledge,
            required_tool,
            tool_wear_probability,
            labor_calorie_cost,
        }
    }
}
