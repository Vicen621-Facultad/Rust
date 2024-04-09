pub fn es_par(num: i32) -> bool {
    num % 2 == 0
}

#[test]
fn test_true_es_par() {
    let num = 10;
    assert!(es_par(num));
}

#[test]
fn test_false_es_par() {
    let num = 9;
    assert!(!es_par(num));
}