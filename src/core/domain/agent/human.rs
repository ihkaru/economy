use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::core::domain::agent::id::AgentId;
use crate::core::domain::agent::traits::{EconomicActor, HasLifecycle, Identifiable, SocialActor};
use crate::core::domain::item::id::ItemId;
use crate::core::domain::time::Tick;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VitalStatus {
    Alive,
    Deceased { tick_of_death: Tick, reason: String },
}

use crate::core::domain::agent::stats::AgentPersonalStats;
use crate::core::domain::spatial::coordinate::GeoCoordinate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Human {
    pub id: AgentId,
    pub sex: Sex,
    pub birth_tick: Tick,
    pub age_ticks: u64,
    pub status: VitalStatus,
    pub spouse_id: Option<AgentId>,
    pub mother_id: Option<AgentId>,
    pub father_id: Option<AgentId>,
    pub children_ids: Vec<AgentId>,
    /// Spatial position in the world map
    pub location: GeoCoordinate,
    /// Calorie reserve (kcal) - baseline 2000 kcal needed per day (1 tick)
    pub calorie_reserve: f64,
    /// Hydration reserve percentage (0.0 to 100.0)
    pub hydration_reserve: f64,
    /// Consecutive days without sufficient calories
    pub days_starving: u32,
    /// Deterministic inventory order via BTreeMap
    pub inventory: BTreeMap<ItemId, u32>,
    /// Personal internal economic statistics and lagged market memory
    #[serde(default)]
    pub personal_stats: AgentPersonalStats,
    /// Flexible JSON attributes (e.g. education, health index, skills)
    pub attributes: serde_json::Value,
}

impl Human {
    pub fn new(id: AgentId, sex: Sex, birth_tick: Tick) -> Self {
        Self {
            id,
            sex,
            birth_tick,
            age_ticks: 0,
            status: VitalStatus::Alive,
            spouse_id: None,
            mother_id: None,
            father_id: None,
            children_ids: Vec::new(),
            location: GeoCoordinate::new(0, 0),
            calorie_reserve: 20000.0, // Initial 10-day buffer
            hydration_reserve: 100.0,
            days_starving: 0,
            inventory: BTreeMap::new(),
            personal_stats: AgentPersonalStats::new(),
            attributes: serde_json::json!({}),
        }
    }

    pub fn with_initial_age(mut self, age_ticks: u64) -> Self {
        self.age_ticks = age_ticks;
        self
    }

    pub fn with_item(mut self, item_id: ItemId, quantity: u32) -> Self {
        self.add_item(item_id, quantity);
        self
    }

    pub fn with_location(mut self, location: GeoCoordinate) -> Self {
        self.location = location;
        self
    }

    pub fn with_calories(mut self, calories: f64) -> Self {
        self.calorie_reserve = calories;
        self
    }

    pub fn with_parents(mut self, father: Option<AgentId>, mother: Option<AgentId>) -> Self {
        self.father_id = father;
        self.mother_id = mother;
        self
    }

    /// Evaluates subjective scarcity multiplier using agent's personal internal memory with lag
    pub fn scarcity_multiplier(&self, item_id: ItemId, current_tick: u64) -> f64 {
        self.personal_stats.get_scarcity_multiplier(item_id, current_tick)
    }

    /// Records a personal bilateral trade and updates subjective surplus and attributes
    pub fn record_personal_trade(&mut self, surplus: f64) {
        self.personal_stats.record_trade(surplus);
        if let Some(obj) = self.attributes.as_object_mut() {
            obj.insert("trade_count".to_string(), serde_json::json!(self.personal_stats.trade_count));
            obj.insert("cumulative_surplus".to_string(), serde_json::json!(self.personal_stats.cumulative_surplus));
        }
    }

