# Clase 5: Tokens en Stellar

## Mini-Glosario
- **Fungible:** Tokens idénticos e intercambiables entre sí, como dinero (un billete de $10 vale igual que otro)
- **TTL (Time To Live):** Tiempo de vida de los datos almacenados en la blockchain de Stellar antes de que expiren
- **Allowance:** Permiso que le das a otra cuenta para que gaste tus tokens en tu nombre
- **Reentrancy:** Vulnerabilidad donde una función puede ser llamada recursivamente antes de terminar su ejecución anterior
- **Mint:** Crear nuevos tokens de la nada, aumentando el supply total
- **Burn:** Destruir tokens permanentemente, reduciendo el supply total
- **Interoperabilidad:** Capacidad de diferentes sistemas para trabajar juntos sin problemas

## Que es un Token?
Un token es una representación digital de valor que vive en la blockchain. Es como tener dinero digital programable que puede representar cualquier cosa de valor.

## Tipos de Tokens
| Tipo de Token | Descripción | Ejemplos |
|---------------|-------------|----------|
| **Fungible** | Intercambiables entre sí, como monedas. | USD Coin (USDC), Stellar Lumens (XLM) |
| **No Fungible (NFTs)** | Únicos y no intercambiables, representan activos digitales únicos. | Arte digital, coleccionables |
| **Stablecoins** | Tokens cuyo valor está vinculado a un activo estable, como una moneda fiduciaria. | Tether (USDT), DAI |
| **Security Tokens** | Representan activos financieros tradicionales, como acciones o bonos. | Tokens de acciones tokenizadas |
| **Utility Tokens** | Proporcionan acceso a un producto o servicio dentro de un ecosistema. | Tokens de plataformas descentralizadas |

## Estandar CAP-46: El blueprint de Stellar
CAP-46 (Core Advancement Proposal 46) es el estándar de Stellar para tokens fungibles, similar a ERC-20 en Ethereum, pero optimizado para la velocidad y costos bajos de Stellar.

**¿Por qué necesitamos estándares?** 
Permiten la interoperabilidad, CAP-46, define las reglas para crear, transferir y gestionar tokens en Stellar, es decir `tu token funciona automáticamente con wallets como Freighter y cualquier DApp en el ecosistema Stellar`.

### Casos de uso:
1. Integracion con exchanges descentralizados (DEX)
2. Pagos transfronterizos: Stablecoins para tranferencias rápidas y económicas
3. Programas de lealtad: Tokens para recompensas y puntos
4. Remesas: Uso de tokens para enviar dinero internacionalmente con comisiones **casi nulas**

### Funcionalidades clave de CAP-46:
```rust
trait TokenInterface {
    // Metadatos
    fn name() -> String;
    fn symbol() -> String;
    fn decimals() -> u32;
    fn total_supply() -> u128;
    
    // Core
    fn balance(owner: Address) -> u128;
    fn transfer(from: Address, to: Address, amount: u128);
    fn approve(owner: Address, spender: Address, amount: u128);
    fn allowance(owner: Address, spender: Address) -> u128;
    fn transfer_from(spender: Address, from: Address, to: Address, amount: u128);
    
    // Admin
    fn mint(to: Address, amount: u128);
    fn burn(from: Address, amount: u128);
}
```
### Comparación CAP-46 vs ERC-20

| Característica | ERC-20 (Ethereum) | Stellar (Soroban) |
|---------------|-------------------|-------------------|
| **Modelo de Gas** | Dinámico (subastas) | Fijo (predecible) |
| **Lenguaje** | Solidity | Rust |
| **Seguridad** | require/assert | Result<T, Error> |
| **Storage** | Mapping infinito | TTL management |
| **Decimales** | Configurable, típicamente 18 | Configurable, típicamente 7 (alineado con XLM) |
| **Upgrades** | Proxy patterns | Nativo |

Ejemplo de Costo Real
```
Transferencia de Token:
━━━━━━━━━━━━━━━━━━━━━
Ethereum: $5 - $50 (variable)
Stellar:  $0.00001 (fijo)

Diferencia: 500,000x más barato
```

## Conceptos de Seguridad
### 1. Authorization (require_auth): 
Verifica que la cuenta que llama a la función tiene permiso para hacerlo, **no siempre fiarse del `caller()`**, quien invoca la función`.

```rust
fn transfer(from: Address, to: Address, amount: u128) {
    from.require_auth();
    // lógica de transferencia
}
```

### 2.Overflow/Underflow: 
Usar tipos de datos seguros que manejan automáticamente estos casos.
```rust
let new_balance = balance.checked_add(amount)
    .ok_or(TokenError::OverflowError)?;
