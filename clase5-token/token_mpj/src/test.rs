#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    Address, Env, String,
};

// ===================================
// 1. TESTS DE INICIALIZACIÓN
// ===================================

#[test]
fn test_initialize() {
    // Configurar entorno de prueba
    let env = Env::default();
    // WARNING: contract_register deprecated, use env.register instead and swap parameters
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    // Inicializar contrato
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "MiPasajeToken");
    let symbol = String::from_str(&env, "MPJ");
    
    // Ejecutar initialize
    client.initialize(&admin, &name, &symbol, &0);

    // Verificar estado después de la inicialización
    assert_eq!(client.name(), name);
    assert_eq!(client.symbol(), symbol);
    assert_eq!(client.decimals(), 0);
    assert_eq!(client.total_supply(), 0);
}

#[test]
fn test_reinitialize() {
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let name = String::from_str(&env, "MiPasajeToken");
    let symbol = String::from_str(&env, "MPJ");

    // Primera inicialización
    client.initialize(&admin, &name, &symbol, &0);

    // Segunda inicialización debe fallar
    // try es como void
    // unwrap es para obtener el valor dentro del Result
    assert_eq!(client.try_initialize(&admin, &name, &symbol, &0),
        Err(Ok(MiPasajeError::AlreadyInitialized)), "Expected AlreadyInitialized error"
    );
}

#[test]
fn test_invalid_decimals() {
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let name = String::from_str(&env, "MiPasajeToken");
    let symbol = String::from_str(&env, "MPJ");

    // Intentar inicializar con decimales inválidos (>18)
    let result = client.try_initialize(&admin, &name, &symbol, &19);

    assert_eq!(result, Err(Ok(MiPasajeError::InvalidDecimals)), "Expected InvalidDecimals error");
}

#[test]
fn test_invalid_metadata(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Intentar inicializar con nombre vacío
    let result_name_empty = client.try_initialize(
        &admin,
        &String::from_str(&env, ""),
        &String::from_str(&env, "MPJ"),
        &0,
    );
    assert_eq!(result_name_empty, Err(Ok(MiPasajeError::InvalidMetadata)), "Expected InvalidMetadata error for empty name");

    // Intentar inicializar con nombre demasiado largo
    let long_name = "A".repeat(101);
    let result_name_long = client.try_initialize(
        &admin,
        &String::from_str(&env, &long_name),
        &String::from_str(&env, "MPJ"),
        &0,
    );

    assert_eq!(result_name_long, Err(Ok(MiPasajeError::InvalidMetadata)), "Expected InvalidMetadata error for long name");

    // Intentar inicializar con símbolo vacío
    let result_symbol = client.try_initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, ""),
        &0,
    );
    assert_eq!(result_symbol, Err(Ok(MiPasajeError::InvalidMetadata)), "Expected InvalidMetadata error for symbol");
}

// ===================================
// 2. TESTS DE MINT Y BALANCE
// ===================================

#[test]
fn test_mint_and_balance(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    // Initialize token
    client.initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, "MPJ"),
        &0,
    );

    // Mock auth: simular la autorizacion del admin, es como si el admin estuviera firmando la transaccion.
    env.mock_all_auths();
    
    // --Mint tokens to user
    client.mint(&user, &100);

    // Check user balance
    assert_eq!(client.balance(&user), 100);
    assert_eq!(client.total_supply(), 100);
}

#[test]
fn test_mint_zero(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    // Initialize token
    client.initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, "MPJ"),
        &0,
    );

    // Mock auth
    env.mock_all_auths();
    
    // --Try to mint zero tokens
    let result = client.try_mint(&user, &0);

    assert_eq!(result, Err(Ok(MiPasajeError::InvalidAmount)), "Expected InvalidAmount error");
}

#[test]
fn test_mint_only_admin(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let attacker = Address::generate(&env);

    // Initialize token
    client.initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, "MPJ"),
        &0,
    );

    // Mock auth
    // env.mock_all_auths(); // Comentado para probar fallo de auth
    
    // --Attacker tries to mint tokens
    // Expect authentication failure (not a contract error, but a host error)
    let result = client.try_mint(&attacker, &100);

    // Soroban auth failure returns Err(Err(_)) 
    // We only check it is an error because checking specific host error is verbose
    assert!(result.is_err(), "Expected authentication error");
}

// ===================================
// 3. TESTS DE TRANSFER
// ===================================
/// Verifica el flujo completo de transfer:
/// 1. Alice tiene 1000 pasajes
/// 2. Alice transfiere 250 pasajes a Bob
/// 3. Ambos balances se actualizan correctamente

#[test]
fn test_transfer(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    // Initialize token
    client.initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, "MPJ"),
        &0,
    );

    // Mock auth
    env.mock_all_auths();
    
    // Mint 1000 pasajes to Alice
    client.mint(&alice, &1000);

    // Alice transfers 250 pasajes to Bob
    client.transfer(&alice, &bob, &250);

    // Verify balances
    assert_eq!(client.balance(&alice), 750);
    assert_eq!(client.balance(&bob), 250);
}

