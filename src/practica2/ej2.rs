pub fn es_primo(num: i32) -> bool {
    let mut counter = 0;
    for i in 1..num + 1 {
        if num % i == 0 {
            counter += 1;
        }

        if counter > 2 {
            break;
        }
    }

    counter <= 2
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

