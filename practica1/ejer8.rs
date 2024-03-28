use std::io;

fn main() {
    // No puedo definir una constante de tipo String
    const CADENA: &str = "Cadena de prueba";

    let mut data = String::new();
    println!("Ingrese un caracter: ");
    io::stdin().read_line(&mut data).expect("error");

    let c: char = data.trim().parse().expect("Invalid character!");

    let mut count = 0;
    for s in CADENA.chars() {
        if s == c {
            count += 1;
        }
    }

    println!("{} aparece {} veces en \"{}\"", c, count, CADENA);
}