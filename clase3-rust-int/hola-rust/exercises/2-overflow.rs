// WHAT DOES EACH CASE PRINT?

// CASO A
let x: u8 = 255;
match x.checked_add(1) {
    Some(valor) => println!("A: {}", valor),
    None => println!("A: Overflow"),
}

// A: Overflow

// CASO B
let y: u32 = 100;
match y.checked_sub(50) {
    Some(valor) => println!("B: {}", valor),
    None => println!("B: Underflow"),
}
// B: 50

// CASO C
let z: u8 = 200;
match z.checked_add(100) {
    Some(valor) => println!("C: {}", valor),
    None => println!("C: Overflow"),
}
// C: Overflow