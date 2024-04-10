pub fn cantidad_en_rango(numbers: [i32; 8], min: i32, max: i32) -> u32 {
    let mut count = 0;
    
    for i in numbers {
        if min <= i && i <= max {
            count += 1;
        }
    }
    
    count 
}

#[test]
fn test_cantidad_en_rango() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];

    assert_eq!(8, cantidad_en_rango(numbers, 1, 8));
    assert_eq!(4, cantidad_en_rango(numbers, 3, 6));
    assert_eq!(2, cantidad_en_rango(numbers, 3, 4));
    assert_eq!(3, cantidad_en_rango(numbers, 5, 7));
}