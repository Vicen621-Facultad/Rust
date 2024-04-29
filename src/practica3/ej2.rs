struct Rectangulo {
    longitud: i32,
    ancho: i32,
}

impl Rectangulo {
    fn new(longitud: i32, ancho: i32) -> Rectangulo {
        Rectangulo {
            longitud,
            ancho,
        }
    }

    fn calcular_area(&self) -> i32 {
        self.longitud * self.ancho
    }

    fn calcular_perimetro(&self) -> i32 {
        self.longitud * 2 + self.ancho * 2
    }

    fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

#[test]
fn test_rectangulo() {
    let rectangulo = Rectangulo::new(4, 10);

    assert_eq!(rectangulo.calcular_area(), 40);
    assert_eq!(rectangulo.calcular_perimetro(), 28);
    assert!(!rectangulo.es_cuadrado());
}

#[test]
fn test_cuadrado() {
    let rectangulo = Rectangulo::new(10, 10);

    assert_eq!(rectangulo.calcular_area(), 100);
    assert_eq!(rectangulo.calcular_perimetro(), 40);
    assert!(rectangulo.es_cuadrado());
}