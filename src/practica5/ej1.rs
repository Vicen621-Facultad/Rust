use std::{fmt::{Display, Formatter}, fs::File, io::{self, Read, Write}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Auto {
    marca: String,
    modelo: String,
    año: u32,
    precio_bruto: f64,
    color: Color,
}

struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad: usize,
    autos: Vec<Auto>
}

#[derive(Debug)]
struct ErrorConcesionaria(String);

impl Color {
    fn es_primario(&self) -> bool {
        match self {
            Color::Rojo | Color::Azul | Color::Amarillo => true,
            _ => false
        }
    }

    fn to_string(&self) -> String {
        match self {
            Color::Rojo => String::from("rojo"),
            Color::Verde => String::from("verde"),
            Color::Azul => String::from("azul"),
            Color::Amarillo => String::from("amarillo"),
            Color::Blanco => String::from("blanco"),
            Color::Negro => String::from("negro"),
        }
    }

    fn equals(&self, other: &Color) -> bool {
        self.to_string() == other.to_string()
    }
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

    fn to_string(&self) -> String {
        format!(
            "Marca: {}\nModelo: {}\nAño: {}\nPrecio Bruto: {}\nColor: {}",
            self.marca, self.modelo, self.año, self.precio_bruto, self.color.to_string()
        )
    }

    fn equals(&self, other: &Auto) -> bool {
        self.to_string() == other.to_string()
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

impl ConcesionarioAuto {
    fn new(nombre: String, direccion: String, capacidad: usize) -> ConcesionarioAuto {
        let autos = match std::fs::File::open("test/".to_string() + nombre.as_str() + ".json") {
            Ok(mut file) => {
                let mut buf = String::new();
                //TODO: Preguntar si se puede hacer unwrap en lugar de match
                file.read_to_string(&mut buf).unwrap();
                let autos: Vec<Auto> = serde_json::from_str(&buf).unwrap();

                if autos.len() > capacidad {
                    Vec::with_capacity(capacidad)
                } else {
                    autos
                }
            },
            Err(_) => Vec::with_capacity(capacidad)
        };
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad,
            autos,
        }
    }

    fn escribir_archivo(&self) -> Result<(), io::Error> {
        let mut file = File::create("test/".to_string() + self.nombre.as_str() + ".json")?;
        let serialized = serde_json::to_string(&self.autos)?;
        file.write_all(&serialized.as_bytes())?;
        Ok(())
    }

    fn agregar_auto(&mut self, auto: Auto) -> Result<(), ErrorConcesionaria> {
        if (self.autos.len() + 1) > self.capacidad {
            Err(ErrorConcesionaria(String::from("No hay más lugar en el concesionario")))
        } else {
            self.autos.push(auto);
            let res = self.escribir_archivo();
            match res {
                Ok(_) => Ok(()),
                Err(e) => return Err(ErrorConcesionaria(e.to_string()))                
            }
        }
    }

    fn eliminar_auto(&mut self, auto: &Auto) -> Result<(), ErrorConcesionaria> {
        let position = self.autos.iter().position(|a| a.equals(auto));

        match position {
            Some(pos) => {
                self.autos.remove(pos);
                let res = self.escribir_archivo();
                match res {
                    Ok(_) => Ok(()),
                    Err(e) => return Err(ErrorConcesionaria(e.to_string()))                
                }
            },
            None => return Err(ErrorConcesionaria(String::from("El auto no se encuentra en el concesionario")))
        }
    }

    fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        self.autos.iter().find(|a| a.equals(auto))
    }
}

impl Display for ErrorConcesionaria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
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
        assert!(auto.color.equals(&Color::Azul));
    }

    #[test]
    fn test_auto_equals() {
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Blanco);
        let auto2 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Blanco);
        let auto3 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo);

        assert!(auto1.equals(&auto2));
        assert!(!auto1.equals(&auto3));
    }

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
        let concesionario = ConcesionarioAuto::new("test_new_concesionario_auto".to_string(), "Calle A".to_string(), 10);

        assert_eq!(concesionario.nombre, "test_new_concesionario_auto");
        assert_eq!(concesionario.direccion, "Calle A");
        assert_eq!(concesionario.capacidad, 10);
        assert_eq!(concesionario.autos.len(), 0);
    }

    #[test]
    fn test_new_concesionario_auto_file() {
        let mut concesionario1 = ConcesionarioAuto::new("test_new_concesionario_auto_file".to_string(), "Calle A".to_string(), 10);

        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Azul);
        concesionario1.agregar_auto(auto1).unwrap();

        let mut concesionario2 = ConcesionarioAuto::new("test_new_concesionario_auto_file".to_string(), "Calle A".to_string(), 10);
        let auto2 = Auto::new("BMW".to_string(), "X5".to_string(), 2000, 50000.0, Color::Rojo);
        concesionario2.agregar_auto(auto2).unwrap();

        assert_eq!(concesionario2.nombre, "test_new_concesionario_auto_file");
        assert_eq!(concesionario2.direccion, "Calle A");
        assert_eq!(concesionario2.capacidad, 10);
        assert_eq!(concesionario2.autos.len(), 2);
    }

    #[test]
    fn test_agregar_auto() {
        let mut concesionario = ConcesionarioAuto::new("test_agregar_auto".to_string(), "Calle A".to_string(), 2);
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Azul);
        let auto2 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo);

        assert!(concesionario.agregar_auto(auto1.clone()).is_ok());
        assert!(concesionario.agregar_auto(auto2.clone()).is_ok());
        assert!(concesionario.agregar_auto(auto1.clone()).is_err());
    }

    #[test]
    fn test_eliminar_auto() {
        let mut concesionario = ConcesionarioAuto::new("test_eliminar_auto".to_string(), "Calle A".to_string(), 2);
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Amarillo);
        let auto2 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo);
        let auto3 = Auto::new("Audi".to_string(), "A3".to_string(), 2020, 50000.0, Color::Verde);

        let _ = concesionario.agregar_auto(auto1.clone());
        let _ = concesionario.agregar_auto(auto2.clone());

        assert!(concesionario.eliminar_auto(&auto1).is_ok());
        assert!(concesionario.eliminar_auto(&auto3).is_err());
        assert_eq!(concesionario.autos.len(), 1);
        assert!(concesionario.autos[0].equals(&auto2));
    }

    #[test]
    fn test_buscar_auto() {
        let mut concesionario = ConcesionarioAuto::new("test_buscar_auto".to_string(), "Calle A".to_string(), 2);
        let auto1 = Auto::new("Toyota".to_string(), "Corolla".to_string(), 2022, 25000.0, Color::Azul);
        let auto2 = Auto::new("BMW".to_string(), "X5".to_string(), 2020, 50000.0, Color::Rojo);
        let auto3 = Auto::new("BMW".to_string(), "M3 Classic".to_string(), 1999, 20000.0, Color::Negro);

        let _ = concesionario.agregar_auto(auto1.clone());
        let _ = concesionario.agregar_auto(auto2.clone());

        assert!(concesionario.buscar_auto(&auto1).unwrap().equals(&auto1));
        assert!(concesionario.buscar_auto(&auto2).unwrap().equals(&auto2));
        assert!(concesionario.buscar_auto(&auto3).is_none())
    }
}