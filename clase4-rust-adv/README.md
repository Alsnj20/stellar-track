# Clase 4: Avanzando con Rust y Soroban

## Mini-glosario
- **Trait**: "Contrato de comportamiento" - define funciones que un tipo debe tener.
- **Address**: Direccion en blockchain (wallets, contratos).
- **Symbol**: Identificador único para activos en Soroban.
- **Storage**: "BD" de un contrato en la blockchain.
- **Ownable**: Patrón para gestionar propiedad de contratos.

## La Magia de los Traits
Los traits en Rust son como contratos que definen un conjunto de métodos que un tipo debe implementar. Permiten la abstraccion y polimorfismo, facilitando la reutilización de código.

|Concepto |Que hace | Similar a... |
|---------|---------|---------------|
| `struct`  | Define datos (campos) | Clase  |
| `trait`   | Define comportamiento común (metodos) | Interface |
| `impl`    | Anade metodos | Metodos |


Ejemplo:
```rust
fn procesar_todas_donaciones<T: Donacion>(donaciones: Vec<T>) {
    for don in donaciones {
        // ✅ Mismo nombre siempre
        let benef = don.beneficiaria(); 
        let monto = don.monto();
        registrar(benef, monto);
    }
}

// Funciona con CUALQUIER tipo que implemente Donacion:
procesar_todas_donaciones(educacion_vec);
procesar_todas_donaciones(salud_vec);
procesar_todas_donaciones(ambiente_vec);
// Agregar un 4to tipo? CERO cambios en esta función
```

### Diseño Clave
- `<T: Donacion>`: Cualquier tipo `T` que implemente el trait `Donacion`.

### Importancia en Blockchain
1. **Interoperabilidad entre contratos**: Permite que diferentes contratos interactúen sin conocer detalles internos.

2. **Seguridad**: Define comportamientos esperados, reduciendo errores.

3. **Composabilidad**: Facilita la creación de contratos modulares y reutilizables.

#### Ejemplo: Trait Token (Estandar Blockchain)
```rust
trait Token {
  fn balance_of(&self, env: &Env, owner: Address) -> i128;
  fn transfer(&self, env: &Env, from: Address, to: Address, amount: i128) -> Result<(), Error>;
  fn total_supply(&self, env: &Env) -> i128;
}
```

## Patron Ownable: Acceso en Contratos
El patrón Ownable es una práctica común en contratos inteligentes para gestionar la propiedad y controlar el acceso a funciones críticas como (cambiar tasas, pausar contrato, update configuraciones.).

```rust
trait Ownable {
    fn get_owner(&self, env: &Env) -> Address;
    fn transfer_ownership(&mut self, env: &Env, new_owner: Address);
    // Verifica de quien es el owner
    fn require_owner(&mut self, env: &Env);
}
```
### Donde guardamos el owner?
En el storage del contrato. cada dato necesita una "key" unica.

## Claves en el Storage
### Diferencia entre has y get
- `has_<key>`: Verifica si existe un valor para la clave dada.
- `get_<key>`: Recupera el valor asociado a la clave dada.

### Que es TTL?
TTL (Time To Live) es un mecanismo que define la duración de validez de ciertos datos o transacciones en la red. En Soroban, puede usarse para limitar el tiempo durante el cual una transacción es válida. Contiene 2 parámetros:
- **Duración**: Tiempo en segundos que la transacción es válida.
- **Timestamp de inicio**: Momento en que comienza a contar la duración.


## Tipos de Storage en Soroban
|Tipo de Storage | Descripción | Uso Común |
|----------------|-------------|-----------|
| **Instance Storage** | Datos específicos de una instancia de contrato. | Configuraciones del contrato, contadores. |
| **Persistent Storage** | Datos que persisten más allá de la vida de una instancia | Estados de usuario, historiales. |
| **Temporary Storage** | Datos temporales durante la ejecución de una transacción. | Caches, datos intermedios. |

## Test en Soroban
En blockchain, un bug puede costar dinero, Los tests son tu red de seguridad.
Si algo se rompe, lo descubre aqui, no en producción.
```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;
    
    #[test]
    fn test_increment() {
        // ARRANGE: Preparar
        let env = Env::default();
        let contract_id = env.register_contract(None, ContadorContract);
        let client = ContadorContractClient::new(&env, &contract_id);
        
        // ACT & ASSERT: Ejecutar y verificar
        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.get_count(), 2);
    }
    
    #[test]
    fn test_decrement() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ContadorContract);
        let client = ContadorContractClient::new(&env, &contract_id);
        
        // Incrementar primero
        client.increment();
        client.increment();
        client.increment();
        
        // Decrementar
        assert_eq!(client.decrement(), 2);
        assert_eq!(client.get_count(), 2);
    }
    
    #[test]
    #[should_panic(expected = "contador ya está en 0")]
    fn test_decrement_panic() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ContadorContract);
        let client = ContadorContractClient::new(&env, &contract_id);
        
        // Esto DEBE causar panic
        client.decrement();
    }
    
    #[test]
    fn test_reset() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ContadorContract);
        let client = ContadorContractClient::new(&env, &contract_id);
        
        client.increment();
        client.increment();
        client.reset();
        
        assert_eq!(client.get_count(), 0);
    }
}
```

## Soroban CLI
Soroban CLI es la herramienta de línea de comandos para interactuar con contratos Soroban en la red Stellar. Permite compilar, desplegar y administrar contratos inteligentes.

**ACTUALIZACION**: Antes del mes de octubre el comando a ejecutar era soroban, actualmente ya no, el nuevo comando es `stellar`.

```
# Ver todos los comandos
stellar --help

# Compilar contrato
stellar contract build

# Optimizar WASM
stellar contract optimize --wasm <path>

# Deployar a testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_tiburona.wasm \
  --network testnet

# Invocar función
stellar contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- \
  hello \
  --usuario <ADDRESS> \
  --nombre "Ana"
```

## Instalar el target de WebAssembly
```bash
rustup target add wasm32-unknown-unknown
```
## Resumen
- **Soroban + Rust**: Contratos que no fallan