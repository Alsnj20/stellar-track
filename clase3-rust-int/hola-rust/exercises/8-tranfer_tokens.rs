// Transfer of tokens

pub fun transfer(env: Env, from: Address, to: Address, amount: u128) -> Result<(), String> {
  // Verificar monto válido
  if amount == 0 {
    return Err("El monto debe ser mayor que cero".to_string());
  }

  // Leer balance
  let from_balance = env.storage()
    .instance()
    .get(&from).
    unwrap_or(0);

  // Verificar balance suficiente
  if from_balance < amount {
    return Err("Balance insuficiente".to_string());
  }

  // Actualizar balance del destinatario
  let to_balance = env.storage()
    .instance()
    .get(&to).
    unwrap_or(0);

  // Restar del remitente
  let new_from_balance = from_balance
    .checked_sub(amount)
    .ok_or("Underflow")?;

  // Sumar al destinatario
  let new_to_balance = to_balance
    .checked_add(amount)
    .ok_or("Overflow")?;

  // Actualizar balances en almacenamiento
  env.storage().set(&from, &new_from_balance);
  env.storage().set(&to, &new_to_balance);

  // Emitir evento
  env.events().publish(symbol_short!("transfer"), (from, to, amount));

  Ok(())
}