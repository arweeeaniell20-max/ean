#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

/// Storage keys for the AgroSplit contract
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    TokenAddress,
    FarmerPercent,
    CoopPercent,
    SavingsPercent,
    CoopWallet,
    SavingsWallet,
}

#[contract]
pub struct AgroSplitContract;

#[contractimpl]
impl AgroSplitContract {
    /// Initializes the contract with admin, token address, split percentages, and destination wallets.
    /// Percentages must sum to 100.
    pub fn initialize(
        env: Env,
        admin: Address,
        token_address: Address,
        farmer_percent: u32,
        coop_percent: u32,
        savings_percent: u32,
        coop_wallet: Address,
        savings_wallet: Address,
    ) {
        // Ensure percentages sum to 100
        assert!(
            farmer_percent + coop_percent + savings_percent == 100,
            "percentages must sum to 100"
        );

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenAddress, &token_address);
        env.storage().instance().set(&DataKey::FarmerPercent, &farmer_percent);
        env.storage().instance().set(&DataKey::CoopPercent, &coop_percent);
        env.storage().instance().set(&DataKey::SavingsPercent, &savings_percent);
        env.storage().instance().set(&DataKey::CoopWallet, &coop_wallet);
        env.storage().instance().set(&DataKey::SavingsWallet, &savings_wallet);
    }

    /// Processes a harvest payment by splitting the total amount among farmer, cooperative, and savings pool.
    /// Called by the cooperative admin after weighing the harvest.
    pub fn process_harvest(env: Env, caller: Address, farmer_wallet: Address, total_amount: i128) {
        caller.require_auth();

        // Verify caller is admin
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        assert!(caller == admin, "only admin can process harvest");

        // Retrieve configuration
        let token_address: Address = env.storage().instance().get(&DataKey::TokenAddress).unwrap();
        let farmer_percent: u32 = env.storage().instance().get(&DataKey::FarmerPercent).unwrap();
        let coop_percent: u32 = env.storage().instance().get(&DataKey::CoopPercent).unwrap();
        let savings_percent: u32 = env.storage().instance().get(&DataKey::SavingsPercent).unwrap();
        let coop_wallet: Address = env.storage().instance().get(&DataKey::CoopWallet).unwrap();
        let savings_wallet: Address = env.storage().instance().get(&DataKey::SavingsWallet).unwrap();

        // Calculate split amounts
        let farmer_amount = (total_amount * farmer_percent as i128) / 100;
        let coop_amount = (total_amount * coop_percent as i128) / 100;
        let savings_amount = (total_amount * savings_percent as i128) / 100;

        // Execute transfers using the token contract
        let token_client = token::Client::new(&env, &token_address);

        // Transfer from the contract's balance (contract must hold funds)
        token_client.transfer(&env.current_contract_address(), &farmer_wallet, &farmer_amount);
        token_client.transfer(&env.current_contract_address(), &coop_wallet, &coop_amount);
        token_client.transfer(&env.current_contract_address(), &savings_wallet, &savings_amount);

        // Emit event for transparency
        env.events().publish(
            (Symbol::new(&env, "harvest_processed"),),
            (farmer_wallet, farmer_amount, coop_amount, savings_amount),
        );
    }

    /// Returns the current split configuration
    pub fn get_config(env: Env) -> (u32, u32, u32) {
        let farmer_percent: u32 = env.storage().instance().get(&DataKey::FarmerPercent).unwrap_or(0);
        let coop_percent: u32 = env.storage().instance().get(&DataKey::CoopPercent).unwrap_or(0);
        let savings_percent: u32 = env.storage().instance().get(&DataKey::SavingsPercent).unwrap_or(0);
        (farmer_percent, coop_percent, savings_percent)
    }
}
