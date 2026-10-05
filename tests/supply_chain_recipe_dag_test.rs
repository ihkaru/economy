use economy::adapters::randomness::ChaChaRngAdapter;
use economy::adapters::storage::{MemoryAgentStore, MemoryLedgerStore};
use economy::core::domain::agent::human::{Human, Sex};
use economy::core::domain::agent::id::AgentId;
use economy::core::domain::agent::traits::EconomicActor;
use economy::core::domain::item::id::ItemId;
use economy::core::domain::production::RecipeRegistry;
use economy::core::domain::spatial::coordinate::GeoCoordinate;
use economy::core::domain::time::{RunId, Tick};
use economy::core::ports::agent_store::AgentStorePort;
use economy::core::ports::ledger_store::LedgerStorePort;
use economy::core::systems::exchange::perform_autonomous_crafting;

#[test]
fn test_canonical_recipe_registry_specifications() {
    let registry = RecipeRegistry::canonical();
    assert_eq!(registry.all().len(), 20, "Expected 20 canonical recipes");

    let stone_axe_recipe = registry.get_recipe(2).expect("Recipe 2 (Stone Axe) should exist");
    assert_eq!(stone_axe_recipe.name, "Polished Stone Axe");
    assert_eq!(stone_axe_recipe.inputs.len(), 2, "Stone Axe requires 2 inputs: Lithic Flake + Timber");
    assert_eq!(stone_axe_recipe.inputs[0].item_id, ItemId::LITHIC_FLAKE);
    assert_eq!(stone_axe_recipe.inputs[0].quantity, 1);
    assert_eq!(stone_axe_recipe.inputs[1].item_id, ItemId::TIMBER);
    assert_eq!(stone_axe_recipe.inputs[1].quantity, 1);

    let cured_fish_recipe = registry.get_recipe(6).expect("Recipe 6 (Cured Fish) should exist");
    assert_eq!(cured_fish_recipe.name, "Salt-Cured Preserved Fish");
    assert_eq!(cured_fish_recipe.category, "food_preservation");
    assert_eq!(cured_fish_recipe.required_knowledge, Some(ItemId::KNOWLEDGE_FISH_CURING));
    assert_eq!(cured_fish_recipe.inputs.len(), 2);
    assert_eq!(cured_fish_recipe.outputs.len(), 1);
    assert_eq!(cured_fish_recipe.outputs[0].item_id, ItemId::CURED_FISH);
    assert_eq!(cured_fish_recipe.outputs[0].quantity, 2);

    let smoked_fish_recipe = registry.get_recipe(8).expect("Recipe 8 (Smoked Fish) should exist");
    assert_eq!(smoked_fish_recipe.name, "Wood-Smoked Preserved Fish");
    assert_eq!(smoked_fish_recipe.required_knowledge, Some(ItemId::KNOWLEDGE_FIRE_MAKING));
    assert_eq!(smoked_fish_recipe.outputs[0].item_id, ItemId::SMOKED_FISH);

    let pottery_recipe = registry.get_recipe(9).expect("Recipe 9 (Pottery Jar) should exist");
    assert_eq!(pottery_recipe.name, "Ceramic Storage Pottery Jar");
    assert_eq!(pottery_recipe.category, "ceramic_storage");
    assert_eq!(pottery_recipe.required_knowledge, Some(ItemId::KNOWLEDGE_POTTERY_MAKING));
    assert_eq!(pottery_recipe.outputs[0].item_id, ItemId::POTTERY_JAR);
    assert_eq!(pottery_recipe.outputs[0].quantity, 1);

    let smoked_meat_recipe = registry.get_recipe(10).expect("Recipe 10 (Smoked Meat) should exist");
    assert_eq!(smoked_meat_recipe.name, "Wood-Smoked Preserved Meat");
    assert_eq!(smoked_meat_recipe.outputs[0].item_id, ItemId::SMOKED_MEAT);

    let leather_recipe = registry.get_recipe(11).expect("Recipe 11 (Leather Clothing) should exist");
    assert_eq!(leather_recipe.name, "Warm Leather Garment");
    assert_eq!(leather_recipe.category, "clothing_tailoring");
    assert_eq!(leather_recipe.required_knowledge, Some(ItemId::KNOWLEDGE_LEATHER_WORKING));
    assert_eq!(leather_recipe.required_tool, Some(ItemId::BONE_NEEDLE));
    assert_eq!(leather_recipe.outputs[0].item_id, ItemId::LEATHER_CLOTHING);

    let spear_recipe = registry.get_recipe(12).expect("Recipe 12 (Hunting Spear) should exist");
    assert_eq!(spear_recipe.name, "Prehistoric Hunting Spear");
    assert_eq!(spear_recipe.category, "capital_tool_production");
    assert_eq!(spear_recipe.inputs[0].item_id, ItemId::LITHIC_FLAKE);
    assert_eq!(spear_recipe.outputs[0].item_id, ItemId::HUNTING_SPEAR);

    let cured_meat_recipe = registry.get_recipe(13).expect("Recipe 13 (Cured Meat) should exist");
    assert_eq!(cured_meat_recipe.name, "Salt-Cured Preserved Meat");
    assert_eq!(cured_meat_recipe.category, "food_preservation");

    let quern_recipe = registry.get_recipe(14).expect("Recipe 14 (Saddle Quern) should exist");
    assert_eq!(quern_recipe.outputs[0].item_id, ItemId::SADDLE_QUERN);

    let flour_recipe = registry.get_recipe(15).expect("Recipe 15 (Grain Flour) should exist");
    assert_eq!(flour_recipe.outputs[0].item_id, ItemId::GRAIN_FLOUR);

    let flatbread_recipe = registry.get_recipe(16).expect("Recipe 16 (Flatbread) should exist");
    assert_eq!(flatbread_recipe.outputs[0].item_id, ItemId::FLATBREAD);

    let charcoal_recipe = registry.get_recipe(17).expect("Recipe 17 (Charcoal) should exist");
    assert_eq!(charcoal_recipe.outputs[0].item_id, ItemId::CHARCOAL);
    assert_eq!(cured_meat_recipe.outputs[0].item_id, ItemId::CURED_MEAT);

    let tablet_recipe = registry.get_recipe(18).expect("Recipe 18 (Clay Tablet) should exist");
    assert_eq!(tablet_recipe.name, "Inscribed Clay Tablet");
    assert_eq!(tablet_recipe.category, "tablet_crafting");
    assert_eq!(tablet_recipe.required_knowledge, Some(ItemId::KNOWLEDGE_POTTERY_MAKING));
    assert_eq!(tablet_recipe.inputs[0].item_id, ItemId::CLAY);
    assert_eq!(tablet_recipe.outputs[0].item_id, ItemId::CLAY_TABLET);
    assert_eq!(tablet_recipe.outputs[0].quantity, 2);

    let flake_recipe = registry.get_recipe(19).expect("Recipe 19 (Lithic Flake) should exist");
    assert_eq!(flake_recipe.name, "Knapped Stone Blade Flake");
    assert_eq!(flake_recipe.inputs[0].item_id, ItemId::STONE);
    assert_eq!(flake_recipe.outputs[0].item_id, ItemId::LITHIC_FLAKE);
    assert_eq!(flake_recipe.outputs[0].quantity, 2);

    let needle_recipe = registry.get_recipe(20).expect("Recipe 20 (Bone Needle) should exist");
    assert_eq!(needle_recipe.name, "Bone Needle Abrasive Grinding");
    assert_eq!(needle_recipe.inputs[0].item_id, ItemId::ANIMAL_BONE);
    assert_eq!(needle_recipe.inputs[1].item_id, ItemId::STONE);
    assert_eq!(needle_recipe.outputs[0].item_id, ItemId::BONE_NEEDLE);
    assert_eq!(needle_recipe.outputs[0].quantity, 1);
}

