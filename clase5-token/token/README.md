# Token MiPasaje – Guía Completa del Proyecto 🚍

## Historia de MiPasaje

**El Problema:** En las microempresas de transporte de pasajeros no existe un sistema confiable para controlar los pasajes vendidos y utilizados. Los registros son manuales, poco claros o inexistentes, lo que genera: 
- ❌ Pasajeros que se cuelan
- ❌ Falta de control sobre cobradores y unidades, ningún historial verificable de viajes, los pasajes son pagos invisibles: se cobran, pero no dejan evidencia real.
- ❌ Ningún historial verificable de viajes
- ❌ Los pasajes son pagos invisibles: se cobran, pero no dejan evidencia real.

**La Solución:** MiPasaje es un token en Stellar donde **1 MPJ = 1 viaje válido**. Cada pasaje vendido se registra como un token en blockchain, creando un sistema digital, transparente e inalterable que garantiza que todo pasajero que sube, paga, y que todo pago queda registrado.

## Visión del Proyecto

MiPasaje es un sistema de registro descentralizado para microempresas de transporte, construido sobre Stellar blockchain, que permite:

- 🎟️ Emitir pasajes digitales verificables
- 🚫 Evitar que pasajeros se cuelen
- 📊 Llevar control real de ingresos y afluencia
- 🚌 Registrar viajes por ruta, unidad y horario
- 🔍 Auditoría transparente para dueños y autoridades

## Flujo del Sistema
```bash
initialize: admin (configura contrato)
   ↓
mint (recarga): admin (vende pasajes)
   ↓
balance (consulta): user (verifica pasajes)
   ↓
approve (autorizar): user (permite gastar pasajes)
   ↓
transfer_from (pagar pasaje): user2 (gasta pasajes autorizados)
   ↓
burn (validar uso) : admin (elimina pasajes usados)
```

## Arquitectura del Proyecto
```bash
token_bdb/
├── src/
│   ├── lib.rs       # Contract principal
│   ├── storage.rs   # Tipos de almacenamiento
│   ├── errors.rs    # Manejo de errores
│   └── test.rs      # Tests unitarios
├── Cargo.toml       # Configuración optimizada
├── docs/
│   ├── token_codebit_guide.md      # Guía técnica original
│   ├── devpoints_deploy_guide.md   # Guía de deployment completa
│   └── test_changes_doc.md   # Documentación de tests
└── img/
    ├── TestRunning.jpg
    ├── CompilaciónExitosa.jpg
    ├── ReporteHtmlTest.jpg
    └── AliceTransaccionesEjemplo.jpg
```

## Ejecutar Test
Comandos Básicos
# Ejecutar todos los tests
cargo test

# Ver output detallado
cargo test -- --nocapture

# Test específico
cargo test test_transfer

# Tests en modo release (más rápido)
cargo test --release
Cobertura de Tests
# Instalar cargo-tarpaulin (una sola vez)
cargo install cargo-tarpaulin

# Generar reporte de cobertura
cargo tarpaulin --out Html

# Abrir reporte en Windows
start tarpaulin-report.html



