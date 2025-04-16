#![cfg(test)]

use crate::{Contract, ContractClient, GitHubNotification, GitHubUser};
use soroban_sdk::{Env, String};

#[test]
fn test_notify_and_get_notification() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Create a sample GitHub user
    let sender = GitHubUser {
        login: String::from_str(&env, "octocat"),
        id: 123456,
        avatar_url: String::from_str(&env, "https://github.com/images/error/octocat_happy.gif"),
    };

    // Create a sample GitHub notification
    let notification = GitHubNotification {
        id: String::from_str(&env, "12345"),
        repository: String::from_str(&env, "stellar/soroban-examples"),
        title: String::from_str(&env, "New pull request: Add GitHub notification example"),
        type_: String::from_str(&env, "pull_request"),
        timestamp: 1678901234,
        sender,
        action: String::from_str(&env, "opened"),
    };

    // Test notify function
    client.notify(&notification);

    // Test get_notification function
    let stored_notification = client.get_notification();
    assert!(stored_notification.is_some());
    let stored = stored_notification.unwrap();
    
    // Verify all fields match
    assert_eq!(stored.id, notification.id);
    assert_eq!(stored.repository, notification.repository);
    assert_eq!(stored.title, notification.title);
    assert_eq!(stored.type_, notification.type_);
    assert_eq!(stored.timestamp, notification.timestamp);
    assert_eq!(stored.sender.login, notification.sender.login);
    assert_eq!(stored.sender.id, notification.sender.id);
    assert_eq!(stored.action, notification.action);
}
