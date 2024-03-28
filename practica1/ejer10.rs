fn main() {
    let arreglo1 = [22, 4, 66, 32, 88];
    let arreglo2 = [31, 12, 14, 49, 21];
    let mut resultados = [0, 0, 0, 0, 0];

    for i in 0..5 {
        resultados[i] = arreglo1[i] + arreglo2[i];
    }

    println!("{:?}", resultados);
}