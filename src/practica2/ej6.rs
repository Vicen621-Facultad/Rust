pub fn longitud_de_cadenas(strings: &[String; 4]) -> [u32; 4] {
    let mut long = [0; 4];

    for i in 0..strings.len() {
        long[i] = strings[i].len() as u32;
    }

    long
}

#[test]
fn test_long_cadenas() {
    let strings = ["Hola".to_string(), "me".to_string(), "llamo".to_string(), "Vicente".to_string()];
    let long = longitud_de_cadenas(&strings);

    assert_eq!(4, long[0]);
    assert_eq!(2, long[1]);
    assert_eq!(5, long[2]);
    assert_eq!(7, long[3]);
}