#![cfg(test)]

use crate::{Contract, ContractClient};
use soroban_sdk::{Env, String};

#[test]
fn test_notify_and_get_notification() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Test notify function with sample data
    let id = String::from_str(&env, "12345");
    let timestamp = 1678901234;
    client.notify(&id, &timestamp);

    // Test get_notification function
    let stored_notification = client.get_notification();
    assert!(stored_notification.is_some());
    let stored = stored_notification.unwrap();
    
    // Verify fields match
    assert_eq!(stored.id, id);
    assert_eq!(stored.timestamp, timestamp);
}
