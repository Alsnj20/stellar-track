# Proximos Pasos

## Actividad:
Los ejercicios realizados y los retos se encuentran dentro de la carpeta `/exercises`.

## Reflexión
```rust
from_balance -= amount;
to_balance += amount;

set_balance(env, from, from_balance);
set_balance(env, to, to_balance);
emit_transfer_event(env, from, to, amount);
```
**Preguntas:**
1. ¿Este código compila? ¿Por qué sí o no?
   - Respuesta: Sí, el código compila porque sigue las reglas de Rust y no hay errores de sintaxis.
   
2. Si no compila, ¿cómo lo arreglarías?
   - Respuesta: N/A

3. ¿Qué validaciones faltan?
   - Respuesta: Faltan validaciones para asegurar de que `from_balance` y `to_balance` no sean negativos después de la transferencia. También se debería verificar que `amount` sea mayor que cero y que `from` y `to` sean direcciones válidas.
