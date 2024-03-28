use std::io;

fn main() {
    let booleano = false;

    let mut data = String::new();

    println!("Ingrese un valor booleano (true o false):");
    io::stdin().read_line(&mut data).expect("error");

    let data_bool: bool = data.trim().parse().expect("Not a boolean value");

    println!("{} AND {} = {}", booleano, data_bool, booleano & data_bool);
    println!("{} OR {} = {}", booleano, data_bool, booleano | data_bool);
}