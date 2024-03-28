use std::io;

fn main() {
    let decimal = 100.5;
    let mut data = String::new();

    // Leo de teclado
    println!("Ingrese un numero decimal: ");
    io::stdin().read_line(&mut data).expect("error");

    let data_decimal: f64 = data.trim().parse().expect("No es un numero decimal!");

    println!("{} + {} = {}", decimal, data_decimal, decimal + data_decimal);
    println!("{} - {} = {}", decimal, data_decimal, decimal - data_decimal);
    println!("{} * {} = {}", decimal, data_decimal, decimal * data_decimal);
    println!("{} / {} = {}", decimal, data_decimal, decimal / data_decimal);
}