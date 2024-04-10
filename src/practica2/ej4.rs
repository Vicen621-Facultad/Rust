pub fn cantidad_impares(numbers: [i32; 4]) -> u32 {
    let mut impares = 0;

    for i in numbers {
        if i % 2 != 0 {
            impares += 1;
        }
    }

    impares
}

#[test]
fn test_cantidad_impares_solo_pares() {
    let numbers = [2, 4, 6, 8];
    assert_eq!(cantidad_impares(numbers), 0);
}

#[test]
fn test_cantidad_impares_solo_impares() {
    let numbers = [1, 3, 5, 7];
    assert_eq!(cantidad_impares(numbers), 4);
}

#[test]
fn test_cantidad_impares_mixto() {
    let numbers = [1, 2, 3, 4];
    assert_eq!(cantidad_impares(numbers), 2);
}