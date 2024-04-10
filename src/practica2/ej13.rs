// TODO: Preguntar si esta bien asi o hay que hacer un sort manual
pub fn ordenar_nombres(names: &mut [String; 4]) {
    names.sort()
}

#[test]
fn test_ordenar_nombres() {
    let mut names = ["Jose".to_string(), "Vicente".to_string(), "Juan".to_string(), "Felipe".to_string()];
    ordenar_nombres(&mut names);

    assert_eq!("Felipe", names[0]);
    assert_eq!("Jose", names[1]);
    assert_eq!("Juan", names[2]);
    assert_eq!("Vicente", names[3]);
}