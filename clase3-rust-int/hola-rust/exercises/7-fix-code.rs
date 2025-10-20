struct TokenInfo {
    name: String,
    symbol: Symbol,
    total_supply: u128,
}

// ✅ EFICIENTE
// Espera una referencia a TokenInfo
fn procesar_bien(info: &TokenInfo) -> u128 {
    verificar_nombre(&info.name);
    verificar_supply(info);
    info.total_supply
}

fn verificar_nombre(name: &String) {
    if name.len() == 0 {
        panic!("Nombre vacío");
    }
}

fn verificar_supply(info: &TokenInfo) {
    if info.total_supply == 0 {
        panic!("Supply no puede ser 0");
    }
}