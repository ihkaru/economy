use super::id::AgentId;
use crate::core::domain::time::Tick;
use std::collections::BTreeMap;
use crate::core::domain::item::id::ItemId;

pub trait Identifiable {
    fn id(&self) -> AgentId;
}

pub trait HasLifecycle {
    fn birth_tick(&self) -> Tick;
    fn age_ticks(&self) -> u64;
    fn is_alive(&self) -> bool;
    fn step_age(&mut self, tick_count: u64);
}

pub trait SocialActor {
    fn spouse_id(&self) -> Option<AgentId>;
    fn set_spouse(&mut self, spouse: Option<AgentId>);
    fn mother_id(&self) -> Option<AgentId>;
    fn father_id(&self) -> Option<AgentId>;
    fn children(&self) -> &[AgentId];
    fn add_child(&mut self, child: AgentId);
}

pub trait EconomicActor {
    fn inventory(&self) -> &BTreeMap<ItemId, u32>;
    fn inventory_mut(&mut self) -> &mut BTreeMap<ItemId, u32>;
    fn add_item(&mut self, item_id: ItemId, quantity: u32);
    fn remove_item(&mut self, item_id: ItemId, quantity: u32) -> Result<(), &'static str>;
    fn total_asset_count(&self) -> u32 {
        self.inventory().values().sum()
    }
    fn has_item(&self, item_id: ItemId) -> bool {
        self.inventory().get(&item_id).copied().unwrap_or(0) > 0
    }
}