#[test]
fn test_autonomous_crafting_execution_and_leontief_consumption() {
    let mut agent_store = MemoryAgentStore::new();
    let mut ledger_store = MemoryLedgerStore::new();
    let mut rng = ChaChaRngAdapter::new(42);

    let agent_id = AgentId::new(101);
    let mut agent = Human::new(agent_id, Sex::Male, Tick::ZERO)
        .with_initial_age(25 * 365)
        .with_calories(5000.0)
        .with_location(GeoCoordinate::new(10, 10));

    // Give materials for Salt-Cured Fish: 4 Fish + 2 Salt + Fish Curing Knowledge
    agent.add_item(ItemId::FISH, 4);
    agent.add_item(ItemId::SALT, 2);
    agent.add_item(ItemId::KNOWLEDGE_FISH_CURING, 1);

    agent_store.insert_human(agent);

    let living_ids = vec![agent_id];
    let run_id = RunId::new("test_supply_chain");
    let mut next_trx_id = 1;
    let mut next_instance_id = 1;

    // Run crafting step
    perform_autonomous_crafting(
        &run_id,
        Tick(1),
        &living_ids,
        &mut agent_store,
        &mut ledger_store,
        &mut rng,
        &mut next_trx_id,
        &mut next_instance_id,
    );

    let agent = agent_store.get_human(agent_id).unwrap();
    // 2 Fish and 1 Salt should be consumed, producing 2 Cured Fish
    assert_eq!(agent.inventory.get(&ItemId::FISH).copied().unwrap_or(0), 2, "2 Fish remaining");
    assert_eq!(agent.inventory.get(&ItemId::SALT).copied().unwrap_or(0), 1, "1 Salt remaining");
    assert_eq!(agent.inventory.get(&ItemId::CURED_FISH).copied().unwrap_or(0), 2, "2 Cured Fish produced");
    assert_eq!(agent.inventory.get(&ItemId::KNOWLEDGE_FISH_CURING).copied().unwrap_or(0), 1, "Non-rival knowledge retained");
    assert!(agent.calorie_reserve < 5000.0, "Labor calories burned");

    // Ledger check
    let entries = ledger_store.all_entries();
    assert_eq!(entries.len(), 1, "1 crafting transaction in ledger");
    assert_eq!(entries[0].metadata.get("transaction_type").unwrap(), "food_preservation");
    assert_eq!(entries[0].metadata.get("product_name").unwrap(), "Salt-Cured Preserved Fish");
}

