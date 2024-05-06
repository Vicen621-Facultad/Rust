#[derive(Debug, PartialEq, Eq, Clone)]
enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro
}

impl Color {
    fn es_primario(&self) -> bool {
        match self {
            Color::Rojo | Color::Azul | Color::Amarillo => true,
            _ => false
        }
    }
}

#[derive(Clone)]
struct Auto {
    marca: String,
    modelo: String,
    año: u32,
    precio_bruto: f64,
    color: Color,
}

impl Auto {
    fn new(marca: String, modelo: String, año: u32, precio_bruto: f64, color: Color) -> Auto {
        Auto {
            marca,
            modelo,
            año,
            precio_bruto,
            color
        }
    }

    fn compare(&self, other: &Auto) -> bool {
        self.modelo == other.modelo && 
            self.marca == other.marca && 
            self.año == other.año && 
            self.color == other.color && 
            self.precio_bruto == other.precio_bruto
    }

    fn calcular_precio(&self) -> f64 {
        let mut descuento = 0.0;
        let mut recargo = 0.0;

        if self.color.es_primario() {
            recargo += 0.25;
        } else {
            descuento += 0.1
        }

        if self.marca == String::from("BMW") {
            recargo += 0.15;
        }

        if self.año < 2000 {
            descuento += 0.05;
        }

        descuento = self.precio_bruto * descuento;
        recargo = self.precio_bruto * recargo;

        self.precio_bruto - descuento + recargo
    }
}

struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad: u32,
    autos: Vec<Auto>
}

impl ConcesionarioAuto {
    fn new(nombre: String, direccion: String, capacidad: u32) -> ConcesionarioAuto {
        let autos = Vec::with_capacity(capacidad as usize);

        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad,
            autos,
        }
    }

    fn agregar_auto(&mut self, auto: Auto) -> bool {
        if (self.autos.len() + 1) as u32 > self.capacidad {
            false
        } else {
            self.autos.push(auto);
            true
        }
    }

    fn eliminar_auto(&mut self, auto: &Auto) {
        let position = self.autos.iter().position(|a| a.compare(auto));

        if let Some(index) = position{
            self.autos.remove(index);
        }
    }

    fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        self.autos.iter().find(|a| a.compare(auto))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_es_primario() {
        assert!(Color::Rojo.es_primario());
        assert!(!Color::Verde.es_primario());
        assert!(Color::Azul.es_primario());
        assert!(Color::Amarillo.es_primario());
        assert!(!Color::Blanco.es_primario());
        assert!(!Color::Negro.es_primario());
    }

    #[test]
    fn test_new_auto() {
        let auto = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Azul);

        assert_eq!(auto.marca, "Toyota");
        assert_eq!(auto.modelo, "Corolla");
        assert_eq!(auto.año, 2022);
        assert_eq!(auto.precio_bruto, 25000.0);
        assert_eq!(auto.color, Color::Azul);
    }

    #[test]
    fn test_auto_compare() {
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Azul);
        let auto2 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Azul);
        let auto3 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo);

        assert!(auto1.compare(&auto2));
        assert!(!auto1.compare(&auto3));
    }

    //TODO: Ver porque falla este test
    #[test]
    fn test_auto_calcular_precio() {
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Azul);
        let auto2 = Auto::new("BMW".to_string(), "X5".to_string(), 2000, 50000.0, Color::Rojo);
        let auto3 = Auto::new("Ford".to_string(), "Fiesta".to_string(), 1999, 15000.0, Color::Verde);

        assert_eq!(auto1.calcular_precio(), 31250.0);
        assert_eq!(auto2.calcular_precio(), 70000.0);
        assert_eq!(auto3.calcular_precio(), 12750.0);
    }

    #[test]
    fn test_new_concesionario_auto() {
        let concesionario = ConcesionarioAuto::new("Autos Juan".to_string(), "Calle A".to_string(), 10);

        assert_eq!(concesionario.nombre, "Autos Juan");
        assert_eq!(concesionario.direccion, "Calle A");
        assert_eq!(concesionario.capacidad, 10);
        assert_eq!(concesionario.autos.len(), 0);
    }

    #[test]
    fn test_agregar_auto() {
        let mut concesionario = ConcesionarioAuto::new("Autos Juan".to_string(), "Calle A".to_string(), 2);
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Azul);
        let auto2 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo);

        assert!(concesionario.agregar_auto(auto1.clone()));
        assert!(concesionario.agregar_auto(auto2.clone()));
        assert!(!concesionario.agregar_auto(auto1));
    }

    #[test]
    fn test_eliminar_auto() {
        let mut concesionario = ConcesionarioAuto::new("Autos Juan".to_string(), "Calle A".to_string(), 2);
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Azul);
        let auto2 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo);

        concesionario.agregar_auto(auto1.clone());
        concesionario.agregar_auto(auto2.clone());

        concesionario.eliminar_auto(&auto1);

        assert_eq!(concesionario.autos.len(), 1);
        assert!(concesionario.autos[0].compare(&auto2));
    }

    #[test]
    fn test_buscar_auto() {
        let mut concesionario = ConcesionarioAuto::new("Autos Juan".to_string(), "Calle A".to_string(), 2);
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Azul);
        let auto2 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo);
        let auto3 = Auto::new("BMW".to_string(), "M3 Classic".to_string(), 1999, 20000.0, Color::Negro);

        concesionario.agregar_auto(auto1.clone());
        concesionario.agregar_auto(auto2.clone());

        assert!(concesionario.buscar_auto(&auto1).unwrap().compare(&auto1));
        assert!(concesionario.buscar_auto(&auto2).unwrap().compare(&auto2));
        assert!(concesionario.buscar_auto(&auto3).is_none())
    }
}