// Nombre: Vicente García Martí | DNI: 46.645.435 | Discord: Vicen621

// El impuesto se debe pasar como un valor entre el 0 y el 1
pub fn calcular_precio_con_impuestos(cantidades: &[u32], precios: &[f64], impuesto: &f64) -> f64 {
    let mut total = 0.0;

    for i in 0..cantidades.len() {
        let mut total_unidad = cantidades[i] as f64 * precios[i];
        total_unidad += total_unidad * impuesto;
        total += total_unidad;
    }

    total
}

#[test]
fn test_calcular_precio_con_impuestos() {
    let cantidades = [1, 0, 2, 4, 0, 0];
    let precios = [2.0, 1.0, 4.0, 6.0, 8.0, 5.0];
    let impuesto = 0.1; // 10%
    let total = calcular_precio_con_impuestos(&cantidades, &precios, &impuesto);

    assert_eq!(total, 37.4);
}

#[test]
fn test_calcular_precio_con_impuesto_0() {
    let cantidades = [1, 0, 2, 4, 0, 0];
    let precios = [2.0, 1.0, 4.0, 6.0, 8.0, 5.0];
    let impuesto = 0.0; // 0%
    let total = calcular_precio_con_impuestos(&cantidades, &precios, &impuesto);

    assert_eq!(total, 34.0);
}

#[test]
fn test_calcular_precio_sin_productos() {
    let cantidades = [0, 0, 0, 0, 0, 0];
    let precios = [2.0, 1.0, 4.0, 6.0, 8.0, 5.0];
    let impuesto = 0.2; // 20%
    let total = calcular_precio_con_impuestos(&cantidades, &precios, &impuesto);

    assert_eq!(total, 0.0);
}