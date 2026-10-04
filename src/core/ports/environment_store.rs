use crate::core::domain::environment::climate::ClimateState;
use crate::core::domain::environment::resource::ResourceNode;

pub trait EnvironmentStorePort: Send + Sync {
    fn climate(&self) -> &ClimateState;
    fn climate_mut(&mut self) -> &mut ClimateState;
    fn nodes(&self) -> &[ResourceNode];
    fn nodes_mut(&mut self) -> &mut [ResourceNode];
    fn add_node(&mut self, node: ResourceNode);
    fn get_node_mut(&mut self, id: u64) -> Option<&mut ResourceNode>;
    fn world_map(&self) -> Option<&crate::core::domain::spatial::WorldMap> {
        None
    }
}

