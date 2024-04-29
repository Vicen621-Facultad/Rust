struct Producto {
    nombre: String,
    precio_bruto: f64,
    id: u32,
}

impl Producto {
    fn new(nombre: &str, precio_bruto: f64, id: u32) -> Producto {
        let nombre = nombre.to_owned();
        Producto {
            nombre,
            precio_bruto,
            id
        }
    }

    fn calcular_impuestos(&self, porcentaje_de_impuestos: f64) -> f64 {
        self.precio_bruto * (porcentaje_de_impuestos / 100.0)
    }

    fn aplicar_descuento(&self, porcentaje_de_descuento: f64) -> f64 {
        self.precio_bruto * (porcentaje_de_descuento / 100.0)
    }

    fn calcular_precio_total(&self, porcentaje_de_impuestos: Option<f64>, porcentaje_de_descuento: Option<f64>) -> f64 {
        let impuesto = match porcentaje_de_impuestos {
            Some(impuestos) => self.calcular_impuestos(impuestos),
            None => 0.0
        };
        let descuento = match porcentaje_de_descuento {
            Some(descuento) => self.aplicar_descuento(descuento),
            None => 0.0
        };

        self.precio_bruto + impuesto - descuento
    }
}

#[test]
fn test_caluclar_impuestos() {
    let producto = Producto::new("Chipa", 200.0, 1);

    assert_eq!(producto.calcular_impuestos(10.0), 20.0);
    assert_eq!(producto.calcular_impuestos(100.0), 200.0);
    assert_eq!(producto.calcular_impuestos(42.5), 85.0);
}

#[test]
fn test_aplicar_descuento() {
    let producto = Producto::new("Chipa", 200.0, 1);

    assert_eq!(producto.aplicar_descuento(10.0), 20.0);
    assert_eq!(producto.aplicar_descuento(100.0), 200.0);
    assert_eq!(producto.aplicar_descuento(42.5), 85.0);
}

#[test]
fn test_calcular_precio_total() {
    let producto = Producto::new("Chipa", 200.0, 1);

    assert_eq!(producto.calcular_precio_total(Some(10.0), Some(10.0)), 200.0);
    assert_eq!(producto.calcular_precio_total(Some(21.0), None), 242.0);
    assert_eq!(producto.calcular_precio_total(None, Some(42.5)), 115.0)
}
