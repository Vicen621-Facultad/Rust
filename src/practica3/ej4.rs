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

    fn determinar_tipo(&self) -> String {
        String::from("TODO")
    }

    fn calcular_area(&self) -> f64 {
        let semiperimetro = (self.lado1 + self.lado2 + self.lado3) / 2;
        // √s(s - a)(s - b)(s - c)
        ((semiperimetro * (semiperimetro - self.lado1) * (semiperimetro - self.lado2) * (semiperimetro - self.lado3)) as f64).sqrt()
    }

    fn caluclar_perimetro(&self) -> u32 {
        self.lado1 + self.lado2 + self.lado3
    }
}