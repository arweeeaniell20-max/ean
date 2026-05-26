#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{AgroSplitContract, AgroSplitContractClient, DataKey};

mod token {
    soroban_sdk::contractimport!(file = "soroban_token_contract.wasm");
}

fn create_token_contract(env: &Env, admin: &Address) -> (Address, token::Client) {
    let contract_id = env.register_stellar_asset_contract_v2(admin.clone());
    (contract_id.address(), token::Client::new(env, &contract_id.address()))
}

#[test]
fn test_happy_path_harvest_payment() {
    // Setup environment and accounts
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let farmer = Address::generate(&env);
    let coop_wallet = Address::generate(&env);
    let savings_wallet = Address::generate(&env);

    // Create token and mint to contract
    let (token_address, token_client) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AgroSplitContract);
    let client = AgroSplitContractClient::new(&env, &contract_id);

    // Initialize with 85% farmer, 10% coop, 5% savings
    client.initialize(
        &admin,
        &token_address,
        &85,
        &10,
        &5,
        &coop_wallet,
        &savings_wallet,
    );

    // Mint tokens to contract address
    token_client.mint(&contract_id, &1000);

    // Process harvest of 1000 tokens
    client.process_harvest(&admin, &farmer, &1000);

    // Verify splits: 850, 100, 50
    assert_eq!(token_client.balance(&farmer), 850);
    assert_eq!(token_client.balance(&coop_wallet), 100);
    assert_eq!(token_client.balance(&savings_wallet), 50);
}

#[test]
#[should_panic(expected = "only admin can process harvest")]
fn test_unauthorized_caller_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let not_admin = Address::generate(&env);
    let farmer = Address::generate(&env);
    let coop_wallet = Address::generate(&env);
    let savings_wallet = Address::generate(&env);

    let (token_address, _) = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, AgroSplitContract);
    let client = AgroSplitContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token_address, &85, &10, &5, &coop_wallet, &savings_wallet);

    // Non-admin attempts to process harvest - should panic
    client.process_harvest(&not_admin, &farmer, &1000);
}

#[test]
fn test_state_after_harvest() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let farmer = Address::generate(&env);
    let coop_wallet = Address::generate(&env);
    let savings_wallet = Address::generate(&env);

    let (token_address, token_client) = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, AgroSplitContract);
    let client = AgroSplitContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token_address, &85, &10, &5, &coop_wallet, &savings_wallet);
    token_client.mint(&contract_id, &1000);

    // Verify config is stored correctly
    let (f, c, s) = client.get_config();
    assert_eq!(f, 85);
    assert_eq!(c, 10);
    assert_eq!(s, 5);

    client.process_harvest(&admin, &farmer, &1000);

    // Contract balance should be 0 after distribution
    assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
#[should_panic(expected = "percentages must sum to 100")]
fn test_invalid_percentages_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let coop_wallet = Address::generate(&env);
    let savings_wallet = Address::generate(&env);
    let (token_address, _) = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, AgroSplitContract);
    let client = AgroSplitContractClient::new(&env, &contract_id);

    // Percentages sum to 90, not 100 - should panic
    client.initialize(&admin, &token_address, &80, &5, &5, &coop_wallet, &savings_wallet);
}

#[test]
fn test_multiple_harvests_accumulate() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let farmer = Address::generate(&env);
    let coop_wallet = Address::generate(&env);
    let savings_wallet = Address::generate(&env);

    let (token_address, token_client) = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, AgroSplitContract);
    let client = AgroSplitContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token_address, &85, &10, &5, &coop_wallet, &savings_wallet);
    
    // Mint enough for two harvests
    token_client.mint(&contract_id, &2000);

    client.process_harvest(&admin, &farmer, &1000);
    client.process_harvest(&admin, &farmer, &1000);

    // Farmer should have 850 * 2 = 1700
    assert_eq!(token_client.balance(&farmer), 1700);
}
