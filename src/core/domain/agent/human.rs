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

    pub fn mark_deceased(&mut self, current_tick: Tick, reason: impl Into<String>) {
        self.status = VitalStatus::Deceased {
            tick_of_death: current_tick,
            reason: reason.into(),
        };
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
