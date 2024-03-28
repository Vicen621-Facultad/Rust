use std::io;

fn main() {
    let entero: u32 = 10;

    let mut data = String::new();
    println!("Ingrese un numero entero: ");
    io::stdin().read_line(&mut data).expect("error");

    let int_data: u32 = data.trim().parse().expect("not a number!");
    let suma = entero + int_data;
    
    println!("{} ^ 2 = {}", suma, suma * suma);
}