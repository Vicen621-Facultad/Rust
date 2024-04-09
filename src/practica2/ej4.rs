pub fn cantidad_impares(numeros: &[i32]) -> i32 {
    let mut impares = 0;

    for i in numeros {
        if i % 2 != 0 {
            impares += 1;
        }
    }

    impares
}

#[test]
fn test_cantidad_impares_solo_pares() {
    let array = [2, 4, 6, 8, 10];
    assert_eq!(cantidad_impares(&array), 0);
}

#[test]
fn test_cantidad_impares_solo_impares() {
    let array = [1, 3, 5, 7];
    assert_eq!(cantidad_impares(&array), 4);
}

#[test]
fn test_cantidad_impares_mixto() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    assert_eq!(cantidad_impares(&array), 6);
}