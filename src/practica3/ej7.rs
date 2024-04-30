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
    fn new(marca: &str, modelo: &str, año: u32, precio_bruto: f64, color: Color) -> Auto {
        let marca = marca.to_owned();
        let modelo = modelo.to_owned();

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

        if self.marca == "BMW" {
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
    fn new(nombre: &str, direccion: &str, capacidad: u32) -> ConcesionarioAuto {
        let nombre = nombre.to_owned();
        let direccion = direccion.to_owned();
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
        let mut index = -1;
        for i in 0..self.autos.len() {
            match self.autos.get(i) {
                Some(auto_comp) => if auto_comp.compare(&auto) { index = i as i32; }
                _ => {}
            }
        }

        if index != -1 {
            self.autos.remove(index as usize);
        }
    }

    //REVIEW: Preguntar si esta bien la manera de extraer en el opt despues del if
    fn buscar_auto(&self, auto: &Auto) -> Option<Auto> {
        let mut opt = None;

        for i in 0..self.autos.len() {
            if self.autos.get(i).unwrap().compare(auto) {
                opt = Some(self.autos.get(i).unwrap().to_owned())
            }
        }

        opt
    }
}

#[test]
fn test_calcular_precio() {
    let audi = Auto::new("Audi", "TT", 2020, 20000.0, Color::Rojo);
    let bmw = Auto::new("BMW", "M3 Classic", 1999, 20000.0, Color::Azul);

    assert_eq!(audi.calcular_precio(), 25000.0);
    assert_eq!(bmw.calcular_precio(), 27000.0)
}

#[test]
fn test_compare() {
    let audi = Auto::new("Audi", "TT", 2020, 20000.0, Color::Rojo);
    let audi2 = Auto::new("Audi", "TT", 2020, 20000.0, Color::Rojo);
    let bmw = Auto::new("BMW", "M3 Classic", 1999, 20000.0, Color::Azul);

    assert!(audi.compare(&audi2));
    assert!(audi2.compare(&audi));
    assert!(!audi.compare(&bmw));
    assert!(!bmw.compare(&audi));
}

#[test]
fn test_agregar_auto() {
    let mut concesionaria = ConcesionarioAuto::new("test", "test", 4);
    let audi = Auto::new("Audi", "TT", 2020, 20000.0, Color::Rojo);
    let audi2 = Auto::new("Audi", "TT", 2020, 20000.0, Color::Azul);
    let bmw = Auto::new("BMW", "M3 Classic", 1999, 20000.0, Color::Azul);
    let bmw2 = Auto::new("BMW", "M3 Classic", 1999, 20000.0, Color::Verde);
    let bmw3 = Auto::new("BMW", "M3 Classic", 1999, 20000.0, Color::Negro);
    assert!(concesionaria.agregar_auto(audi));
    assert!(concesionaria.agregar_auto(audi2));
    assert!(concesionaria.agregar_auto(bmw));
    assert!(concesionaria.agregar_auto(bmw2));
    assert!(!concesionaria.agregar_auto(bmw3));
    assert_eq!(concesionaria.autos.len(), 4);
}

#[test]
fn test_buscar_auto() {
    let mut concesionaria = ConcesionarioAuto::new("test", "test", 4);
    let audi = Auto::new("Audi", "TT", 2020, 20000.0, Color::Rojo);

    concesionaria.agregar_auto(audi);

    let audi = Auto::new("Audi", "TT", 2020, 20000.0, Color::Rojo);
    assert!(concesionaria.buscar_auto(&audi).is_some());
    let audi_azul = Auto::new("Audi", "TT", 2020, 20000.0, Color::Azul);
    assert!(concesionaria.buscar_auto(&audi_azul).is_none());
}

#[test]
fn test_eliminar_auto() {
    let mut concesionaria = ConcesionarioAuto::new("test", "test", 4);
    let audi = Auto::new("Audi", "TT", 2020, 20000.0, Color::Rojo);

    concesionaria.agregar_auto(audi);
    let audi = Auto::new("Audi", "TT", 2020, 20000.0, Color::Rojo);
    concesionaria.eliminar_auto(&audi);
    assert_eq!(concesionaria.autos.len(), 0);
    assert!(concesionaria.buscar_auto(&audi).is_none());
}