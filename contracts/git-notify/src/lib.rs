#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol};

#[contract]
pub struct Contract;

#[derive(Clone)]
#[contracttype]
pub struct GitHubUser {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
}

#[derive(Clone)]
#[contracttype]
pub struct GitHubNotification {
    pub id: String,
    pub repository: String,
    pub title: String,
    pub type_: String,
    pub timestamp: u64,
    pub sender: GitHubUser,
    pub action: String,  // e.g., "opened", "closed", "created"
}

#[contractimpl]
impl Contract {
    pub fn notify(env: Env, notification: GitHubNotification) {
        // Store the notification in contract storage
        let key = Symbol::new(&env, "notification");
        env.storage().persistent().set(&key, &notification);

        // Emit an event with the notification details
        // TODO: NEEDS ENCRYPTION WITH USER KEYS
        env.events().publish(
            (Symbol::new(&env, "github_notification"),),
            (
                notification.id,
                notification.repository,
                notification.title,
                notification.type_,
                notification.timestamp,
                notification.sender.login,
                notification.sender.id,
                notification.action,
            ),
        );
    }

    pub fn get_notification(env: Env) -> Option<GitHubNotification> {
        let key = Symbol::new(&env, "notification");
        env.storage().persistent().get(&key)
    }
}

mod test;
