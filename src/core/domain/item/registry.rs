use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

use super::definition::{ItemCategory, ItemDefinition, ItemNature};
use super::id::ItemId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRegistry {
    items: BTreeMap<ItemId, ItemDefinition>,
}

impl ItemRegistry {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, def: ItemDefinition) {
        self.items.insert(def.id, def);
    }

    pub fn get(&self, id: ItemId) -> Option<&ItemDefinition> {
        self.items.get(&id)
    }

    pub fn all(&self) -> Vec<&ItemDefinition> {
        self.items.values().collect()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Canonical master item catalog reflecting historical economic emergence
    /// from Paleolithic immediate return to Neolithic specialization, services, and proto-money
    pub fn canonical() -> Self {
        let mut reg = Self::new();

        // -------------------------------------------------------------------------
        // Fase 1: Paleolithic Foraging & Immediate Return (Survival Base)
        // -------------------------------------------------------------------------
        reg.register(
            ItemDefinition::new(
                ItemId::BERRIES,
                "Buah Beri Liar (Wild Berries)",
                ItemCategory::Good,
                serde_json::json!({
                    "historical_era": "Paleolithic_Foraging",
                    "unit": "kg",
                    "utility_type": "Nutrition",
                    "calories_per_unit": 300.0,
                    "shelf_life_days": 7,
                    "description": "Pangan segar cepat saji dari semak belukar liar, mudah busuk"
                }),
            )
            .with_weight(0.2)
            .with_perishable(true),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::FISH,
                "Ikan Segar (Fresh Fish)",
                ItemCategory::Good,
                serde_json::json!({
                    "historical_era": "Paleolithic_Foraging",
                    "unit": "ekor",
                    "utility_type": "Nutrition",
                    "calories_per_unit": 500.0,
                    "shelf_life_days": 3,
                    "description": "Sumber protein hewani air tawar, membutuhkan konsumsi segera sebelum busuk"
                }),
            )
            .with_weight(0.5)
            .with_perishable(true),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::GRAIN,
                "Biji Gandum Liar (Wild Grain)",
                ItemCategory::Good,
                serde_json::json!({
                    "historical_era": "Paleolithic_Foraging",
                    "unit": "kg",
                    "utility_type": "Nutrition",
                    "calories_per_unit": 800.0,
                    "shelf_life_days": 365,
                    "description": "Pangan pokok berkarbohidrat padat, kering dan tahan simpan lama"
                }),
            )
            .with_weight(1.0)
            .with_perishable(false),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::TIMBER,
                "Kayu Gelondongan (Raw Timber)",
                ItemCategory::Good,
                serde_json::json!({
                    "historical_era": "Paleolithic_Foraging",
                    "unit": "batang",
                    "utility_type": "RawMaterial",
                    "fuel_value_kcal": 2500.0,
                    "description": "Bahan baku konstruksi, tiang rakit, dan bahan bakar penghangat tubuh"
                }),
            )
            .with_weight(5.0)
            .with_perishable(false),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::SERVICE_LABOR,
                "Waktu Tenaga Kerja Fisik (Labor Hours)",
                ItemCategory::Service,
                serde_json::json!({
                    "historical_era": "Paleolithic_Foraging",
                    "unit": "man_hour",
                    "utility_type": "LaborTime",
                    "energy_cost_kcal": 300.0,
                    "description": "Alokasi waktu kerja fisik manusia untuk panen, gotong-royong, atau angkut beban"
                }),
            )
            .with_weight(0.0)
            .with_perishable(false),
        );

        // -------------------------------------------------------------------------
        // Fase 2: Paleolithic Pyrotechnology & Lithic Tools (First Capital Goods)
        // -------------------------------------------------------------------------
        reg.register(
            ItemDefinition::new(
                ItemId::KNOWLEDGE_FIRE_MAKING,
                "Gagasan Menyalakan Api (Pyrotechnology Blueprint)",
                ItemCategory::Knowledge,
                serde_json::json!({
                    "historical_era": "Paleolithic_Pyrotechnology",
                    "unit": "idea",
                    "utility_type": "Skill",
                    "rivalry": "NonRival",
                    "description": "Gagasan teknologi pembuatan api untuk memasak dan termoregulasi dingin"
                }),
            )
            .with_weight(0.0)
            .with_nature(ItemNature::NonRivalKnowledge),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::STONE_AXE,
                "Kapak Batu Genggam (Stone Hand-Axe)",
                ItemCategory::Good,
                serde_json::json!({
                    "historical_era": "Paleolithic_Pyrotechnology",
                    "unit": "buah",
                    "utility_type": "CapitalTool",
                    "harvest_multiplier": 3.0,
                    "target_resource": "Timber",
                    "recipe": {"timber": 5},
                    "description": "Barang modal purba pertama: meningkatkan efisiensi tebang kayu 300%"
                }),
            )
            .with_weight(2.5)
            .with_perishable(false),
        );

        // -------------------------------------------------------------------------
        // Fase 3: Mesolithic Aquatic & Navigation Revolution (Water Conquest)
        // -------------------------------------------------------------------------
        reg.register(
            ItemDefinition::new(
                ItemId::KNOWLEDGE_TOOL_CRAFTING,
                "Gagasan Rancang Bangun Alat (Tool Crafting Blueprint)",
                ItemCategory::Knowledge,
                serde_json::json!({
                    "historical_era": "Mesolithic_Aquatic_Revolution",
                    "unit": "idea",
                    "utility_type": "Skill",
                    "rivalry": "NonRival",
                    "description": "Gagasan perakitan alat modal dari kayu dan batu"
                }),
            )
            .with_weight(0.0)
            .with_nature(ItemNature::NonRivalKnowledge),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::FISHING_NET,
                "Jaring Ikan Anyaman (Woven Fishing Net)",
                ItemCategory::Good,
                serde_json::json!({
                    "historical_era": "Mesolithic_Aquatic_Revolution",
                    "unit": "set",
                    "utility_type": "CapitalTool",
                    "harvest_multiplier": 3.0,
                    "target_resource": "Fish",
                    "recipe": {"timber": 4},
                    "description": "Barang modal penangkap ikan perairan: melipatgandakan panen protein 300%"
                }),
            )
            .with_weight(1.5)
            .with_perishable(false),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::KNOWLEDGE_RAFT_BUILDING,
                "Gagasan Konstruksi Rakit (Maritime Raft Blueprint)",
                ItemCategory::Knowledge,
                serde_json::json!({
                    "historical_era": "Mesolithic_Aquatic_Revolution",
                    "unit": "idea",
                    "utility_type": "Skill",
                    "rivalry": "NonRival",
                    "description": "Gagasan teknologi penyeberangan laut dalam dan navigasi pulau"
                }),
            )
            .with_weight(0.0)
            .with_nature(ItemNature::NonRivalKnowledge),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::RAFT,
                "Rakit Kayu Jelajah Maritim (Maritime Raft)",
                ItemCategory::Good,
                serde_json::json!({
                    "historical_era": "Mesolithic_Aquatic_Revolution",
                    "unit": "unit",
                    "utility_type": "CapitalTool",
                    "enables_deep_ocean": true,
                    "recipe": {"timber": 10},
                    "description": "Alat transportasi air membuka akses ke pulau seberang dan tambang garam"
                }),
            )
            .with_weight(45.0)
            .with_perishable(false),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::SERVICE_TRANSPORT,
                "Jasa Penyeberangan Air (Water Ferry Service)",
                ItemCategory::Service,
                serde_json::json!({
                    "historical_era": "Mesolithic_Aquatic_Revolution",
                    "unit": "trip",
                    "utility_type": "LaborTime",
                    "required_tool": "Raft",
                    "description": "Jasa menyeberangkan agen tanpa rakit melintasi sungai atau laut dalam"
                }),
            )
            .with_weight(0.0)
            .with_perishable(false),
        );

        // -------------------------------------------------------------------------
        // Fase 4: Neolithic Preservation & Storage Revolution (Capital Accumulation)
        // -------------------------------------------------------------------------
        reg.register(
            ItemDefinition::new(
                ItemId::SALT,
                "Garam Kristal Mineral (Rock Salt)",
                ItemCategory::Good,
                serde_json::json!({
                    "historical_era": "Neolithic_Preservation_Storage",
                    "unit": "kg",
                    "utility_type": "MediumOfExchange",
                    "preservative": true,
                    "digestive_bonus_pct": 10.0,
                    "description": "Komoditas berdaya tahan tinggi, pengawet ikan, dan media perantara barter"
                }),
            )
            .with_weight(0.5)
            .with_perishable(false),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::KNOWLEDGE_FISH_CURING,
                "Gagasan Pengawetan & Pengasinan Ikan (Curing Blueprint)",
                ItemCategory::Knowledge,
                serde_json::json!({
                    "historical_era": "Neolithic_Preservation_Storage",
                    "unit": "idea",
                    "utility_type": "Skill",
                    "rivalry": "NonRival",
                    "description": "Gagasan teknologi mengawetkan protein mudah busuk menjadi aset tahan lama"
                }),
            )
            .with_weight(0.0)
            .with_nature(ItemNature::NonRivalKnowledge),
        );

        // -------------------------------------------------------------------------
        // Fase 5: Neolithic Division of Labor, Specialization & Services
        // -------------------------------------------------------------------------
        reg.register(
            ItemDefinition::new(
                ItemId::SERVICE_EDUCATION,
                "Jasa Pendidikan & Bimbingan Magang (Apprenticeship Tutoring)",
                ItemCategory::Service,
                serde_json::json!({
                    "historical_era": "Neolithic_Division_Of_Labor",
                    "unit": "session",
                    "utility_type": "Skill",
                    "description": "Waktu kerja guru untuk mentransfer pengetahuan non-rival kepada murid"
                }),
            )
            .with_weight(0.0)
            .with_perishable(false),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::SERVICE_MEDICAL,
                "Jasa Perawatan & Pemulihan Sakit (Caregiving & Healing)",
                ItemCategory::Service,
                serde_json::json!({
                    "historical_era": "Neolithic_Division_Of_Labor",
                    "unit": "treatment",
                    "utility_type": "LaborTime",
                    "description": "Jasa merawat agen lapar atau terluka untuk mencegah kematian"
                }),
            )
            .with_weight(0.0)
            .with_perishable(false),
        );

        // -------------------------------------------------------------------------
        // Fase 6: Proto-Historic Proto-Money & Institutional Property Regimes
        // -------------------------------------------------------------------------
        reg.register(
            ItemDefinition::new(
                ItemId::SHELLS,
                "Cangkang Kerang Cowrie (Cowrie Shells)",
                ItemCategory::Currency,
                serde_json::json!({
                    "historical_era": "Proto_Historic_Currency",
                    "unit": "biji",
                    "utility_type": "MediumOfExchange",
                    "liquidity": "High",
                    "description": "Uang komoditas purba: ringan, seragam, tahan lama, dan diterima luas"
                }),
            )
            .with_weight(0.05)
            .with_perishable(false),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::PERMIT_FISHING_RIGHT,
                "Izin Hak Akses Perikanan (Fishing Access Right)",
                ItemCategory::Permit,
                serde_json::json!({
                    "historical_era": "Proto_Historic_Currency",
                    "unit": "concession",
                    "utility_type": "InstitutionalRight",
                    "description": "Hak institusional pemanfaatan sumber daya perairan bersama (CPR regime)"
                }),
            )
            .with_weight(0.0)
            .with_nature(ItemNature::InstitutionalRight),
        );

        reg.register(
            ItemDefinition::new(
                ItemId::PERMIT_FORESTRY_RIGHT,
                "Izin Konsesi Pemanfaatan Hutan (Forestry Concession)",
                ItemCategory::Permit,
                serde_json::json!({
                    "historical_era": "Proto_Historic_Currency",
                    "unit": "concession",
                    "utility_type": "InstitutionalRight",
                    "description": "Hak institusional penebangan kayu pada zona hutan adat tertentu"
                }),
            )
            .with_weight(0.0)
            .with_nature(ItemNature::InstitutionalRight),
        );

        reg
    }
}

impl Default for ItemRegistry {
    fn default() -> Self {
        Self::canonical()
    }
}
