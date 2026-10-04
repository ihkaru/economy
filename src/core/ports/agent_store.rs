use crate::core::domain::agent::human::Human;
use crate::core::domain::agent::id::AgentId;

pub trait AgentStorePort: Send + Sync {
    fn get_human(&self, id: AgentId) -> Option<&Human>;
    fn get_human_mut(&mut self, id: AgentId) -> Option<&mut Human>;
    fn insert_human(&mut self, human: Human);
    fn all_human_ids(&self) -> Vec<AgentId>;
    fn living_human_ids(&self) -> Vec<AgentId>;
    fn count_alive(&self) -> usize;
    fn count_total(&self) -> usize;
    fn get_all_humans(&self) -> Vec<Human>;
    fn iter_humans(&self) -> Box<dyn Iterator<Item = &Human> + '_>;
    fn iter_living_humans(&self) -> Box<dyn Iterator<Item = &Human> + '_>;
}
