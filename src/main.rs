mod practica2;

fn main() {
    let num = 10;
    let array = [1, 2, 3, 4, 5, 6, 7, 8];
    practica2::ej1::es_par(num);
    practica2::ej2::es_primo(num);
    practica2::ej3::suma_pares(&array);
    practica2::ej4::cantidad_impares(&array);
}
