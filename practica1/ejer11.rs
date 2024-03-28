use std::io::{stdin};

fn main() {
    let cadenas = ["Estas", "son", "las", "cinco", "cadenas"];

    let mut cadena_usuario = String::new();
    println!("Ingrese una cadena para comprobar si se encuentra en el arreglo: ");
    stdin().read_line(&mut cadena_usuario).expect("Error al leer.");

    cadena_usuario = cadena_usuario.trim().to_string();

    for cadena in cadenas {
        if cadena == cadena_usuario {
            println!("Cadena encontrada!");
            break;
        }
    }
}