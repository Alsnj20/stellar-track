// STRING VS SYMBOL
#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, symbol_long, SymbolShort, String};

pub struct StringSymbolContract;

#[contractimpl]
impl StringSymbolContract {
    // 1. Key para almacenar balance en storage
    pub fn key_balance() -> SymbolShort {
        symbol_short!("balance")
    }

    // 2. Descripción de producto (escrita por vendedor)
    pub fn key_product_description() -> String {
        String::from("product_description")
    }

    // 3. Estado del contrato: "active", "paused", "finalized"
    pub fn key_contract_status() -> SymbolShort {
        symbol_short!("active")
    }

    // 4. Comentario de usuario en review
    pub fn key_user_review_comment() -> String {
        String::from("user_review_comment")
    }

    // 5. Topic de evento de transferencia
    pub fn key_transfer_event_topic() -> SymbolShort {
        symbol_short!("transfer_event")
    }

    // 6. Nombre de token (puede ser "Mi Token Super Largo 2024")
    pub fn key_token_name() -> String {
        String::from("My Token Super Long 2025")
    }

    // 7. Key para almacenar owner del contrato
    pub fn key_contract_owner() -> SymbolShort {
        symbol_short!("contract_owner")
    }

    // 8. Mensaje de error personalizado
    pub fn key_custom_error_message() -> String {
        String::from("This is a custom error message")
    }
}