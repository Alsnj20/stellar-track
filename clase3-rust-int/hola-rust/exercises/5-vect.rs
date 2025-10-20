// FUNCTION COUNT NUMBERS MORE THAN 100 IN A VECTOR

pub fn contar_mayores(env: Env, numeros: Vec<u32>) -> u32 {
    let mut counter = 0;
    for numero in numeros.iter() {
      if numero > 100 {
        counter += 1;
      }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_contar_mayores() {
        let numeros = Vec::from_array(&env, [50, 150, 200, 75, 120]);
        let resultado = contar_mayores(env, numeros);
        assert_eq!(resultado, 3);
    }
}