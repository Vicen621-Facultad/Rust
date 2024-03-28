fn main() {
    let tupla = ("test string".to_string(), [1, 2, 3, 4, 5, 6]);

    let mut suma = 0;
    for i in tupla.1 {
        suma += i;
    }

    println!("Cadena: {}", tupla.0);
    println!("Suma del array: {}", suma);
}