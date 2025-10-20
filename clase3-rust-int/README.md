# Clase 3: Introducción a Rust para Soroban
## Objetivos
- Comprender los conceptos básicos de Rust.
- Configurar el entorno de desarrollo para Soroban.
- Escribir y compilar un contrato inteligente simple en Rust para Soroban.

## Herramientas necesarias
- **rustc**: El compilador de Rust. `rustc --version` para verificar la instalación.
- **cargo**: El gestor de paquetes y compilación de Rust. `cargo --version` para verificar la instalación.
- **cli-stellar**: Herramienta para desarrollar en la red Stellar. `stellar --version` para verificar la instalación.

## Palabras clave de Rust
- **let**: Declaración de variables.
- **fn**: Declaración de funciones.
- **struct**: Definición de estructuras de datos.
- **impl**: Implementación de métodos para estructuras.
- **mod**: Definición de módulos.
- **use**: Importación de módulos o elementos específicos.
- **pub**: Visibilidad pública para funciones, estructuras o módulos.
- **borrowing**: Prestar sin dar la propiedad.
- **ownership**: Propiedad única de los datos.

## Palabras claves en Soroban SDK
- **no_std**: No usar la biblioteca estándar de Rust.
- **Env**: Módulo para interactuar con el entorno de ejecución de Soroban.
- **Symbol**: Tipo de dato optimizado para cadenas cortas y únicas.
- **panic!**: Macro para detener la ejecución y reportar un error.
- **rollback**: Revertir cambios en caso de error.
- **contract/contractimpl**: Macros para definir e implementar contratos inteligentes.
- **WASM**: Formato de código ejecutable en la blockchain.

[!NOTE]: Todas las funciones reciben `env::Env` como parámetro.

## Tipos de datos para Smart Contracts (Rust + Soroban)

### Datos Numéricos
- **u8**: 
  - 0 a 255
  - Edades, contadores pequeños.
  - Uso: `let age: u8 = 25;`
- **u32**:
  - 0 a 4 mil millones.
  - Cantidades grandes, identificadores.
  - Uso: `let id: u32 = 1;`
- **u128**:
  - 0 a 340 undecillones.
  - Balance de tokens, cantidades masivas.
  - Uso: `let balance: u128 = 1000000000000000000;`

NOTA: 
  - u significa sin signo (positivo).
  - i significa con signo (positivo y negativo).

#### PROBLEMAS:
- **Overflow**: Cuando un número excede su valor máximo.
- **Underflow**: Cuando un número es menor que su valor mínimo.
Ejemplo de Overflow:
```rust
let x: u8 = 255;
let y = x + 1; // ❌ Overflow
```

### Datos de Texto
- **String**:
  - Texto dinámico.
  - Uso: Nombres, descripciones.
  - Ejemplo: `let name: String = String::from("Alice");`

- **Symbol**:
  - Limitacion: Máximo 10 bytes.
  - Texto inmutable y único.
  - Uso: Claves, identificadores.
  - Ejemplo: `let key: Symbol = Symbol::new("my_key");`

### Listas Dinamicas (Vectors)
- **Vec<T>**:
  - Colección dinámica de elementos del mismo tipo.
  - Uso: Listas de usuarios, transacciones.
  - Nota: Siempre se debe inicializar con el entorno `env`.

  - Ejemplo:
  ```rust
  use soroban_sdk::Vec;
  let mut lista: Vec<u32> = Vec::new(&env);
  lista.push_back(10);
  lista.push_back(20);

  let primero = lista.get(0); // Some(10)
  let longitud = lista.len(); // 2
  ```

### Structs y Nums
- **Structs** : 
  - Agrupación de datos, cada uno con su propio tipo.
  - Uso: Representar entidades complejas.
- **Enums** : 
  - Definición de tipos con variantes.
  - Uso: Representar estados de un contrato.

## Stack y Heap
- **Stack**:
  - Almacenamiento de datos de tamaño fijo y vida corta.
  - Acceso rápido.
  - Ejemplo: Variables locales, tipos primitivos.

- **Heap**:
  - Almacenamiento de datos de tamaño variable y vida larga.
  - Acceso más lento.
  - Ejemplo: Strings, Vectors, Structs.

## Ownership y Borrowing

