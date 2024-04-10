pub fn cantidad_de_cadenas_mayor_a(strings: &[String; 4], limit: u32) -> u32 {
    let mut count = 0;

    for s in strings {
        if s.len() as u32 > limit {
            count += 1;
        }
    }

    count
}

#[test]
fn test_cantidad_de_cadenas_mayor_a() {
    let strings = ["esto".to_string(), "es".to_string(), "un".to_string(), "test".to_string()];

    assert_eq!(cantidad_de_cadenas_mayor_a(&strings, 1), 4);
    assert_eq!(cantidad_de_cadenas_mayor_a(&strings, 2), 2);
}