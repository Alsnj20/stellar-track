# Primer Contrato: Hello Tiburona 🦈💗

## Resumen
Este doc describe el proceso completo de creacion, compilacion y despliegue de un contrato inteligente en Soroban.

## Herramientas Necesarias
- **Stellar CLI**: v23.0.1
- **Rust**: Con target `wasm32v1-none`
- **WSL**: Windows Subsystem for Linux
- **VS Code**: Editor recomendado

## Estructura del Proyecto

```bash
hello-tiburona/
└── src/
|    └── lib.rs
|    └── test.rs
├── Cargo.toml // Archivo de configuración de Rust
|-- Makefile
|-- README.md //Este documento
|-- test_snapshots/
```
## Proceso de Desarrollo
### 1. Verificar la instalacion de Stellar CLI
```bash
stellar --version
```

### 2. Crear el Proyecto
```bash
stellar contract init first-contract
```
3. Revisar la estructura
```bash
ls contracts/hello-tiburona/src
```

### 4. Abrir en VS Code
```bash
code .
```

### 5. Verificar que compila
```bash
cargo check
```
### 6. Compilar a WebAssembly
.wasm es un formato de bytecode que puede ser ejecutado en la máquina virtual de Stellar.Es el resultado de compilar Rust
```bash
stellar contract build
```
#### Optimizar el WASM  
```rust
stellar contract optimize --wasm target/wasm32v1-none/release/hello_tiburona.wasm
```

### 7. Configurar Identidad (Wallet)
```bash
stellar keys generate alice --network testnet
```

#### Fondear con XML en Testnet
```bash
stellar keys fund alice --network testnet
```

### 8. Deployar en Testnet
```bash
stellar contract deploy
```

### 9. Invocar el Contrato
```bash
stellar contract invoke
```

## Conceptos Clave

1. **Mainnet vs Testnet**: La red principal (Mainnet) es donde las transacciones son reales y tienen valor, mientras que la red de prueba (Testnet) es un entorno simulado para pruebas.
Se recomienda utilizar Testnet para el desarrollo y las pruebas antes de implementar en Mainnet.

# Actividades
Se creo un nuevo proyecto llamado "hello-tiburona" el cual contiene un smart contract en Rust.

## Build Contract
Proceso de construcción del contrato:
![Build Contract](../../../img/build.png)

## Test Contract
Test corriendo:
![Test Contract](../../../img/test.png)

## Optimizar Contrato
Proceso de optimización del contrato:
![Optimize Contract](../../../img/optimize.png)