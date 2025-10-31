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
