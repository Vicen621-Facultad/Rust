pub fn suma_pares(numbers: [i32; 8]) -> i32 {
    let mut total = 0;

    for i in numbers {
        if i % 2 == 0 {
            total += i;
        }
    }

    total
}

#[test]
fn test_suma_pares_solo_impares() {
    let numbers = [1, 3, 5, 7, 9, 11, 13, 15];
    assert_eq!(suma_pares(numbers), 0);
}

#[test]
fn test_suma_pares_solo_pares() {
    let numbers = [2, 4, 6, 8, 10, 12, 14, 16];
    assert_eq!(suma_pares(numbers), 72);
}

#[test]
fn test_suma_pares_mixto() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(suma_pares(numbers), 20);
}