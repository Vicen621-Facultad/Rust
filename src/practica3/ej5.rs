struct Producto {
    nombre: String,
    precio_bruto: f64,
    id: u32,
}

impl Producto {
    fn new(nombre: String, precio_bruto: f64, id: u32) -> Producto {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_producto() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);
        
        assert_eq!(producto.nombre, "Laptop");
        assert_eq!(producto.precio_bruto, 1000.0);
        assert_eq!(producto.id, 1);
    }

    #[test]
    fn test_calcular_impuestos() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);
        
        assert_eq!(producto.calcular_impuestos(10.0), 100.0);
    }

    #[test]
    fn test_aplicar_descuento() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);
        
        assert_eq!(producto.aplicar_descuento(20.0), 200.0);
    }

    #[test]
    fn test_calcular_precio_total() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);
        
        // Con impuestos y descuento
        assert_eq!(producto.calcular_precio_total(Some(10.0), Some(20.0)), 900.0);
        
        // Sin impuestos ni descuento
        assert_eq!(producto.calcular_precio_total(None, None), 1000.0);
        
        // Solo con impuestos
        assert_eq!(producto.calcular_precio_total(Some(10.0), None), 1100.0);
        
        // Solo con descuento
        assert_eq!(producto.calcular_precio_total(None, Some(20.0)), 800.0);
    }
}
