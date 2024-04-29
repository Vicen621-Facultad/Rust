struct Persona {
    nombre: String,
    edad: i32,
    direccion: Option<String>,
}

impl Persona {
    fn new(nombre: String, edad: i32, direccion: Option<String>) -> Persona {
        Persona {
            nombre, 
            edad, 
            direccion
        }
    }

    fn to_string(&self) -> String {
        let desconocida = String::from("Desconocida");
        let direccion = match &self.direccion {
            Some(direccion) => direccion,
            None => &desconocida
        };
        format!("nombre: {}, edad: {}, direccion: {}", self.nombre, self.edad, direccion)
    }

    fn obtener_edad(&self) -> i32 {
        self.edad
    }

    fn actualizar_direccion(&mut self, nueva_direccion: String) {
        self.direccion = Some(nueva_direccion);
    }
}

#[test]
fn test_persona() {
    let mut persona = Persona::new("Vicente".to_string(), 18, None);

    assert_eq!(persona.to_string(), "nombre: Vicente, edad: 18, direccion: Desconocida");
    assert_eq!(persona.obtener_edad(), 18);

    persona.actualizar_direccion("Calle 7".to_string());

    assert_eq!(persona.to_string(), "nombre: Vicente, edad: 18, direccion: Calle 7");
}