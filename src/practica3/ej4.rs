#[derive(Debug, PartialEq, Eq)]
enum TipoTriangulo {
    Equilatero,
    Isoceles,
    Escaleno
}

struct Triangulo {
    lado1: u32,
    lado2: u32,
    lado3: u32,
}

impl Triangulo {
    fn new(lado1: u32, lado2: u32, lado3: u32) -> Triangulo {
        Triangulo {
            lado1,
            lado2,
            lado3
        }
    }

    fn determinar_tipo(&self) -> TipoTriangulo {
        if self.lado1 == self.lado2 && self.lado2 == self.lado3 {
            TipoTriangulo::Equilatero
        } else if self.lado1 == self.lado2 || self.lado1 == self.lado3 || self.lado2 == self.lado3 {
            TipoTriangulo::Isoceles
        } else {
            TipoTriangulo::Escaleno
        }
    }

    fn calcular_area(&self) -> f64 {
        // semiperimetro
        let s = (self.lado1 + self.lado2 + self.lado3) as f64 / 2.0;
        // Formula de heron = √s(s - a)(s - b)(s - c)
        ((s * (s - self.lado1 as f64) * (s - self.lado2 as f64) * (s - self.lado3 as f64))).sqrt()
    }

    fn caluclar_perimetro(&self) -> u32 {
        self.lado1 + self.lado2 + self.lado3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_triangulo() {
        let triangulo = Triangulo::new(3, 4, 5);
        
        assert_eq!(triangulo.lado1, 3);
        assert_eq!(triangulo.lado2, 4);
        assert_eq!(triangulo.lado3, 5);
    }

    #[test]
    fn test_determinar_tipo_equilatero() {
        let triangulo = Triangulo::new(5, 5, 5);
        
        assert_eq!(triangulo.determinar_tipo(), TipoTriangulo::Equilatero);
    }

    #[test]
    fn test_determinar_tipo_isoceles() {
        let triangulo = Triangulo::new(5, 5, 3);
        
        assert_eq!(triangulo.determinar_tipo(), TipoTriangulo::Isoceles);
    }

    #[test]
    fn test_determinar_tipo_escaleno() {
        let triangulo = Triangulo::new(3, 4, 5);
        
        assert_eq!(triangulo.determinar_tipo(), TipoTriangulo::Escaleno);
    }

    #[test]
    fn test_calcular_area() {
        let triangulo = Triangulo::new(3, 4, 5);
        
        assert_eq!(triangulo.calcular_area(), 6.0);
    }

    #[test]
    fn test_calcular_perimetro() {
        let triangulo = Triangulo::new(3, 4, 5);
        
        assert_eq!(triangulo.caluclar_perimetro(), 12);
    }
}