fn main() {
    const MUL: u8 = 3;
    let mut arreglo = [2, 6, 5, 1, 3, 4];

    for i in 0..5 {
        arreglo[i] = arreglo[i] * MUL;
    }

    println!("{:?}", arreglo);
}