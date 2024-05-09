#[derive(Clone, Copy)]
struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

impl<'a> Default for Persona<'a> {
    fn default() -> Self {
        Persona {
            nombre: "default",
            apellido: "default",
            direccion: "default",
            ciudad: "default",
            salario: 100.0,
            edad: 18
        }
    }
}

trait GettersPersona {
    fn get_salario(&self) -> f64;
    fn get_ciudad(&self) -> &str;
    fn get_edad(&self) -> u8;
}

impl<'a> GettersPersona for Persona<'a> {
    fn get_ciudad(&self) -> &str {
        self.ciudad
    }

    fn get_edad(&self) -> u8 {
        self.edad
    }

    fn get_salario(&self) -> f64 {
        self.salario
    }
}

impl<'a> Persona<'a> {
    fn es_salario_mayor(&self, salario: f64) -> bool {
        self.get_salario() > salario
    }

    fn vive_en_ciudad(&self, ciudad: &str) -> bool {
        self.get_ciudad() == ciudad
    }

    fn es_mayor(&self, edad: u8) -> bool {
        self.get_edad() > edad
    }
}

impl PartialEq for Persona<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.nombre == other.nombre &&
            self.apellido == other.apellido &&
            self.direccion == other.direccion &&
            self.ciudad == other.ciudad &&
            self.salario == other.salario &&
            self.edad == other.edad
    }
}

impl Eq for Persona<'_> {}

fn salario_mayor<'a>(vec: &Vec<Persona<'a>>, salario: f64) -> Vec<Persona<'a>> {
    vec.iter().filter(|p| p.es_salario_mayor(salario)).cloned().collect()
}

fn viven_en_ciudad_edad<'a>(vec: &Vec<Persona<'a>>, ciudad: &str, edad: u8) -> Vec<Persona<'a>> {
    vec.iter()
        .filter(|p| p.es_mayor(edad))
        .filter(|p| p.vive_en_ciudad(ciudad))
        .cloned()
        .collect()
}

fn viven_todas_en_ciudad<'a>(vec: &Vec<Persona<'a>>, ciudad: &str) -> bool {
    vec.iter().all(|p| p.vive_en_ciudad(ciudad))
}

fn vive_alguna_en_ciudad<'a>(vec: &Vec<Persona<'a>>, ciudad: &str) -> bool {
    vec.iter().any(|p| p.vive_en_ciudad(ciudad))
}

fn existe_persona<'a>(vec: &Vec<Persona<'a>>, persona: &Persona<'a>) -> bool {
    vec.iter().find(|p| p == &persona).is_some()
}

fn obtener_edades<'a>(vec: &Vec<Persona<'a>>) -> Vec<u8> {
    vec.iter().map(|p| p.get_edad()).collect()
}

//TODO: Hacer
fn mayor_menor_salario<'a>(_vec: &Vec<Persona<'a>>) {
}