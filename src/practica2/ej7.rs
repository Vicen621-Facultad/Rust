pub fn cantidad_de_mayores(numbers: [i32; 8], limit: i32) -> u32 {
    let mut count = 0;

    for i in numbers {
        if i > limit {
            count += 1;
        }
    }

    count
}

#[test]
fn test_cantidad_mayores() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];

    assert_eq!(cantidad_de_mayores(numbers, 8), 0);
    assert_eq!(cantidad_de_mayores(numbers, 5), 3);
    assert_eq!(cantidad_de_mayores(numbers, 4), 4);
    assert_eq!(cantidad_de_mayores(numbers, 0), 8);
}