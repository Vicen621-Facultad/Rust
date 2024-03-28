fn main() {
    let arreglo = [44, 57, 83, 69, 24];

    let mut suma = 0;

    for i in arreglo {
        suma += i;
    }

    println!("El valor total del arreglo es: {}", suma);
}