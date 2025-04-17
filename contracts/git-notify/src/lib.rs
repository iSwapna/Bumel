#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol};

#[contract]
pub struct Contract;

#[derive(Clone)]
#[contracttype]
pub struct GitHubNotification {
    pub id: String,
    pub timestamp: u64,
}

#[contractimpl]
impl Contract {
    pub fn notify(env: Env, id: String, timestamp: u64) {
        // Store the notification in contract storage
        let notification = GitHubNotification { id, timestamp };
        let key = Symbol::new(&env, "notification");
        env.storage().persistent().set(&key, &notification);

        // Emit an event with the notification details
        // TODO: NEEDS ENCRYPTION WITH USER KEYS
        env.events().publish(
            (Symbol::new(&env, "github_notification"),),
            (timestamp, notification.id),
        );
    }

    pub fn get_notification(env: Env) -> Option<GitHubNotification> {
        let key = Symbol::new(&env, "notification");
        env.storage().persistent().get(&key)
    }
}

mod test;
