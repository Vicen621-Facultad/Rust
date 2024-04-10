pub fn reemplazar_pares(numbers: &mut [i32; 4]) {
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            numbers[i] = -1;
        }
    }
}

#[test]
fn test_reemplazar_pares() {
    let mut numbers = [1, 2, 3, 4];
    reemplazar_pares(&mut numbers);

    assert_eq!(numbers[0], 1);
    assert_eq!(numbers[1], -1);
    assert_eq!(numbers[2], 3);
    assert_eq!(numbers[3], -1);
}