### Ownership
Garantiza seguridad de memoria sin garbage collector.

Reglas:
1. Cada valor en Rust tiene un **owner**.
2. Solo puede haber un **owner** a la vez.
3. Cuando el **owner** sale del alcance, el valor se libera "destrucción".

Como funciona segun el tipo de dato:
- **Heap**:
  - El valor se almacena en el heap.
  - El puntero al valor se almacena en el stack.
  - Al transferir ownership, el puntero se mueve al nuevo owner.
  - Ejemplo:
  ```rust
  let s1 = String::from("hello");
  let s2 = s1; // ownership transferida a s2
  // println!("{}", s1); // ❌ Error: s1 ya no es válido
  ```

- **Stack**: 
  - El valor se almacena directamente en el stack.
  - Al transferir ownership, se copia el valor al nuevo owner.
  - Ejemplo:
  ```rust
  let x = 5;
  let y = x; // copia el valor
  println!("{}", x); // ✅ x sigue siendo válido
  ```
NOTA: Previene bugs de memoria comunes en otros lenguajes.

### Borrowing
Permite referencias a un valor sin transferir ownership. 

Reglas:
1. Puedes tener múltiples referencias inmutables o una mutable.
2. Las referencias deben ser válidas mientras se usan.

#### Referencias Inmutables (&T)
Permiten leer el valor sin modificarlo. **Multiples referencias**.
Uso: Lectura de datos.
Ejemplo:
```rust
let s = String::from("hello");
let len = calculate_length(&s); // pasa una referencia inmutable
println!("La longitud de '{}' es {}", s, len);
fn calculate_length(s: &String) -> usize {
    s.len()
}

```

Regla de oro:

🔹 En la definición → text: &Tipo significa "esperar una referencia".

🔹 En la llamada → &text significa "pasar una referencia".

#### Referencias Mutables (&mut T)
Permiten modificar el valor. **Solo una referencia**.
Uso: Modificación de datos.
Ejemplo:
```rust
let mut s = String::from("hello");
change(&mut s); // pasa una referencia mutable
println!("{}", s); // "hello, world"

fn change(s: &mut String) {
    s.push_str(", world");
}
```
NOTA: Evita condiciones de carrera y garantiza seguridad en concurrencia.

## Pattern Matching
Permite comparar un valor contra una serie de patrones y ejecutar código basado en el patrón que coincida.

### Match Statement
Ejemplo:
```rust
fn showState(state: u8) {
    match state {
        0 => println!("Estado inicial"),
        1 => println!("Estado en progreso"),
        2 => println!("Estado completado"),
        _ => println!("Estado desconocido"),
    }
}
```
NOTA: El compilador verifica que todos los casos estén cubiertos.
Si falta uno, el código no compila. Garantizando **exhaustividad**.

### Option - Goodbye Null
Representa un valor que puede estar presente o ausente (sin crashes de null)
Ejemplo:
```rust
enum Option<T> {
    Some(T),
    None,
}

fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

let user = find_user(2);
match user {
    Some(name) => println!("Usuario encontrado: {}", name),
    None => println!("Usuario no encontrado"),
}
```

#### Funciones útiles de Option
- `unwrap_or(default)`: Devuelve el valor si está presente, o el valor por defecto.
- `unwrap_or_else(func)`: Devuelve el valor si está presente, o llama a la función para obtener un valor por defecto.
- `map(func)`: Aplica una función al valor si está presente, devolviendo una nueva Option.

### Result - Manejo de Errores
Representa el resultado de una operación que puede tener éxito o fallar. Evita la propagacion de **panics** o **?**.
Ejemplo:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: u32, b: u32) -> Result<u32, String> {
    if b == 0 {
        Err(String::from("División por cero"))
    } else {
        Ok(a / b)
    }
}

