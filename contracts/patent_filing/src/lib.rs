#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String};

#[contract]
pub struct PatentFiling;

#[contractimpl]
impl PatentFiling {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// File a new patent with timestamp
    pub fn file_patent(env: Env, inventor: Address, patent_id: u64, title: String, description: String) {
        inventor.require_auth();

        let mut patents: Map<u64, (Address, String, String, u64)> = env
            .storage()
            .instance()
            .get(&"patents")
            .unwrap_or(Map::new(&env));

        if patents.contains_key(patent_id) {
            panic!("Patent already exists");
        }

        let timestamp = env.ledger().timestamp();
        patents.set(patent_id, (inventor, title, description, timestamp));
        env.storage().instance().set(&"patents", &patents);
    }

    /// Get patent details: returns (inventor, title, description, filing_timestamp)
    pub fn get_patent(env: Env, patent_id: u64) -> (Address, String, String, u64) {
        let patents: Map<u64, (Address, String, String, u64)> = env
            .storage()
            .instance()
            .get(&"patents")
            .unwrap_or(Map::new(&env));

        patents.get(patent_id).unwrap_or_else(|| panic!("Patent not found"))
    }

    /// Transfer patent to new inventor (only current inventor can transfer)
    pub fn transfer_patent(env: Env, new_inventor: Address, patent_id: u64) {
        new_inventor.require_auth();

        let mut patents: Map<u64, (Address, String, String, u64)> = env
            .storage()
            .instance()
            .get(&"patents")
            .unwrap_or(Map::new(&env));

        let entry = patents.get(patent_id).unwrap_or_else(|| panic!("Patent not found"));
        let (current_inventor, title, description, timestamp) = entry;

        // Require auth from current inventor to authorize transfer
        current_inventor.require_auth();

        patents.set(patent_id, (new_inventor, title, description, timestamp));
        env.storage().instance().set(&"patents", &patents);
    }

    /// Verify if an address is the inventor of a patent
    pub fn verify_inventor(env: Env, patent_id: u64, address: Address) -> bool {
        let patents: Map<u64, (Address, String, String, u64)> = env
            .storage()
            .instance()
            .get(&"patents")
            .unwrap_or(Map::new(&env));

        match patents.get(patent_id) {
            Some((inventor, _, _, _)) => inventor == address,
            None => false,
        }
    }
}