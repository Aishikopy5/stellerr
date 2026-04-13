#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Map, String};

#[contract]
pub struct IdentityHub;

#[contractimpl]
impl IdentityHub {
    // Register identity
    pub fn register(env: Env, user: String, data: String) {
        let key = Symbol::new(&env, "IDENTITY");
        let mut identities: Map<String, String> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        identities.set(user.clone(), data);
        env.storage().instance().set(&key, &identities);
    }

    // Get identity
    pub fn get(env: Env, user: String) -> Option<String> {
        let key = Symbol::new(&env, "IDENTITY");
        let identities: Map<String, String> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        identities.get(user)
    }

    // Update identity
    pub fn update(env: Env, user: String, data: String) {
        let key = Symbol::new(&env, "IDENTITY");
        let mut identities: Map<String, String> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        if identities.contains_key(user.clone()) {
            identities.set(user.clone(), data);
            env.storage().instance().set(&key, &identities);
        }
    }

    // Delete identity
    pub fn delete(env: Env, user: String) {
        let key = Symbol::new(&env, "IDENTITY");
        let mut identities: Map<String, String> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        identities.remove(user);
        env.storage().instance().set(&key, &identities);
    }
}
