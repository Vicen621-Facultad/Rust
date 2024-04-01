use std::io;

fn main() {
    // No puedo definir una constante de tipo String
    const CADENA: &str = "Cadena de prueba";

    let mut data = String::new();
    println!("Ingrese un caracter: ");
    io::stdin().read_line(&mut data).expect("error");

    let c: char = data.trim().parse().expect("Invalid character!");

    let count = CADENA.matches(c).count();
    println!("{} aparece {} veces en \"{}\"", c, count, CADENA);
}