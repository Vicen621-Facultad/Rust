fn main() {
    let tupla = ("test string".to_string(), [1, 2, 3, 4, 5, 6]);

    let suma: u8 = tupla.1.iter().sum();

    println!("Cadena: {}", tupla.0);
    println!("Suma del array: {}", suma);
}