#[test]
fn test_transfer_insufficient_balance(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    // Initialize token
    client.initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, "MPJ"),
        &0,
    );

    // Mock auth
    env.mock_all_auths();
    
    // Mint 100 pasajes to Alice
    client.mint(&alice, &100);

    let result = client.try_transfer(&alice, &bob, &200);

    assert_eq!(result, Err(Ok(MiPasajeError::InsufficientBalance)), "Expected InsufficientBalance error");
}

#[test]
fn test_transfer_to_self(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);

    // Initialize token
    client.initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, "MPJ"),
        &0,
    );

    // Mock auth
    env.mock_all_auths();
    
    // Mint 100 pasajes to Alice
    client.mint(&alice, &100);

    // Alice tries to transfer 50 pasajes to herself
    let result = client.try_transfer(&alice, &alice, &50);

    assert_eq!(result, Err(Ok(MiPasajeError::InvalidRecipient)), "Expected InvalidRecipient error");
    assert_eq!(client.balance(&alice), 100, "Alice's balance should remain unchanged");
}

// ===================================
// 4. TESTS DE APPROVE + TRANSFER_FROM
// ===================================
/// Test del flujo completo de approve + transfer_from
/// 
/// Este es el patrón "allowance" usado en DeFi:
/// 1. Alice aprueba a Bob para gastar hasta 300 pasajes en su nombre
/// 2. Bob usa transfer_from para mover 200 pasajes de Alice a Charlie
/// 3. El allowance se reduce automáticamente a 100 pasajes

#[test]
fn test_approve_and_transfer_from(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    // Initialize token
    client.initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, "MPJ"),
        &0,
    );

    // Mock auth
    env.mock_all_auths();
    
    // Mint 1000 pasajes to Alice
    client.mint(&alice, &1000);

    // Alice approves Bob to spend 300 pasajes
    client.approve(&alice, &bob, &300);
    assert_eq!(client.allowance(&alice, &bob), 300);

    // Bob transfers 200 pasajes from Alice to Charlie
    client.transfer_from(&bob, &alice, &charlie, &200);

    // Verify balances and allowance
    assert_eq!(client.balance(&alice), 800);
    assert_eq!(client.balance(&charlie), 200);
    assert_eq!(client.allowance(&alice, &bob), 100);
}

#[test]
fn test_transfer_from_insufficient_allowance(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    // Initialize token
    client.initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, "MPJ"),
        &0,
    );

    // Mock auth
    env.mock_all_auths();
    
    // Mint 1000 pasajes to Alice
    client.mint(&alice, &1000);

    // Alice approves Bob to spend 100 pasajes
    client.approve(&alice, &bob, &100);
    assert_eq!(client.allowance(&alice, &bob), 100);

    // Bob tries to transfer 200 pasajes from Alice to Charlie
    let result = client.try_transfer_from(&bob, &alice, &charlie, &200);

    assert_eq!(result, Err(Ok(MiPasajeError::InsufficientAllowance)), "Expected InsufficientAllowance error");
}

#[test]
fn test_approve_revoke(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, "MPJ"),
        &0
    );
    
    env.mock_all_auths();
    client.mint(&alice, &1000);

    // Aprobar y luego revocar
    client.approve(&alice, &bob, &500);
    assert_eq!(client.allowance(&alice, &bob), 500);

    // Revocación, no deberia permitirse
    client.approve(&alice, &bob, &0);  
    assert_eq!(client.allowance(&alice, &bob), 0);
}

// ===================================
// 5. TESTS DE BURN
// ===================================
#[test]
fn test_burn(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);

    // Initialize token
    client.initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, "MPJ"),
        &0,
    );

    // Mock auth
    env.mock_all_auths();

    // Mint 500 pasajes to Alice
    client.mint(&alice, &500);
    // Burn 200 pasajes from Alice
    client.burn(&alice, &200);

    // Verify balance and total supply
    assert_eq!(client.balance(&alice), 300);
    assert_eq!(client.total_supply(), 300);
}

// ==================================
// 6. TESTS DE OPERACIONES SIN INICIALIZAR
// ==================================

#[test]
fn test_operations_without_init(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    env.mock_all_auths();

    // Try to mint without initialization
    // Todas las operaciones deben fallar con NotInitialized
    assert_eq!(
        client.try_mint(&alice, &100),
        Err(Ok(MiPasajeError::NotInitialized)), 
        "Expected NotInitialized error on mint"
    );
    
    assert_eq!(
        client.try_transfer(&alice, &bob, &50),
        Err(Ok(MiPasajeError::NotInitialized)),
        "Expected NotInitialized error on transfer"
    );
    
    assert_eq!(
        client.try_burn(&alice, &10),
        Err(Ok(MiPasajeError::NotInitialized)),
        "Expected NotInitialized error on burn"
    );
}



