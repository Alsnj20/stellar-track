#[cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Symbol, testutils::Address as _};

#[test]
fn test_initialize() {
    let env = Env::default();
    // WARNING: contract_register deprecated, use env.register instead and swap parameters
    let contract_id = env.register(HelloContract, ());

    let client = HelloContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    client.initialize(&admin);

    assert_eq!(client.get_greeting_count(), 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_no_reinicializar() {
    let env = Env::default();
    let contract_id = env.register(HelloContract, ());
    let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        client.initialize(&admin);  // Segunda vez debe fallar
    }


#[test]
fn test_hello_successful(){
    let env = Env::default();
    let contract_id = env.register(HelloContract, ());
    let client = HelloContractClient::new(&env, &contract_id);
        
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);

    let name_user = String::from_str(&env, "Alice");
    let greeting = client.hello(&user, &name_user);

    assert_eq!(greeting, Symbol::new(&env, "Hello"));
    assert_eq!(client.get_greeting_count(), 1);
    assert_eq!(client.get_last_greeting(&user), Some(name_user));
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_name_empty(){
    let env: Env = Env::default();
    let contract_id = env.register(HelloContract, ());
    let client = HelloContractClient::new(&env, &contract_id);
        
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);

    let empty = String::from_str(&env, "");
    client.hello(&user, &empty);
} 

#[test]
fn test_reset_admin(){
    let env: Env = Env::default();
    let contract_id = env.register(HelloContract, ());
    let client = HelloContractClient::new(&env, &contract_id);
        
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);

    client.hello(&user, &String::from_str(&env, "Test"));
    assert_eq!(client.get_greeting_count(), 1);

    // Admin can reset greeting count
    client.reset_greeting_count(&admin);
    assert_eq!(client.get_greeting_count(), 0); 
}

#[test]
#[should_panic(expected =  "Error(Contract, #3)")]
fn test_reset_admin_unauthorized(){
    let env: Env = Env::default();
    let contract_id = env.register(HelloContract, ());
    let client = HelloContractClient::new(&env, &contract_id);
        
    let admin = Address::generate(&env);
    let unauthorized = Address::generate(&env);

    client.initialize(&admin);

    // Unauthorized user tries to reset greeting count
    client.reset_greeting_count(&unauthorized);
}
