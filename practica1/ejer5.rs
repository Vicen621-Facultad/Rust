use std::io;

fn main() {
    let mut cadena = "Hola ".to_string();

    let mut data = String::new();
    println!("Ingrese un nombre: ");
    io::stdin().read_line(&mut data).expect("error");

    cadena = cadena + &data;
    println!("Cadena: {}", cadena.to_uppercase());
}