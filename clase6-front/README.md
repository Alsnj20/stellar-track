# Clase 6: Tu Token cobra vida

## Mini-Glosario
- **WASM** : WebAssembly, codigo Rust compilado en un formato que la blockchain puede ejecutar.
- **Contract ID** : Identificador único de tu contrato en la blockchain, como una dirección.
- **Network Passphrase** : Frase que identifica la red de Stellar a la que te conectas (testnet, mainnet, etc).
- **Testnet** : Red de pruebas de Stellar, blockchain gratis.
_  `Public Key (G...)` : Clave pública de una cuenta.
- `Secret Key (S...)` : Clave secreta de una cuenta, no compartir.

## Motivacion de Hoy
Hoy vamos a:
- Darle vida a nuestro token con una interfaz.
- Ver tu token en acción en la blockchain.
- Conectar tu contrato a una wallet para interactuar con él.
- Deployar en TESTNET real, no mas localhost.

## Antes de Sumergirnos

### Asegúrate de tener tu entorno de desarrollo listo (Node, Rust, Stellar SDK, etc).

```bash
# Node.js (necesitas v22+)
node --version

# Stellar CLI (necesitas v22+)
stellar --version

# Plugin Scaffold 
stellar scaffold --version

# Rust y target correcto
rustup show (deberías ver wasm31v1-none)
```

### Verificar el archivo WASM
```bash
ls clase6-front/token_mpj/target/wasm32v1-none/release/token_mpj.wasm
# Deberías ver el archivo token_mpj.wasm
```

### Crear el proyecto Scaffold
```bash
stellar scaffold init my_token_mpj
cd my_token_mpj
```

### Copiar tu contrato WASM
```bash
cp -r ../../clase5-token/token_mpj ./contracts/token_mpj
```

### Instalar dependencias
```bash
npm install
```
#### Adicional
Stellar Freighter-api/Stellar-sdk
```bash
npm install @stellar/freighter-api @stellar/stellar-sdk
```
Verificar instalacion
```bash
npm list @stellar/freighter-api @stellar/stellar-sdk
```

### Configurar variables de entorno
Copiar el archivo .env.example a .env y editar las variables necesarias.

```bash
cp .env.example .env
```

Ver `archivo .env`
```env
# ===================================
# CONFIGURACIÓN DE RED
# ===================================

# Red de Stellar (testnet para práctica, mainnet para producción)
VITE_STELLAR_NETWORK=testnet

# URL del servidor RPC de Stellar
# Testnet: https://soroban-testnet.stellar.org
# Mainnet: https://soroban-mainnet.stellar.org
VITE_STELLAR_RPC_URL=https://soroban-testnet.stellar.org

# ===================================
# TU CONTRATO PSJ
# ===================================

# Contract ID que obtuviste en el deploy (empieza con C)
# IMPORTANTE: Reemplazá con TU Contract ID real
VITE_PSJ_CONTRACT_ID=
```
### Caminos para correr el proyecto
Podemos partir por 3 caminos y de estos depende las cosas que vamos a poder hacer:
```bash
┌─────────────────────────────────────────────────────────┐
│                                                         │
│  🏠 LOCAL (Docker)         🧪 TESTNET        💰 MAINNET│
│  ├── Solo en tu PC         ├── Pública       ├── Real  │
│  ├── Privada               ├── Gratis        ├── $$ XLM│
│  ├── Reiniciable           ├── Permanente    ├── Prod  │
│  └── Desaparece al apagar  └── Compartida    └── Live  │
│                                                         │
└─────────────────────────────────────────────────────────┘
NO están conectadas - Son blockchains diferentes
```
#### Camino 1: Local (Docker)
- Pros: Rápido, fácil de resetear, ideal para desarrollo.
- Contras: No es público, no refleja condiciones reales de la red.
```bash
# 1️⃣ Asegurar que Docker está corriendo
docker ps

# 2️⃣ Si no hay contenedor, iniciarlo manualmente (opcional)
docker run --rm -d -p 8000:8000 --name stellar \
  stellar/quickstart:testing --local --enable-soroban-rpc
# 3. Otra opcion
docker run --rm -it -p 8000:8000 stellar/quickstart:testing --local
```
Si esta primera ves demorara entre 2-10 minutos.

