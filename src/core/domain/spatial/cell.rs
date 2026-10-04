use serde::{Deserialize, Serialize};
use super::coordinate::GeoCoordinate;
use super::terrain::TerrainType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCell {
    pub coord: GeoCoordinate,
    pub terrain: TerrainType,
    pub elevation_meters: f32,
    pub region_id: u32,
    pub attached_resource_nodes: Vec<u64>,
}

impl MapCell {
    pub fn new(coord: GeoCoordinate, terrain: TerrainType, elevation_meters: f32, region_id: u32) -> Self {
        Self {
            coord,
            terrain,
            elevation_meters,
            region_id,
            attached_resource_nodes: Vec::new(),
        }
    }

    pub fn attach_resource_node(&mut self, node_id: u64) {
        if !self.attached_resource_nodes.contains(&node_id) {
            self.attached_resource_nodes.push(node_id);
        }
    }
}
