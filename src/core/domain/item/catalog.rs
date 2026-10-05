use super::definition::{InteractionRole, ItemCategory, ItemDefinition, ItemNature};
use super::id::ItemId;

fn def(
    id: ItemId,
    name: &str,
    cat: ItemCategory,
    role: InteractionRole,
    era: &str,
    unit: &str,
    util: &str,
    desc: &str,
    wt: f64,
    perish: bool,
) -> ItemDefinition {
    let mut item = ItemDefinition::new(
        id,
        name,
        cat,
        serde_json::json!({
            "historical_era": era,
            "unit": unit,
            "utility_type": util,
            "description": desc,
        }),
    )
    .with_weight(wt)
    .with_perishable(perish)
    .with_interaction_role(role);

    if cat == ItemCategory::Knowledge {
        item = item.with_nature(ItemNature::NonRivalKnowledge);
    } else if cat == ItemCategory::Permit {
        item = item.with_nature(ItemNature::InstitutionalRight);
    }
    item
}

/// Builds the canonical repertoire of economic items, capital goods, knowledge blueprints, and services
pub fn build_canonical_items() -> Vec<ItemDefinition> {
    vec![
        // Fase 1: Paleolithic Foraging & Immediate Return
        def(ItemId::BERRIES, "Buah Beri Liar (Wild Berries)", ItemCategory::Good, InteractionRole::Sustenance, "Paleolithic_Foraging", "kg", "Nutrition", "Pangan segar cepat saji dari semak belukar liar, mudah busuk", 0.2, true),
        def(ItemId::FISH, "Ikan Segar (Fresh Fish)", ItemCategory::Good, InteractionRole::Sustenance, "Paleolithic_Foraging", "ekor", "Nutrition", "Sumber protein hewani air tawar, konsumsi segera", 0.5, true),
        def(ItemId::GRAIN, "Biji Gandum Liar (Wild Grain)", ItemCategory::Good, InteractionRole::Sustenance, "Paleolithic_Foraging", "kg", "Nutrition", "Pangan pokok berkarbohidrat padat, kering dan tahan simpan", 1.0, false),
        def(ItemId::TIMBER, "Kayu Gelondongan (Raw Timber)", ItemCategory::Good, InteractionRole::RawInputMaterial, "Paleolithic_Foraging", "batang", "RawMaterial", "Bahan baku konstruksi, tiang rakit, dan bahan bakar penghangat", 5.0, false),
        def(ItemId::RAW_MEAT, "Daging Satwa Buruan (Fresh Wild Meat)", ItemCategory::Good, InteractionRole::Sustenance, "Paleolithic_Foraging", "kg", "Nutrition", "Protein hewani padat hasil perburuan fauna darat", 0.5, true),
        def(ItemId::HERBAL_MEDICINE, "Tanaman Obat Liar (Medicinal Herbs)", ItemCategory::Good, InteractionRole::Sustenance, "Paleolithic_Foraging", "ikat", "Healthcare", "Flora liar dengan kandungan antimikroba alami untuk menyembuhkan demam", 0.1, false),
        def(ItemId::SERVICE_LABOR, "Waktu Tenaga Kerja Fisik (Labor Hours)", ItemCategory::Service, InteractionRole::ProductionCapital, "Paleolithic_Foraging", "man_hour", "LaborTime", "Waktu dan tenaga biologis yang dicurahkan manusia untuk aktivitas produktif", 0.0, false),

        // Fase 2: Paleolithic Pyrotechnology & Lithic Reduction
        def(ItemId::STONE, "Batu Kali Keras (Hard River Stone)", ItemCategory::Good, InteractionRole::RawInputMaterial, "Paleolithic_Pyrotechnology", "kg", "RawMaterial", "Batu kali keras untuk bahan baku perkakas batu dan pemantik api", 1.0, false),
        def(ItemId::LITHIC_FLAKE, "Bilah Batu Serpih (Knapped Lithic Flake)", ItemCategory::Good, InteractionRole::ProductionCapital, "Paleolithic_Pyrotechnology", "buah", "CapitalTool", "Bilah serpihan batu tajam hasil pemangkasan litik untuk menguliti dan mata tombak", 0.2, false),
        def(ItemId::STONE_AXE, "Kapak Batu Genggam (Stone Hand-Axe)", ItemCategory::Good, InteractionRole::ProductionCapital, "Paleolithic_Pyrotechnology", "buah", "CapitalTool", "Barang modal purba pertama: meningkatkan efisiensi tebang kayu 300%", 2.5, false),
        def(ItemId::HUNTING_SPEAR, "Tombak Berburu Litik (Lithic Hunting Spear)", ItemCategory::Good, InteractionRole::ProductionCapital, "Paleolithic_Pyrotechnology", "buah", "CapitalTool", "Alat berburu satwa darat: melipatgandakan panen daging 300% dan menurunkan risiko", 1.5, false),
        def(ItemId::RAW_HIDE, "Kulit Binatang Mentah (Animal Raw Hide)", ItemCategory::Good, InteractionRole::RawInputMaterial, "Paleolithic_Pyrotechnology", "lembar", "RawMaterial", "Kulit hewan segar hasil sampingan perburuan fauna darat", 1.0, false),
        def(ItemId::ANIMAL_BONE, "Tulang Satwa Buruan (Animal Bone)", ItemCategory::Good, InteractionRole::RawInputMaterial, "Paleolithic_Pyrotechnology", "ruas", "RawMaterial", "Sisa kerangka hewan buruan bernilai osteologi untuk jarum dan gurdi", 0.5, false),
        def(ItemId::BONE_NEEDLE, "Jarum Tulang Halus (Eyed Bone Needle)", ItemCategory::Good, InteractionRole::ProductionCapital, "Paleolithic_Pyrotechnology", "buah", "CapitalTool", "Alat penusuk dan penjahit kulit binatang untuk membuat pakaian hangat pas badan", 0.05, false),
        def(ItemId::LEATHER_CLOTHING, "Pakaian Kulit Hangat (Warm Leather Garment)", ItemCategory::Good, InteractionRole::SocialStatusAndGifting, "Paleolithic_Pyrotechnology", "set", "CapitalTool", "Pakaian jahitan kulit hewan pelindung hipotermia dan simbol kematangan ekonomi", 1.5, false),
        def(ItemId::SMOKED_MEAT, "Dendeng Daging Asap (Wood-Smoked Preserved Meat)", ItemCategory::Good, InteractionRole::Sustenance, "Paleolithic_Pyrotechnology", "kg", "Nutrition", "Daging satwa yang diasapi piroteknologi agar tahan simpan lama", 0.4, false),
        def(ItemId::SMOKED_FISH, "Ikan Asap Kayu (Wood-Smoked Preserved Fish)", ItemCategory::Good, InteractionRole::Sustenance, "Paleolithic_Pyrotechnology", "ekor", "Nutrition", "Ikan asap antimikroba tahan simpan hasil pengolahan piroteknologi", 0.4, false),
        def(ItemId::DRIED_BERRIES, "Buah Beri Kering (Sun-Dried Desiccated Berries)", ItemCategory::Good, InteractionRole::Sustenance, "Paleolithic_Pyrotechnology", "kg", "Nutrition", "Buah beri yang diawetkan melalui dehidrasi sinar matahari", 0.1, false),
        def(ItemId::KNOWLEDGE_FIRE_MAKING, "Gagasan Menyalakan Api (Pyrotechnology Blueprint)", ItemCategory::Knowledge, InteractionRole::ProductionCapital, "Paleolithic_Pyrotechnology", "idea", "Skill", "Gagasan teknologi menghasilkan api melalui gesekan kayu atau pemantik", 0.0, false),
        def(ItemId::KNOWLEDGE_LEATHER_WORKING, "Gagasan Pengolahan Kulit (Leather Working Blueprint)", ItemCategory::Knowledge, InteractionRole::ProductionCapital, "Paleolithic_Pyrotechnology", "idea", "Skill", "Gagasan teknik penyamakan, pembersihan lemak, dan penjahitan kulit", 0.0, false),

        // Fase 3: Mesolithic Aquatic Revolution & Logistics
        def(ItemId::FISHING_NET, "Jaring Ikan Anyaman (Woven Fishing Net)", ItemCategory::Good, InteractionRole::ProductionCapital, "Mesolithic_Aquatic_Revolution", "set", "CapitalTool", "Barang modal penangkap ikan perairan: melipatgandakan panen protein 300%", 1.5, false),
        def(ItemId::WOVEN_BASKET, "Keranjang Anyaman Wadah Angkut (Woven Carrying Basket)", ItemCategory::Good, InteractionRole::ProductionCapital, "Mesolithic_Aquatic_Revolution", "buah", "CapitalTool", "Barang modal logistik: memperluas kapasitas angkut ransel agen hingga 45 kg", 0.5, false),
        def(ItemId::RAFT, "Rakit Kayu Jelajah Maritim (Maritime Raft)", ItemCategory::Good, InteractionRole::ProductionCapital, "Mesolithic_Aquatic_Revolution", "unit", "CapitalTool", "Alat transportasi air perintis untuk ekspedisi kepulauan dan penyeberangan", 45.0, false),
        def(ItemId::SERVICE_TRANSPORT, "Jasa Penyeberangan Air (Water Ferry Service)", ItemCategory::Service, InteractionRole::ProductionCapital, "Mesolithic_Aquatic_Revolution", "trip", "LaborTime", "Layanan penyeberangan sungai dan selat menggunakan armada rakit kayu", 0.0, false),
        def(ItemId::KNOWLEDGE_TOOL_CRAFTING, "Gagasan Rancang Bangun Alat (Tool Crafting Blueprint)", ItemCategory::Knowledge, InteractionRole::ProductionCapital, "Mesolithic_Aquatic_Revolution", "idea", "Skill", "Pengetahuan teknik pemangkasan litik, pengasahan kapak, dan perakitan tombak", 0.0, false),
        def(ItemId::KNOWLEDGE_BASKET_WEAVING, "Gagasan Anyaman Wadah Angkut (Basket Weaving Blueprint)", ItemCategory::Knowledge, InteractionRole::ProductionCapital, "Mesolithic_Aquatic_Revolution", "idea", "Skill", "Pengetahuan teknik menganyam serat kayu lentur menjadi wadah dan jaring", 0.0, false),
        def(ItemId::KNOWLEDGE_RAFT_BUILDING, "Gagasan Konstruksi Rakit (Maritime Raft Blueprint)", ItemCategory::Knowledge, InteractionRole::ProductionCapital, "Mesolithic_Aquatic_Revolution", "idea", "Skill", "Pengetahuan hidrodinamika kayu apung dan pengikatan balok rakit maritim", 0.0, false),

        // Fase 4: Neolithic Sedentary, Storage & Value Chain
        def(ItemId::CLAY, "Tanah Liat Halus Aluvial (Fine Alluvial Clay)", ItemCategory::Good, InteractionRole::RawInputMaterial, "Neolithic_Sedentary_Revolution", "kg", "RawMaterial", "Bahan mentah keramik dan media tulis inskripsi tablet utang", 0.5, false),
        def(ItemId::POTTERY_JAR, "Tempayan Gerabah Keramik (Ceramic Storage Pottery Jar)", ItemCategory::Good, InteractionRole::ProductionCapital, "Neolithic_Sedentary_Revolution", "buah", "CapitalTool", "Barang modal penyimpanan: wadah kedap air dan cikal bakal lumbung pangan", 4.0, false),
        def(ItemId::SALT, "Garam Kristal Mineral (Rock Salt)", ItemCategory::Good, InteractionRole::MediumAndCollateral, "Neolithic_Preservation_Storage", "kg", "MediumOfExchange", "Mineral pengawet makanan sekaligus komoditas bernilai tinggi", 0.5, false),
        def(ItemId::CURED_MEAT, "Dendeng Daging Asin (Salt-Cured Preserved Meat)", ItemCategory::Good, InteractionRole::Sustenance, "Neolithic_Preservation_Storage", "kg", "Nutrition", "Daging satwa yang diawetkan dengan garam murni tanpa pembusukan", 0.4, false),
        def(ItemId::CURED_FISH, "Ikan Kering Asin (Salt-Cured Preserved Fish)", ItemCategory::Good, InteractionRole::Sustenance, "Neolithic_Preservation_Storage", "ekor", "Nutrition", "Ikan air tawar yang diawetkan dengan kristal garam", 0.4, false),
        def(ItemId::KNOWLEDGE_FISH_CURING, "Gagasan Pengawetan & Pengasinan Ikan (Curing Blueprint)", ItemCategory::Knowledge, InteractionRole::ProductionCapital, "Neolithic_Preservation_Storage", "idea", "Skill", "Gagasan teknologi desikasi osmotik menggunakan mineral garam", 0.0, false),
        def(ItemId::KNOWLEDGE_POTTERY_MAKING, "Gagasan Pembuatan Gerabah Keramik (Pottery Blueprint)", ItemCategory::Knowledge, InteractionRole::ProductionCapital, "Neolithic_Sedentary_Revolution", "idea", "Skill", "Gagasan teknologi pembentukan dan pembakaran tanah liat bersuhu tinggi", 0.0, false),
        def(ItemId::SADDLE_QUERN, "Batu Gilang Penggiling (Saddle Quern Stone)", ItemCategory::Good, InteractionRole::ProductionCapital, "Neolithic_Sedentary_Revolution", "buah", "CapitalTool", "Batu penggiling gandum menjadi tepung halus", 3.0, false),
        def(ItemId::GRAIN_FLOUR, "Tepung Gandum Halus (Milled Grain Flour)", ItemCategory::Good, InteractionRole::Sustenance, "Neolithic_Sedentary_Revolution", "kg", "Nutrition", "Tepung gandum olahan siap panggang berdaya serap tinggi", 0.5, false),
        def(ItemId::FLATBREAD, "Roti Pipih Panggang (Baked Flatbread)", ItemCategory::Good, InteractionRole::Sustenance, "Neolithic_Sedentary_Revolution", "porsi", "Nutrition", "Pangan pokok berkalori tinggi 1200 kcal hasil pembakaran tepung", 0.3, false),
        def(ItemId::CHARCOAL, "Arang Kayu Piroteknologi (High-Heat Charcoal)", ItemCategory::Good, InteractionRole::RawInputMaterial, "Neolithic_Sedentary_Revolution", "kg", "RawMaterial", "Bahan bakar pirolisis pembakaran suhu tinggi", 0.5, false),

        // Fase 5: Neolithic Division of Labor & Professional Services
        def(ItemId::SERVICE_EDUCATION, "Jasa Pendidikan & Bimbingan Magang (Apprenticeship Tutoring)", ItemCategory::Service, InteractionRole::ProductionCapital, "Neolithic_Division_Of_Labor", "session", "Skill", "Waktu kerja guru untuk mentransfer pengetahuan non-rival kepada murid", 0.0, false),
        def(ItemId::SERVICE_MEDICAL, "Jasa Perawatan & Pemulihan Sakit (Caregiving & Healing)", ItemCategory::Service, InteractionRole::Sustenance, "Neolithic_Division_Of_Labor", "treatment", "LaborTime", "Jasa merawat agen lapar atau sakit untuk memulihkan kesehatan", 0.0, false),
        def(ItemId::KNOWLEDGE_HERBAL_MEDICINE, "Gagasan Ramuan Obat Tradisional (Herbal Medicine Blueprint)", ItemCategory::Knowledge, InteractionRole::ProductionCapital, "Neolithic_Division_Of_Labor", "idea", "Skill", "Gagasan teknologi identifikasi flora obat dan peracikan ramuan", 0.0, false),

        // Fase 6: Proto-Historic Currencies & Institutional Permits
        def(ItemId::SHELLS, "Cangkang Kerang Cowrie (Cowrie Shells)", ItemCategory::Currency, InteractionRole::SocialStatusAndGifting, "Proto_Historic_Currency", "biji", "MediumOfExchange", "Uang komoditas purba: ringan, seragam, tahan lama, dan mas kawin pernikahan", 0.05, false),
        def(ItemId::CLAY_TABLET, "Lempengan Tanah Liat Piutang (Promissory Debt Tablet)", ItemCategory::Currency, InteractionRole::MediumAndCollateral, "Proto_Historic_Currency", "keping", "MediumOfExchange", "Uang kredit purba bertuliskan kewajiban utang lumbung", 0.1, false),
        def(ItemId::WAREHOUSE_RECEIPT, "Sertifikat Deposito Lumbung (Warehouse Receipt)", ItemCategory::Currency, InteractionRole::MediumAndCollateral, "Proto_Historic_Currency", "lembar", "MediumOfExchange", "Kuitansi klaim simpanan gandum pada lumbung tembikar", 0.01, false),
        def(ItemId::PERMIT_FISHING_RIGHT, "Izin Hak Akses Perikanan (Fishing Access Right)", ItemCategory::Permit, InteractionRole::InstitutionalConcession, "Proto_Historic_Currency", "concession", "InstitutionalRight", "Hak institusional pemanfaatan sumber daya perairan bersama", 0.0, false),
        def(ItemId::PERMIT_FORESTRY_RIGHT, "Izin Konsesi Pemanfaatan Hutan (Forestry Concession)", ItemCategory::Permit, InteractionRole::InstitutionalConcession, "Proto_Historic_Currency", "concession", "InstitutionalRight", "Hak institusional penebangan kayu pada zona hutan adat", 0.0, false),
    ]
}
