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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rectangulo() {
        let rectangulo = Rectangulo::new(5, 10);
        
        assert_eq!(rectangulo.longitud, 5);
        assert_eq!(rectangulo.ancho, 10);
    }

    #[test]
    fn test_calcular_area() {
        let rectangulo = Rectangulo::new(5, 10);
        
        assert_eq!(rectangulo.calcular_area(), 50);
    }

    #[test]
    fn test_calcular_perimetro() {
        let rectangulo = Rectangulo::new(5, 10);
        
        assert_eq!(rectangulo.calcular_perimetro(), 30);
    }

    #[test]
    fn test_es_cuadrado() {
        let rectangulo1 = Rectangulo::new(5, 10);
        let rectangulo2 = Rectangulo::new(5, 5);
        
        assert!(!rectangulo1.es_cuadrado());
        assert!(rectangulo2.es_cuadrado());
    }
}
