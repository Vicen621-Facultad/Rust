pub fn duplicar_valores(numbers: [f64; 4]) -> [f64; 4] {
    let mut double = [0.0; 4];

    for i in 0..numbers.len() {
        double[i] = numbers[i] * 2.0;
    }

    double
}

#[test]
fn test_duplicar_valores() {
    let numbers = [1.0, 2.0, 3.0, 4.0];
    let double = duplicar_valores(numbers);

    assert_eq!(double[0], 2.0);
    assert_eq!(double[1], 4.0);
    assert_eq!(double[2], 6.0);
    assert_eq!(double[3], 8.0);
}