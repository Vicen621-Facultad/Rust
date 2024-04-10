pub fn multiplicar_valores(numbers: &mut [i32; 4], factor: i32) {
    for i in 0..numbers.len() {
        numbers[i] = numbers[i] * factor;
    }
}

#[test]
fn test_multiplicar_valores() {
    let mut numbers = [2, 4, 6, 8];
    multiplicar_valores(&mut numbers, 2);

    assert_eq!(numbers[0], 4);
    assert_eq!(numbers[1], 8);
    assert_eq!(numbers[2], 12);
    assert_eq!(numbers[3], 16);
}