#[test]
fn test_pottery_jar_crafting_and_storage_expansion() {
    let mut agent_store = MemoryAgentStore::new();
    let mut ledger_store = MemoryLedgerStore::new();
    let mut rng = ChaChaRngAdapter::new(42);

    let agent_id = AgentId::new(102);
    let mut agent = Human::new(agent_id, Sex::Female, Tick::ZERO)
        .with_initial_age(20 * 365)
        .with_calories(6000.0)
        .with_location(GeoCoordinate::new(15, 26));

    assert_eq!(agent.carrying_capacity_kg(), 25.0, "Base capacity is 25 kg");

    // Give materials: 4 Clay + 1 Timber + Pottery Knowledge
    agent.add_item(ItemId::CLAY, 4);
    agent.add_item(ItemId::TIMBER, 1);
    agent.add_item(ItemId::KNOWLEDGE_POTTERY_MAKING, 1);

    agent_store.insert_human(agent);

    let living_ids = vec![agent_id];
    let run_id = RunId::new("test_pottery_chain");
    let mut next_trx_id = 1;
    let mut next_instance_id = 1;

    perform_autonomous_crafting(
        &run_id,
        Tick(1),
        &living_ids,
        &mut agent_store,
        &mut ledger_store,
        &mut rng,
        &mut next_trx_id,
        &mut next_instance_id,
    );

    let agent = agent_store.get_human(agent_id).unwrap();
    assert_eq!(agent.inventory.get(&ItemId::CLAY).copied().unwrap_or(0), 0, "Clay consumed");
    assert_eq!(agent.inventory.get(&ItemId::TIMBER).copied().unwrap_or(0), 0, "Timber consumed");
    assert_eq!(agent.inventory.get(&ItemId::POTTERY_JAR).copied().unwrap_or(0), 1, "Pottery jar produced");
    assert_eq!(agent.carrying_capacity_kg(), 75.0, "Capacity expanded from 25 kg to 75 kg (+50 kg granary jar)");

    let entries = ledger_store.all_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].metadata.get("transaction_type").unwrap(), "ceramic_storage");
}

