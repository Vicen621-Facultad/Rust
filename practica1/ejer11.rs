use std::io::{stdin};

fn main() {
    let cadenas = ["Estas".to_string(), "son".to_string(), "las".to_string(), "cinco".to_string(), "cadenas".to_string()];

    let mut cadena_usuario = String::new();
    println!("Ingrese una cadena para comprobar si se encuentra en el arreglo: ");
    stdin().read_line(&mut cadena_usuario).expect("Error al leer.");

    cadena_usuario = cadena_usuario.trim().to_string();

    if cadenas.contains(&cadena_usuario) {
        println!("Cadena encontrada!");   
    }
}