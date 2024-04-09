pub fn suma_pares(numeros: &[i32]) -> i32 {
    let mut total = 0;

    for i in numeros {
        if i % 2 == 0 {
            total += i;
        }
    }

    total
}

#[test]
fn test_suma_pares_solo_impares() {
    let array = [1, 3, 5, 7];
    assert_eq!(suma_pares(&array), 0);
}

#[test]
fn test_suma_pares_solo_pares() {
    let array = [2, 4, 6, 8];
    assert_eq!(suma_pares(&array), 20);
}

#[test]
fn test_suma_pares_mixto() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(suma_pares(&array), 30);
}