Con esto estamos iniciando una red local de Stellar con soporte para Soroban. Ahora podemos correr nuestro frontend conectado a esta red local.
```bash
# 3️⃣ Verificar environments.toml (ya está configurado por defecto)
# [development.network]
# rpc-url = "http://localhost:8000/rpc"
# run-locally = true
```
Tomate un cafecito este comando hara varias cosas:
1. Compilar tu contrato Rust a WASM.
2. Generar el client TS para interactuar con tu contrato.
3. Levantar un servidor de desarrollo con hot reload para tu frontend.
```bash
# 4️⃣ Desarrollar con hot reload
npm start
# ✅ Inicia Docker automáticamente
# ✅ Compila contratos
# ✅ Despliega en localhost:8000
# ✅ Genera clientes TS
# ✅ Levanta frontend en localhost:5173

# 5️⃣ O solo compilar contratos (sin frontend)
npm run build:contracts

# 6️⃣ Ver logs de la blockchain
docker logs stellar -f

# 7️⃣ Detener Docker cuando termines
docker stop stellar
```

#### Generar el client TS (MAGIA)
Esto es lo más épico del Scaffold. Va a leer tu contrato Rust y crear automáticamente código TypeScript para interactuar con él, npm start ya hace esto y mas pero si solo quieres generar el client sin levantar la blockchain local, puedes correr:
```bash
┌─────────────────────────────────────────┐
│  Contenedor Docker                      │
│  ┌────────────────────────────────────┐ │
│  │ Stellar Core (blockchain engine)   │ │
│  │ Horizon API (REST API)             │ │
│  │ Soroban RPC (smart contracts)      │ │
│  │ PostgreSQL (ledger data)           │ │
│  └────────────────────────────────────┘ │
│                                         │
│  Puerto 8000 → http://localhost:8000    │
└─────────────────────────────────────────┘
```

```bash
npm run build:contracts
```
```bash
build:contracts NECESITA:
├─ Rust/Cargo (para compilar) ✅ Lo tienes
├─ Una blockchain (para desplegar)
│  ├─ Opción A: Docker (local) 🐳
│  └─ Opción B: Testnet (internet) 🌐
```

#### Camino 2: Testnet
- Pros: Pública, gratuita, refleja condiciones reales.
- Contras: Más lenta, no se puede resetear, requiere deploy real.
```bash
# 1️⃣ Configurar environment variable
export STELLAR_SCAFFOLD_ENV=staging

# O en PowerShell:
$env:STELLAR_SCAFFOLD_ENV="staging"

# 2️⃣ Verificar configuración de testnet
# En environments.toml debe existir:
# [staging.network]
# rpc-url = "https://soroban-testnet.stellar.org"
# network-passphrase = "Test SDF Network ; September 2015"

# 3️⃣ Crear/verificar cuenta testnet (si no existe)
stellar keys generate alice --network testnet

# 4️⃣ Financiar cuenta con friendbot
stellar keys fund alice --network testnet

# 5️⃣ Compilar y desplegar en testnet
STELLAR_SCAFFOLD_ENV=staging npm run build:contracts
# ✅ Compila contratos
# ✅ Despliega en testnet
# ✅ Genera clientes TS con testnet contract ID

# 6️⃣ Iniciar frontend (apunta a testnet)
STELLAR_SCAFFOLD_ENV=staging npm start

# 7️⃣ Ver tu contrato en el explorador
# https://stellar.expert/explorer/testnet/contract/[TU_CONTRACT_ID]
```

## Empezamos a Interactuar
Una ves asegurado correr el local, vamos a configurar el frontend. Para estos nos vamos a nuestra billetera de Freighter 
Conect localhost a Freighter
```bash
Setting -> Network -> 
Nombre: local
HORIZON: http://localhost:8000/
STELLAR: http://localhost:8000/rpc
Passphrase: Standalone Network ; February 2017
Friendbot: http://localhost:8000/friendbot

# Invoke the contract
stellar keys use me
stellar network use local
```

## RPC
RPC (Remote Procedure Call) es un protocolo que permite ejecutar funciones en un servidor remoto como si fueran locales. En Stellar, el RPC Server (también llamado Soroban RPC) permite:
- Invocar smart contracts
- Simular transacciones antes de enviarlas
- Consultar datos del estado del contrato
- Es más moderno y específico para smart contracts que Horizon
En palabras simples, el RPC es tu puerta de entrada para interactuar con tus contratos en la blockchain.

