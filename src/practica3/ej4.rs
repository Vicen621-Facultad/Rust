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
            TipoTriangulo::Escaleno
        } else {
            TipoTriangulo::Isoceles
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

#[test]
fn test_determinar_tipo() {
    let escaleno = Triangulo::new(4, 7, 7);
    let equilatero = Triangulo::new(4, 4, 4);
    let isoceles = Triangulo::new(4, 6, 7);

    assert_eq!(escaleno.determinar_tipo(), TipoTriangulo::Escaleno);
    assert_eq!(equilatero.determinar_tipo(), TipoTriangulo::Equilatero);
    assert_eq!(isoceles.determinar_tipo(), TipoTriangulo::Isoceles);
}

#[test]
fn test_calcular_area() {
    let triangulo = Triangulo::new(2, 3, 4);

    assert_eq!(triangulo.calcular_area(), 2.9047375096555625);
}

#[test]
fn test_calcular_perimtetro() {
    let triangulo = Triangulo::new(3, 8, 5);

    assert_eq!(triangulo.caluclar_perimetro(), 16);
}