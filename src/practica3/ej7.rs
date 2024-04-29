#[derive(Debug, PartialEq, Eq)]
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

    //TODO: Ver si esta bien esto
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

    //TODO: Hacer
    fn buscar_auto(&self, auto: &Auto) -> Auto {
        
    }
}