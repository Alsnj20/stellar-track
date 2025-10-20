// VALIDATION WITH RESULT


pub fn validar_cantidad(cantidad: u32) -> Result<(), String> {
    match cantidad {
        0 => Err(String::from("Cantidad no puede ser cero")),
        1..=1000 => Ok(()),
        _ => Err(String::from("Cantidad máxima: 1000")),
    }
}

// use validation
pub fn procesar_cantidad(cantidad: u32) {
    match validar_cantidad(cantidad) {
        Ok(_) => println!("Cantidad procesada: {}", cantidad),
        Err(e) => println!("Error al procesar cantidad: {}", e),
    }
}