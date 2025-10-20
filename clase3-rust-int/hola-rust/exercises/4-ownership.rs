// WHICH LINES CAUSE ERRORS>?

// BLOQUE 1: String
let s1 = String::from("rust");
let s2 = s1;
// println!("{}", s1);  // Línea A - ¿Error? SI, s1 ya no tiene el valor
println!("{}", s2);     // Línea B - ¿Error? NO, s2 tiene el valor

// BLOQUE 2: u32
let x: u32 = 10;
let y = x;
println!("{}, {}", x, y);  // Línea C - ¿Error? NO, ambos tienen el valor

// BLOQUE 3: Vec
let v1 = Vec::from([1, 2, 3]);
let v2 = v1;
// let v3 = v1;  // Línea D - ¿Error? SI, v1 ya no es válido