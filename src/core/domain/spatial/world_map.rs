use serde::{Deserialize, Serialize};
use super::cell::MapCell;
use super::coordinate::GeoCoordinate;
use super::terrain::TerrainType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<MapCell>,
}

impl WorldMap {
    pub fn new(width: u32, height: u32, cells: Vec<MapCell>) -> Self {
        assert_eq!(cells.len(), (width * height) as usize, "Cells count must match width * height");
        Self { width, height, cells }
    }

    pub fn index_of(&self, coord: GeoCoordinate) -> Option<usize> {
        if coord.x < self.width && coord.y < self.height {
            Some((coord.y * self.width + coord.x) as usize)
        } else {
            None
        }
    }

    pub fn get_cell(&self, coord: GeoCoordinate) -> Option<&MapCell> {
        self.index_of(coord).and_then(|idx| self.cells.get(idx))
    }

    pub fn get_cell_mut(&mut self, coord: GeoCoordinate) -> Option<&mut MapCell> {
        self.index_of(coord).and_then(|idx| self.cells.get_mut(idx))
    }

    /// Evaluates travel friction between two adjacent coordinates
    pub fn travel_friction(&self, target_coord: GeoCoordinate, can_traverse_water: bool) -> f64 {
        if let Some(cell) = self.get_cell(target_coord) {
            match cell.terrain {
                TerrainType::DeepOcean => {
                    if can_traverse_water {
                        1.2 // Smooth sailing with raft or vessel
                    } else {
                        f64::INFINITY // Impassable without vessel
                    }
                }
                terrain => terrain.movement_friction(),
            }
        } else {
            f64::INFINITY
        }
    }

    /// Generates a realistic continental landmass with a coastal ocean barrier and offshore archipelago
    pub fn generate_continent_and_archipelago(width: u32, height: u32) -> Self {
        let mut cells = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let coord = GeoCoordinate::new(x, y);

                // Mainland is on the left (x < width * 6 / 10)
                // Ocean belt is in the middle (width * 6 / 10 <= x < width * 8 / 10)
                // Archipelago is on the right (x >= width * 8 / 10)
                let (terrain, elevation, region_id) = if x < (width * 6) / 10 {
                    // Region 1: The Great Mainland
                    if y < height / 5 {
                        (TerrainType::Mountain, 1800.0, 1) // Northern Mountain Ridge
                    } else if y == height / 2 || (y == (height / 2) + 1 && x > 5) {
                        (TerrainType::River, 50.0, 1) // Central River Valley
                    } else if y > (height * 7) / 10 {
                        (TerrainType::Forest, 200.0, 1) // Dense Southern Forest
                    } else if x == ((width * 6) / 10) - 1 {
                        (TerrainType::ShallowWater, 5.0, 1) // Coastal Estuary
                    } else {
                        (TerrainType::Plains, 100.0, 1) // Fertile Arable Plains
                    }
                } else if x < (width * 8) / 10 {
                    // Region 2: The Deep Ocean Barrier
                    (TerrainType::DeepOcean, 0.0, 2)
                } else {
                    // Region 3: The Crescent Archipelago (Isolated offshore islands)
                    let island_y_centers = [height / 4, height / 2, (height * 3) / 4];
                    let is_island = island_y_centers.iter().any(|&cy| {
                        let dy = y.abs_diff(cy);
                        let dx = x.abs_diff((width * 9) / 10);
                        dx * dx + dy * dy <= 4
                    });

                    if is_island {
                        if x == (width * 9) / 10 && y == height / 2 {
                            (TerrainType::Mountain, 950.0, 3) // Volcanic Peak (Rich in minerals/salt)
                        } else {
                            (TerrainType::Forest, 80.0, 3) // Tropical Island Woods
                        }
                    } else {
                        (TerrainType::DeepOcean, 0.0, 2)
                    }
                };

                cells.push(MapCell::new(coord, terrain, elevation, region_id));
            }
        }

        Self::new(width, height, cells)
    }
}
