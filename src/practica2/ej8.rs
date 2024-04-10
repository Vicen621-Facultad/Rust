pub fn sumar_arreglos(numbers1: [f64; 4], numbers2: [f64; 4]) -> [f64; 4] {
    let mut results = [0.0; 4];

    for i in 0..4 {
        results[i] = numbers1[i] + numbers2[i];
    }

    results
}

#[test]
fn test_sumar_arreglos() {
    let numbers1 = [1.0, 2.0, 3.0, 4.0];
    let numbers2 = [5.0, 6.0, 7.0, 8.0];
    let results = sumar_arreglos(numbers1, numbers2);

    assert_eq!(results[0], 6.0);
    assert_eq!(results[1], 8.0);
    assert_eq!(results[2], 10.0);
    assert_eq!(results[3], 12.0);
}