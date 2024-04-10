mod practica2;

fn main() {
    let num = 10;
    let mut value = 10.0;
    let array8 = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut array4 = [1, 2, 3, 4];
    let arrayf = [1.0, 2.0, 3.0, 4.0];
    let mut arrays = ["esto".to_string(), "es".to_string(), "un".to_string(), "test".to_string()];
    
    practica2::ej1::es_par(num);
    practica2::ej2::es_primo(num);
    practica2::ej3::suma_pares(array8);
    practica2::ej4::cantidad_impares(array4);
    practica2::ej5::duplicar_valores(arrayf);
    practica2::ej6::longitud_de_cadenas(&arrays);
    practica2::ej7::cantidad_de_mayores(array8, 4);
    practica2::ej8::sumar_arreglos(arrayf, arrayf);
    practica2::ej9::cantidad_en_rango(array8, 2, 5);
    practica2::ej10::cantidad_de_cadenas_mayor_a(&arrays, 2);
    practica2::ej11::multiplicar_valores(&mut array4, 2);
    practica2::ej12::reemplazar_pares(&mut array4);
    practica2::ej13::ordenar_nombres(&mut arrays);
    practica2::ej14::incrementar(&mut value);
}
