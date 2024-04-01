fn main() {
    let arreglo1 = [22, 4, 66, 32, 88];
    let arreglo2 = [31, 12, 14, 49, 21];
    let mut resultados = [0, 0, 0, 0, 0];

    resultados[0] = arreglo1[0] + arreglo2[0];
    resultados[1] = arreglo1[1] + arreglo2[1];
    resultados[2] = arreglo1[2] + arreglo2[2];
    resultados[3] = arreglo1[3] + arreglo2[3];
    resultados[4] = arreglo1[4] + arreglo2[4];

    println!("{:?}", resultados);
}