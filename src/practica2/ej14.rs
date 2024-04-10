pub fn incrementar(value: &mut f64) {
    *value += 1.0;
}

#[test]
fn test_incrementar() {
    let mut value = 2.3;
    incrementar(&mut value);

    assert_eq!(3.3, value);
    incrementar(&mut value);
    assert_eq!(4.3, value);
}