#[test]
fn test_warm_leather_garment_tailoring() {
    let mut agent_store = MemoryAgentStore::new();
    let mut ledger_store = MemoryLedgerStore::new();
    let mut rng = ChaChaRngAdapter::new(42);

    let agent_id = AgentId::new(103);
    let mut agent = Human::new(agent_id, Sex::Male, Tick::ZERO)
        .with_initial_age(22 * 365)
        .with_calories(6000.0)
        .with_location(GeoCoordinate::new(16, 27));

    // Give materials: 2 Raw Hide + 1 Timber + Leather Working Knowledge + Bone Needle tool
    agent.add_item(ItemId::RAW_HIDE, 2);
    agent.add_item(ItemId::TIMBER, 1);
    agent.add_item(ItemId::KNOWLEDGE_LEATHER_WORKING, 1);
    agent.add_item(ItemId::BONE_NEEDLE, 1);

    agent_store.insert_human(agent);

    let living_ids = vec![agent_id];
    let run_id = RunId::new("test_clothing_chain");
    let mut next_trx_id = 1;
    let mut next_instance_id = 1;

    perform_autonomous_crafting(
        &run_id,
        Tick(1),
        &living_ids,
        &mut agent_store,
        &mut ledger_store,
        &mut rng,
        &mut next_trx_id,
        &mut next_instance_id,
    );

    let agent = agent_store.get_human(agent_id).unwrap();
    assert_eq!(agent.inventory.get(&ItemId::RAW_HIDE).copied().unwrap_or(0), 0, "Raw hide consumed");
    assert_eq!(agent.inventory.get(&ItemId::TIMBER).copied().unwrap_or(0), 0, "Timber consumed");
    assert_eq!(agent.inventory.get(&ItemId::LEATHER_CLOTHING).copied().unwrap_or(0), 1, "Leather clothing produced");
    assert_eq!(agent.inventory.get(&ItemId::BONE_NEEDLE).copied().unwrap_or(0), 1, "Bone needle capital tool retained");
    assert!(agent.has_item(ItemId::LEATHER_CLOTHING));

    let entries = ledger_store.all_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].metadata.get("transaction_type").unwrap(), "clothing_tailoring");
    assert_eq!(entries[0].metadata.get("product_name").unwrap(), "Warm Leather Garment");
}

#[test]
fn test_prehistoric_hunting_spear_and_cured_meat_crafting() {
    let mut agent_store = MemoryAgentStore::new();
    let mut ledger_store = MemoryLedgerStore::new();
    let mut rng = ChaChaRngAdapter::new(42);

    let agent_id = AgentId::new(104);
    let mut agent = Human::new(agent_id, Sex::Female, Tick::ZERO)
        .with_initial_age(24 * 365)
        .with_calories(6000.0)
        .with_location(GeoCoordinate::new(16, 27));

    // Agent already has Stone Axe, and now gathers materials for spear (1 Lithic Flake + 1 Timber + Tool Crafting)
    // and cured meat (2 Raw Meat + 1 Salt + Fish Curing)
    agent.add_item(ItemId::STONE_AXE, 1);
    agent.add_item(ItemId::LITHIC_FLAKE, 1);
    agent.add_item(ItemId::TIMBER, 1);
    agent.add_item(ItemId::KNOWLEDGE_TOOL_CRAFTING, 1);
    agent.add_item(ItemId::RAW_MEAT, 2);
    agent.add_item(ItemId::SALT, 1);
    agent.add_item(ItemId::KNOWLEDGE_FISH_CURING, 1);

    agent_store.insert_human(agent);

    let living_ids = vec![agent_id];
    let run_id = RunId::new("test_spear_and_meat");
    let mut next_trx_id = 1;
    let mut next_instance_id = 1;

    // Day 1: Autonomous Crafting (Crafts Prehistoric Hunting Spear)
    perform_autonomous_crafting(
        &run_id,
        Tick(1),
        &living_ids,
        &mut agent_store,
        &mut ledger_store,
        &mut rng,
        &mut next_trx_id,
        &mut next_instance_id,
    );

    // Day 2: Autonomous Crafting (Crafts Salt-Cured Preserved Meat)
    perform_autonomous_crafting(
        &run_id,
        Tick(2),
        &living_ids,
        &mut agent_store,
        &mut ledger_store,
        &mut rng,
        &mut next_trx_id,
        &mut next_instance_id,
    );

    let agent = agent_store.get_human(agent_id).unwrap();
    assert_eq!(agent.inventory.get(&ItemId::LITHIC_FLAKE).copied().unwrap_or(0), 0, "Lithic flake consumed");
    assert_eq!(agent.inventory.get(&ItemId::TIMBER).copied().unwrap_or(0), 0, "Timber consumed");
    assert_eq!(agent.inventory.get(&ItemId::HUNTING_SPEAR).copied().unwrap_or(0), 1, "Hunting spear crafted");
    assert_eq!(agent.inventory.get(&ItemId::RAW_MEAT).copied().unwrap_or(0), 0, "Raw meat consumed");
    assert_eq!(agent.inventory.get(&ItemId::SALT).copied().unwrap_or(0), 0, "Salt consumed");
    assert_eq!(agent.inventory.get(&ItemId::CURED_MEAT).copied().unwrap_or(0), 2, "2 Cured meat produced");

    let entries = ledger_store.all_entries();
    assert_eq!(entries.len(), 2, "2 crafting transactions executed across 2 days");
}

