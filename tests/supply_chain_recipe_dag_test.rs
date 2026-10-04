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
    assert_eq!(registry.all().len(), 6, "Expected 6 canonical recipes");

    let cured_fish_recipe = registry.get_recipe(6).expect("Recipe 6 (Cured Fish) should exist");
    assert_eq!(cured_fish_recipe.name, "Salt-Cured Preserved Fish");
    assert_eq!(cured_fish_recipe.category, "food_preservation");
    assert_eq!(cured_fish_recipe.required_knowledge, Some(ItemId::KNOWLEDGE_FISH_CURING));
    assert_eq!(cured_fish_recipe.inputs.len(), 2);
    assert_eq!(cured_fish_recipe.outputs.len(), 1);
    assert_eq!(cured_fish_recipe.outputs[0].item_id, ItemId::CURED_FISH);
    assert_eq!(cured_fish_recipe.outputs[0].quantity, 2);
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
