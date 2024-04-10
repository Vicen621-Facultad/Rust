pub fn es_primo(num: i32) -> bool {
    let mut count = 0;
    for i in 1..num + 1 {
        if num % i == 0 {
            count += 1;
        }

        if count > 2 {
            break;
        }
    }

    count <= 2
}

#[test]
fn test_true_es_primo() {
    let num = 97;
    assert!(es_primo(num));
}

#[test]
fn test_false_es_primo() {
    let num = 15;
    assert!(!es_primo(num));
}

