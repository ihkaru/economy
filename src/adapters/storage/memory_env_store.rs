use crate::core::domain::environment::climate::ClimateState;
use crate::core::domain::environment::resource::ResourceNode;
use crate::core::domain::spatial::WorldMap;
use crate::core::ports::environment_store::EnvironmentStorePort;

pub struct MemoryEnvironmentStore {
    climate: ClimateState,
    nodes: Vec<ResourceNode>,
    world_map: Option<WorldMap>,
}

impl MemoryEnvironmentStore {
    pub fn new(climate: ClimateState, nodes: Vec<ResourceNode>) -> Self {
        Self {
            climate,
            nodes,
            world_map: None,
        }
    }

    pub fn with_world_map(mut self, map: WorldMap) -> Self {
        self.world_map = Some(map);
        self
    }
}

impl EnvironmentStorePort for MemoryEnvironmentStore {
    fn climate(&self) -> &ClimateState {
        &self.climate
    }

    fn climate_mut(&mut self) -> &mut ClimateState {
        &mut self.climate
    }

    fn nodes(&self) -> &[ResourceNode] {
        &self.nodes
    }

    fn nodes_mut(&mut self) -> &mut [ResourceNode] {
        &mut self.nodes
    }

    fn add_node(&mut self, node: ResourceNode) {
        self.nodes.push(node);
    }

    fn get_node_mut(&mut self, id: u64) -> Option<&mut ResourceNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    fn world_map(&self) -> Option<&WorldMap> {
        self.world_map.as_ref()
    }
}
