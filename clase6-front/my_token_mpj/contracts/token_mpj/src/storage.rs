use soroban_sdk::{contracttype, Address, String};

/// Enum que define todas las claves de almacenamiento

/// Separamos los datos en dos tipos de storage:
/// - Instance Storage: Metadatos globales (más barato)
/// - Persistent Storage: Datos de usuarios (requiere TTL)

#[contracttype]
pub enum DataKey {
    /// Balance de cada usuario - Persistent Storage
    /// Usa Address como key para acceso O(1)
    Balance(Address),
    
    /// Permisos de gasto entre usuarios - Persistent Storage
    /// Tupla (owner, spender) para lookup eficiente
    Allowance(Address, Address),
    
    /// Supply total de tokens - Instance Storage
    /// Contador global de tokens en circulación
    TotalSupply,
    
    /// Dirección del administrador - Instance Storage
    /// Solo esta cuenta puede mintear tokens
    Admin,
    
    /// Metadata del token - Instance Storage
    /// Nombre, símbolo ("BDB", "USDC", etc), decimales
    TokenName,    
    TokenSymbol,    
    Decimals,
    
    /// Flag para verificar inicialización - Instance Storage
    /// Previene re-inicialización del contrato
    Initialized,
}

/// Metadata struct para almacenar información del token
/// Usado en initialize() para pasar múltiples parámetros
#[contracttype]
#[derive(Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}