    pub fn mark_deceased(&mut self, current_tick: Tick, reason: impl Into<String>) {
        let r = reason.into();
        self.status = VitalStatus::Deceased {
            tick_of_death: current_tick,
            reason: r.clone(),
        };
        if let Some(obj) = self.attributes.as_object_mut() {
            obj.insert("death_reason".to_string(), serde_json::json!(r));
            obj.insert("death_tick".to_string(), serde_json::json!(current_tick.0));
        }
    }

    pub fn death_tick(&self) -> Option<Tick> {
        match &self.status {
            VitalStatus::Alive => None,
            VitalStatus::Deceased { tick_of_death, .. } => Some(*tick_of_death),
        }
    }

    pub fn is_sick(&self) -> bool {
        self.attributes
            .get("is_sick")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    pub fn set_sick(&mut self, sick: bool) {
        if let Some(obj) = self.attributes.as_object_mut() {
            obj.insert("is_sick".to_string(), serde_json::json!(sick));
        }
    }

    /// Calculate total weight of carried physical items in inventory (excluding watercraft)
    pub fn inventory_weight_kg(&self) -> f64 {
        let mut total = 0.0;
        for (item_id, qty) in &self.inventory {
            if *item_id == ItemId::RAFT {
                continue; // Moored on water / vessel
            }
            total += item_id.default_weight_kg() * (*qty as f64);
        }
        total
    }

    /// Physical carrying capacity limit: base 25.0 kg, expanded by woven baskets (+25 kg each, up to 75 kg)
    /// and sedentary granary pottery jars (+50 kg each, up to 175 kg total)
    pub fn carrying_capacity_kg(&self) -> f64 {
        let base = 25.0;
        let baskets = self.inventory.get(&ItemId::WOVEN_BASKET).copied().unwrap_or(0);
        let jars = self.inventory.get(&ItemId::POTTERY_JAR).copied().unwrap_or(0);
        base + (baskets as f64 * 25.0).min(50.0) + (jars as f64 * 50.0).min(100.0)
    }

    pub fn can_carry_additional_weight(&self, additional_kg: f64) -> bool {
        self.inventory_weight_kg() + additional_kg <= self.carrying_capacity_kg()
    }
}

impl Identifiable for Human {
    fn id(&self) -> AgentId {
        self.id
    }
}

impl HasLifecycle for Human {
    fn birth_tick(&self) -> Tick {
        self.birth_tick
    }

    fn age_ticks(&self) -> u64 {
        self.age_ticks
    }

    fn is_alive(&self) -> bool {
        matches!(self.status, VitalStatus::Alive)
    }

    fn step_age(&mut self, tick_count: u64) {
        if self.is_alive() {
            self.age_ticks += tick_count;
        }
    }
}

impl SocialActor for Human {
    fn spouse_id(&self) -> Option<AgentId> {
        self.spouse_id
    }

    fn set_spouse(&mut self, spouse: Option<AgentId>) {
        self.spouse_id = spouse;
    }

    fn mother_id(&self) -> Option<AgentId> {
        self.mother_id
    }

    fn father_id(&self) -> Option<AgentId> {
        self.father_id
    }

    fn children(&self) -> &[AgentId] {
        &self.children_ids
    }

    fn add_child(&mut self, child: AgentId) {
        if !self.children_ids.contains(&child) {
            self.children_ids.push(child);
        }
    }
}

impl EconomicActor for Human {
    fn inventory(&self) -> &BTreeMap<ItemId, u32> {
        &self.inventory
    }

    fn inventory_mut(&mut self) -> &mut BTreeMap<ItemId, u32> {
        &mut self.inventory
    }

    fn add_item(&mut self, item_id: ItemId, quantity: u32) {
        *self.inventory.entry(item_id).or_insert(0) += quantity;
    }

    fn remove_item(&mut self, item_id: ItemId, quantity: u32) -> Result<(), &'static str> {
        let current = self.inventory.get_mut(&item_id).ok_or("Item not in inventory")?;
        if *current < quantity {
            return Err("Insufficient item quantity in inventory");
        }
        *current -= quantity;
        if *current == 0 {
            self.inventory.remove(&item_id);
        }
        Ok(())
    }
}