#[test]
fn test_lithic_flaking_and_bone_needle_crafting() {
    let mut agent_store = MemoryAgentStore::new();
    let mut ledger_store = MemoryLedgerStore::new();
    let mut rng = ChaChaRngAdapter::new(42);

    let agent_id = AgentId::new(105);
    let mut agent = Human::new(agent_id, Sex::Male, Tick::ZERO)
        .with_initial_age(23 * 365)
        .with_calories(6000.0)
        .with_location(GeoCoordinate::new(15, 25));

    // Agent has raw lithic stone, carcass animal bone, and tool crafting knowledge
    agent.add_item(ItemId::STONE, 2);
    agent.add_item(ItemId::ANIMAL_BONE, 1);
    agent.add_item(ItemId::KNOWLEDGE_TOOL_CRAFTING, 1);

    agent_store.insert_human(agent);

    let living_ids = vec![agent_id];
    let run_id = RunId::new("test_lithic_bone");
    let mut next_trx_id = 1;
    let mut next_instance_id = 1;

    // Day 1: Lithic Reduction (Knaps Stone into 2 Lithic Flakes)
    perform_autonomous_crafting(
        &run_id,
        Tick(1),
        &living_ids,
        &mut agent_store,
        &mut ledger_store,
        &mut rng,
        &mut next_trx_id,
        &mut next_instance_id,
    );

    let agent = agent_store.get_human(agent_id).unwrap();
    assert_eq!(agent.inventory.get(&ItemId::LITHIC_FLAKE).copied().unwrap_or(0), 2, "2 Lithic Flakes produced");
    assert_eq!(agent.inventory.get(&ItemId::STONE).copied().unwrap_or(0), 1, "1 Stone remaining");

    // Day 2: Bone Toolworking (Grinds Animal Bone + Stone into Bone Needle)
    perform_autonomous_crafting(
        &run_id,
        Tick(2),
        &living_ids,
        &mut agent_store,
        &mut ledger_store,
        &mut rng,
        &mut next_trx_id,
        &mut next_instance_id,
    );

    let agent = agent_store.get_human(agent_id).unwrap();
    assert_eq!(agent.inventory.get(&ItemId::BONE_NEEDLE).copied().unwrap_or(0), 1, "1 Bone Needle produced");
    assert_eq!(agent.inventory.get(&ItemId::ANIMAL_BONE).copied().unwrap_or(0), 0, "Animal bone consumed");
    assert_eq!(agent.inventory.get(&ItemId::STONE).copied().unwrap_or(0), 0, "Second stone consumed as abrasive");

    let entries = ledger_store.all_entries();
    assert_eq!(entries.len(), 2, "2 tool manufacturing transactions in ledger");
}


