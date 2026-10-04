use super::definition::{ItemCategory, ItemDefinition, ItemNature};
use super::id::ItemId;

/// Builds the canonical repertoire of economic items, capital goods, knowledge blueprints, and services
pub fn build_canonical_items() -> Vec<ItemDefinition> {
    vec![
        // Fase 1: Paleolithic Foraging & Immediate Return
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

        ItemDefinition::new(
            ItemId::TIMBER,
            "Kayu Gelondongan (Raw Timber)",
            ItemCategory::Good,
            serde_json::json!({
                "historical_era": "Paleolithic_Foraging",
                "unit": "batang",
                "utility_type": "RawMaterial",
                "description": "Bahan baku konstruksi, tiang rakit, dan bahan bakar penghangat tubuh"
            }),
        )
        .with_weight(5.0)
        .with_perishable(false),

        ItemDefinition::new(
            ItemId::SERVICE_LABOR,
            "Waktu Tenaga Kerja Fisik (Labor Hours)",
            ItemCategory::Service,
            serde_json::json!({
                "historical_era": "Paleolithic_Foraging",
                "unit": "man_hour",
                "utility_type": "LaborTime",
                "description": "Waktu dan tenaga biologis yang dicurahkan manusia untuk aktivitas produktif"
            }),
        )
        .with_weight(0.0)
        .with_perishable(false),

        ItemDefinition::new(
            ItemId::KNOWLEDGE_FIRE_MAKING,
            "Gagasan Menyalakan Api (Pyrotechnology Blueprint)",
            ItemCategory::Knowledge,
            serde_json::json!({
                "historical_era": "Paleolithic_Pyrotechnology",
                "unit": "idea",
                "utility_type": "Skill",
                "rivalry": "NonRival",
                "description": "Gagasan teknologi menghasilkan api melalui gesekan kayu atau batu pemantik"
            }),
        )
        .with_weight(0.0)
        .with_nature(ItemNature::NonRivalKnowledge),

        // Fase 2: Paleolithic Pyrotechnology & Hand-Axe
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

        // Fase 3: Mesolithic Aquatic Revolution & Basketry
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

        ItemDefinition::new(
            ItemId::HERBAL_MEDICINE,
            "Tanaman Obat Liar (Medicinal Herbs)",
            ItemCategory::Good,
            serde_json::json!({
                "historical_era": "Paleolithic_Foraging",
                "unit": "ikat",
                "utility_type": "Healthcare",
                "cures_disease": true,
                "description": "Tanaman herbal alami (seperti daun willow, mint liar, jahe hutan) untuk meredakan infeksi demam"
            }),
        )
        .with_weight(0.1)
        .with_perishable(false),

        ItemDefinition::new(
            ItemId::WOVEN_BASKET,
            "Keranjang Anyaman Wadah Angkut (Woven Carrying Basket)",
            ItemCategory::Good,
            serde_json::json!({
                "historical_era": "Mesolithic_Aquatic_Revolution",
                "unit": "buah",
                "utility_type": "CapitalTool",
                "capacity_expansion_kg": 25.0,
                "recipe": {"timber": 2},
                "description": "Wadah anyaman serat kayu/ranting: memperluas kapasitas angkut logistik (+25 kg)"
            }),
        )
        .with_weight(0.5)
        .with_perishable(false),

        ItemDefinition::new(
            ItemId::KNOWLEDGE_BASKET_WEAVING,
            "Gagasan Anyaman Wadah Angkut (Basket Weaving Blueprint)",
            ItemCategory::Knowledge,
            serde_json::json!({
                "historical_era": "Mesolithic_Aquatic_Revolution",
                "unit": "idea",
                "utility_type": "Skill",
                "rivalry": "NonRival",
                "description": "Gagasan teknologi anyaman serat alami untuk wadah penyimpanan dan angkut pangan"
            }),
        )
        .with_weight(0.0)
        .with_nature(ItemNature::NonRivalKnowledge),

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

        ItemDefinition::new(
            ItemId::RAFT,
            "Rakit Kayu Jelajah Maritim (Maritime Raft)",
            ItemCategory::Good,
            serde_json::json!({
                "historical_era": "Mesolithic_Aquatic_Revolution",
                "unit": "unit",
                "utility_type": "CapitalTool",
                "recipe": {"timber": 10},
                "description": "Alat transportasi air membuka akses ke pulau seberang dan tambang garam"
            }),
        )
        .with_weight(45.0)
        .with_perishable(false),

        ItemDefinition::new(
            ItemId::KNOWLEDGE_TOOL_CRAFTING,
            "Gagasan Rancang Bangun Alat (Tool Crafting Blueprint)",
            ItemCategory::Knowledge,
            serde_json::json!({
                "historical_era": "Mesolithic_Aquatic_Revolution",
                "unit": "idea",
                "utility_type": "Skill",
                "rivalry": "NonRival",
                "description": "Pengetahuan metodologis mengolah bahan baku menjadi barang modal berdaya guna"
            }),
        )
        .with_weight(0.0)
        .with_nature(ItemNature::NonRivalKnowledge),

        ItemDefinition::new(
            ItemId::SERVICE_TRANSPORT,
            "Jasa Penyeberangan Air (Water Ferry Service)",
            ItemCategory::Service,
            serde_json::json!({
                "historical_era": "Mesolithic_Aquatic_Revolution",
                "unit": "trip",
                "utility_type": "LaborTime",
                "description": "Jasa memindahkan orang atau barang melintasi perairan menggunakan rakit"
            }),
        )
        .with_weight(0.0)
        .with_perishable(false),

        // Fase 4: Neolithic Preservation & Storage
        ItemDefinition::new(
            ItemId::SALT,
            "Garam Kristal Mineral (Rock Salt)",
            ItemCategory::Good,
            serde_json::json!({
                "historical_era": "Neolithic_Preservation_Storage",
                "unit": "kg",
                "utility_type": "MediumOfExchange",
                "liquidity": "High",
                "description": "Komoditas berdaya tahan tinggi, pengawet ikan, dan media perantara barter"
            }),
        )
        .with_weight(0.5)
        .with_perishable(false),

        ItemDefinition::new(
            ItemId::KNOWLEDGE_FISH_CURING,
            "Gagasan Pengawetan & Pengasinan Ikan (Curing Blueprint)",
            ItemCategory::Knowledge,
            serde_json::json!({
                "historical_era": "Neolithic_Preservation_Storage",
                "unit": "idea",
                "utility_type": "Skill",
                "rivalry": "NonRival",
                "description": "Gagasan teknologi memperpanjang daya simpan protein hewani menggunakan garam"
            }),
        )
        .with_weight(0.0)
        .with_nature(ItemNature::NonRivalKnowledge),

        // Fase 5: Neolithic Division of Labor & Services
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

        ItemDefinition::new(
            ItemId::SERVICE_MEDICAL,
            "Jasa Perawatan & Pemulihan Sakit (Caregiving & Healing)",
            ItemCategory::Service,
            serde_json::json!({
                "historical_era": "Neolithic_Division_Of_Labor",
                "unit": "treatment",
                "utility_type": "LaborTime",
                "description": "Jasa merawat agen lapar atau sakit untuk memulihkan kesehatan dan mencegah kematian"
            }),
        )
        .with_weight(0.0)
        .with_perishable(false),

        ItemDefinition::new(
            ItemId::KNOWLEDGE_HERBAL_MEDICINE,
            "Gagasan Ramuan Obat Tradisional (Herbal Medicine Blueprint)",
            ItemCategory::Knowledge,
            serde_json::json!({
                "historical_era": "Neolithic_Division_Of_Labor",
                "unit": "idea",
                "utility_type": "Skill",
                "rivalry": "NonRival",
                "description": "Gagasan teknologi identifikasi flora obat dan peracikan ramuan untuk menyembuhkan penyakit"
            }),
        )
        .with_weight(0.0)
        .with_nature(ItemNature::NonRivalKnowledge),

        // Fase 6: Proto-Historic Currency & CPR Institutions
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
    ]
}
