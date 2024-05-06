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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_persona() {
        let persona = Persona::new("Juan".to_string(), 30, Some("Calle A".to_string()));
        
        assert_eq!(persona.nombre, "Juan");
        assert_eq!(persona.edad, 30);
        assert_eq!(persona.direccion, Some("Calle A".to_string()));
    }

    #[test]
    fn test_to_string() {
        let persona_con_direccion = Persona::new("Juan".to_string(), 30, Some("Calle A".to_string()));
        let persona_sin_direccion = Persona::new("Ana".to_string(), 25, None);
        
        let persona_con_direccion_str = persona_con_direccion.to_string();
        let persona_sin_direccion_str = persona_sin_direccion.to_string();
        
        assert_eq!(persona_con_direccion_str, "nombre: Juan, edad: 30, direccion: Calle A");
        assert_eq!(persona_sin_direccion_str, "nombre: Ana, edad: 25, direccion: Desconocida");
    }

    #[test]
    fn test_obtener_edad() {
        let persona = Persona::new("Juan".to_string(), 30, Some("Calle A".to_string()));
        
        assert_eq!(persona.obtener_edad(), 30);
    }

    #[test]
    fn test_actualizar_direccion() {
        let mut persona = Persona::new("Juan".to_string(), 30, Some("Calle A".to_string()));
        
        persona.actualizar_direccion("Calle B".to_string());
        
        assert_eq!(persona.direccion, Some("Calle B".to_string()));
    }
}
