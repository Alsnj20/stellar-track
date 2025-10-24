#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events},
    Address, Env, String, symbol_short,
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
    let result = client.initialize(&admin, &name, &symbol, &0);
    assert!(result.is_ok());

    // Verificar estado después de la inicialización
    assert_eq!(client.get_name(), name);
    assert_eq!(client.get_symbol(), symbol);
    assert_eq!(client.get_decimals(), 0);
    assert_eq!(client.get_total_supply(), 0);
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
    assert!(client.initialize(&admin, &name, &symbol, &0).is_ok());

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
    let result = client.initialize(&admin, &name, &symbol, &19);

    assert_eq!(result, Err(MiPasajeError::InvalidDecimals), "Expected InvalidDecimals error");
}

#[test]
fn test_invalid_metadata(){
    let env = Env::default();
    let contract_id = env.register(TokenMiPasaje, ());
    let client = TokenMiPasajeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Intentar inicializar con nombre vacío
    let result_name = client.try_initialize(
        &admin,
        &String::from_str(&env, ""),
        &symbol_short(&env, "MPJ"),
        &0,
    )

    // Intentar inicializar con nombre demasiado largo
    let long_name = "A".repeat(65);
    let result_name = client.try_initialize(
        &admin,
        &String::from_str(&env, &long_name),
        &symbol_short(&env),
        &0,
    );

    assert_eq!(result_name, Err(Ok(MiPasajeError::InvalidMetadata)), "Expected InvalidMetadata error for name");

    // Intentar inicializar con símbolo vacío
    let result_symbol = client.try_initialize(
        &admin,
        &String::from_str(&env, "MiPasajeToken"),
        &String::from_str(&env, ""),
        &0,
    );
    assert_eq!(result_symbol, Err(Ok(MiPasajeError::InvalidMetadata)), "Expected InvalidMetadata error for symbol");
}








