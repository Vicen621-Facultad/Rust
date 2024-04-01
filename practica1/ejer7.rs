fn main() {
    const MUL: u8 = 3;
    let mut arreglo = [2, 6, 5, 1, 3, 4];

    arreglo[0] *= MUL;
    arreglo[1] *= MUL;
    arreglo[2] *= MUL;
    arreglo[3] *= MUL;
    arreglo[4] *= MUL;
    arreglo[5] *= MUL;

    println!("{:?}", arreglo);
}