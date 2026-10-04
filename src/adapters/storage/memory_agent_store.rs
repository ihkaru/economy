use std::collections::BTreeMap;
use crate::core::domain::agent::human::Human;
use crate::core::domain::agent::id::AgentId;
use crate::core::domain::agent::traits::HasLifecycle;
use crate::core::ports::agent_store::AgentStorePort;

#[derive(Default)]
pub struct MemoryAgentStore {
    agents: BTreeMap<AgentId, Human>,
}

impl MemoryAgentStore {
    pub fn new() -> Self {
        Self {
            agents: BTreeMap::new(),
        }
    }
}

impl AgentStorePort for MemoryAgentStore {
    fn get_human(&self, id: AgentId) -> Option<&Human> {
        self.agents.get(&id)
    }

    fn get_human_mut(&mut self, id: AgentId) -> Option<&mut Human> {
        self.agents.get_mut(&id)
    }

    fn insert_human(&mut self, human: Human) {
        self.agents.insert(human.id, human);
    }

    fn all_human_ids(&self) -> Vec<AgentId> {
        self.agents.keys().copied().collect()
    }

    fn living_human_ids(&self) -> Vec<AgentId> {
        self.agents.values().filter(|a| a.is_alive()).map(|a| a.id).collect()
    }

    fn count_alive(&self) -> usize {
        self.agents.values().filter(|a| a.is_alive()).count()
    }

    fn count_total(&self) -> usize {
        self.agents.len()
    }

    fn get_all_humans(&self) -> Vec<Human> {
        self.agents.values().cloned().collect()
    }

    fn iter_humans(&self) -> Box<dyn Iterator<Item = &Human> + '_> {
        Box::new(self.agents.values())
    }

    fn iter_living_humans(&self) -> Box<dyn Iterator<Item = &Human> + '_> {
        Box::new(self.agents.values().filter(|a| a.is_alive()))
    }
}