```

### 3. Reentrancy Prevention 
En Soroban, el modelo de ejecución secuencial reduce naturalmente los riesgos de reentrancy, pero siempre:
- Actualiza el estado ANTES de llamadas externas
- Valida el estado DESPUÉS de operaciones

## Storage en Stellar: Intance vs Persistent

### Instance Storage:
- Metadatos del contrato (admin, name, symbol).
- Compartido entre invocaciones.
- Mas Barato.

### Persistent Storage:
- Datos de usuarios (balances, allowances).
- Especifico por key (ej. balance de una cuenta).
- Requiere manejo de TTL (tiempo de vida de los datos en blockchain).

```rust
// Instance - Metadatos globales
env.storage().instance().set(&DataKey::Admin, &admin);

// Persistent - Datos de usuario
env.storage().persistent().set(&DataKey::Balance(user), &amount);
```

## TTL (Time to Live) Management:
En Stellar, los datos no viven siempre. Debes definir cuánto tiempo deben persistir.

```rust
fn set_balance(account: Address, amount: u128, ttl: u64) {
    env.storage().persistent().extend_ttl(ttl);
}
```

### TTL en diferentes Redes
| Red        | TTL Dafault | Costo por Storage |
|------------|-------------|-------------------|
| **Testnet** | ~30 dias      | Gratis         |
| **Mainnet** | Configurable      | Minimo     |

### Ejemplo: Que pasa si no extiendes TTL?
Si no extiendes el TTL, los datos almacenados en Persistent Storage expirarán después del tiempo definido, lo que puede resultar en la pérdida de balances(sin recuperarse) o allowances si no se renuevan a tiempo.

## Eventos: La caja negra del Blockchain
Los eventos son registros inmutables de acciones que ocurren dentro de un contrato inteligente. Son útiles para auditoría y seguimiento de actividades.
Son criticos para:
- Auditoría y compliance
- Monitoreo de actividades
- Actualizaciones en tiempo real para DApps

### Como consume eventos las UIs?
**Ejemplo práctico**: Una wallet como Freighter escucha el evento `transfer` para actualizar el saldo en la UI sin necesidad de consultar el contrato directamente. Cuando haces una transferencia:

1. El contrato emite evento: transfer(from, to, amount)
2. Freighter detecta el evento en tiempo real
3. La UI actualiza el balance instantáneamente
4. Usuario ve el cambio sin refrescar la página

### Evento Rico vs Basico
```rust
// Evento básico - información limitada
env.events().publish(
    (Symbol::new(&env, "transfer"),),
    amount
);

// Evento rico - información completa
env.events().publish(
    (Symbol::new(&env, "transfer"), from, to),
    (amount, new_from_balance, new_to_balance)
);
```

## Decisiones de Diseños y Trade-offs
`TRADE-OFF`: Decidir que ganar y que perder.

### 1. Transfer a sí mismo
Un caso especial es cuando una cuenta transfiere tokens a sí misma. En este caso, el evento debe reflejar que no hubo cambio neto en el balance.

```rust
// Opción A: Permitir (no-op)
if from == to {
    return Ok(()); // No hacer nada
}

// Opción B: Prohibir (gas-efficient)
if from == to {
    return Err(TokenError::InvalidRecipient);
}
// lógica de transferencia
```
`TRADE-OFF: Permitir trasnferencias a uno mismo mejor la UX, pero puede ser ineficiente en términos de gas (operacion innecesaria).`

### 2. Burn Mechanism
Quemar un token, significa retirarlo de circulacion, osea ya no puede usarse ni transferirse.
```rust
// Opción A: Reduce supply (deflacionario->menos cantidad, el valor de cada una sube)
total_supply = total_supply - burn_amount;

// Opción B: Solo reduce balance
balance = 0; // Supply permanece igual
```
`TRADE-OFF: Reducir el supply total puede afectar la economía del token, mientras que solo reducir el balance puede llevar a confusión sobre el total en circulación.`

### 3. Decimales
Definir la cantidad de decimales afecta la divisibilidad del token.
```rust
// Opción A: 18 decimales (alta divisibilidad)
decimals = 18;
// Opción B: 7 decimales (alineado con XLM)
decimals = 7;
```
`TRADE-OFF: Más decimales permiten mayor precisión en transacciones pequeñas, pero pueden complicar la UX y cálculos.`

## Mejores Practicas Tiburona 🦈
1. **Siempre validar autorización** con `require_auth()`.
    En test para simular autorizaciones usamos `mock_all_auths()`.
2. **Usar Result<T, Error> en lugar de panic!** para errores manejables
3. **Emitir eventos ricos con contexto** para mejor experiencia de usuario
4. **Extender TTL en operaciones críticas** para evitar pérdida de datos
5. **Testear casos edge, no solo happy path**
6. **Documentar decisiones de diseño** en comentarios del código


## 🚨 Anti-Patrones Comunes: 
1. Siempre verificar el admin.
2. Usar Result<T, Error> para manejo de errores.
3. Siempre extender TTL en operaciones que modifican estado crítico.