let result = divide(10, 0);
match result {
    Ok(value) => println!("Resultado: {}", value),
    Err(e) => println!("Error: {}", e),
}
```

NOTA: En Smart Contracts, deben estar preparados para manejar errores de manera segura y predecible. Los Pattern Matching te permiten hacerlo.

## Entorno Soroban
El entorno de Soroban **env** es la interfaz principal para interactuar con la blockchain de Stellar. Proporciona acceso a funcionalidades como almacenamiento, eventos, criptografía y memoria dinámica.

## Eventos en Soroban
Cada vez que un contrato inteligente realiza una acción significativa, **debe** emitir un evento para notificar a los observadores externos sobre esa acción.
```rust
//En soroban 
let text = String::from_str(&env, "mi_evento"); // SI FUNCIONA
```

### Cómo emitir eventos
Para emitir un evento en Soroban, se utiliza el método `env.events().publish()`. Este método toma dos argumentos: un `Symbol` que representa el nombre del evento y un valor que contiene los datos del evento (un dato, multiples datos, o una estructura).
Ejemplo:
```rust
env.events().publish(
    Symbol::new("transferencia_realizada"),
    (from_account, to_account, amount),
);
```
Buenas practicas para eventos:
1. Define nombres claros y descriptivos para los eventos.
2. Incluye toda la información relevante en los datos del evento.
3. Documenta tus eventos para facilitar la integración con aplicaciones externas.

### Otros eventos
- `env.storage()`: Permite almacenar y recuperar datos persistentes en el contrato.
```rust
// Guardar
env.storage().instance().set(&key, &valor);

// Leer
let valor = env.storage().instance().get(&key);
```

- `env.crypto()`: Proporciona funciones criptográficas como hashing y firmas digitales.
```rust
let hash = env.crypto().sha256(&data);
```

## Pasos para usar Soroban
Leer -> Validar -> Modificar -> Guardar -> Emitir es la base de TODOS los smart contracts.
``` rust
// Importar las dependencias necesarias
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol::Symbol, symbol_short!};

// Definir el contrato inteligente
pub struct ContadorContract;

// Implementar las funciones del contrato
impl ContadorContract {
    // PASO 1: Leer del storage
    pub fn incrementar(env: Env) -> u32 {
        let mut valor: u32 = env.storage().instance().get(&symbol_short!("contador")).unwrap_or(0);
        // PASO 2: Incrementar el valor
        valor + 1
        // PASO 3: Guardar en el storage
        env.storage().instance().set(&symbol_short!("contador"), &nuevo_valor);
        // PASO 4: Emitir un evento
        env.events().publish(
            symbol_short!("contador_incrementado"),
            nuevo_valor,
        );

        // PASO 5: Devolver el nuevo valor
        nuevo_valor
    }

    pub fn decrement(env: Env) -> u32 {
        let key = symbol_short!("COUNTER");
        let mut contador = env.storage().instance()
            .get(&key)
        .unwrap_or(0);

    // VALIDAR: Prevenir underflow
    if contador == 0 {
        // Emitir evento de error
        panic!("No se puede decrementar desde 0");
    }

    contador -= 1;
    env.storage().instance().set(&key, &contador);
    env.events().publish(
        (symbol_short!("decrement"),),
        contador
    );
    contador
    }

    pub fn get_counter(env: Env) -> u32 {
        env.storage().instance()
            .get(&symbol_short!("COUNTER"))
            .unwrap_or(0)
    }

    pub fn reset(env: Env) {
        env.storage().instance().set(&symbol_short!("COUNTER"), &0);
        env.events().publish(
            (symbol_short!("reset"),),
            0
        );
    }
}
```

NOTA:
- `#![no_std]`: Indica que no se usará la biblioteca estándar de Rust(threads, filesystem, networking), no existe en blockchain.
- `use soroban_sdk::{...}`: Importa los módulos necesarios del SDK de Soroban para desarrollar contratos inteligentes.
- `contract`: Struct que define el contrato inteligente.
- `contractimpl`: Macro que se utiliza para implementar las funciones del contrato.
- `Env`: Ambiente - Tu interfaz para interactuar con la blockchain de Soroban, siempre primer parámetro en funciones.
- `Symbol`: Tipo de dato optimizado para representar cadenas cortas y únicas, utilizado comúnmente para nombres de eventos y claves.
- `symbol_short!`: Macro para crear símbolos de manera eficiente.
- `pub struct MiContrato;`: Define una estructura vacía que representa el contrato inteligente sirve como **contenedor**
- `unwrap_or(0)`: En Smart Contracts, si no esta inicializado, devuelve 0 para evitar errores equivalente a None.
- `valor + 1`: Incrementa el valor leído del storage, en produccion se debe validar que no haya overflow para solucionar `valor.checked_add(1)`