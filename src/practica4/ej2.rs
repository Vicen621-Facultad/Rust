use std::cmp::Ordering;

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
    fn new(nombre: &'a str, apellido: &'a str, direccion: &'a str, ciudad: &'a str, salario: f64, edad: u8) -> Persona<'a> {
        Persona {
            nombre,
            apellido,
            direccion,
            ciudad,
            salario,
            edad
        }
    }



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

fn salario_mayor<'a>(vec: &'a Vec<Persona>, salario: f64) -> Vec<Persona<'a>> {
    vec.iter().filter(|p| p.es_salario_mayor(salario)).cloned().collect()
}

fn viven_en_ciudad_edad<'a>(vec: &'a Vec<Persona>, ciudad: &str, edad: u8) -> Vec<Persona<'a>> {
    vec.iter()
        .filter(|p| p.es_mayor(edad))
        .filter(|p| p.vive_en_ciudad(ciudad))
        .cloned()
        .collect()
}

fn viven_todas_en_ciudad<'a>(vec: &'a Vec<Persona>, ciudad: &str) -> bool {
    vec.iter().all(|p| p.vive_en_ciudad(ciudad))
}

fn vive_alguna_en_ciudad<'a>(vec: &'a Vec<Persona>, ciudad: &str) -> bool {
    vec.iter().any(|p| p.vive_en_ciudad(ciudad))
}

fn existe_persona<'a>(vec: &'a Vec<Persona>, persona: &Persona<'a>) -> bool {
    vec.iter().find(|p| p == &persona).is_some()
}

fn obtener_edades<'a>(vec: &'a Vec<Persona>) -> Vec<u8> {
    vec.iter().map(|p| p.get_edad()).collect()
}

fn mayor_menor_salario<'a>(vec: &'a Vec<Persona>) -> Option<(&'a Persona<'a>, &'a Persona<'a>)> {
    if vec.is_empty() {
        return None;
    }

    let (menor, mayor) = vec.iter().fold(
        (None, None),
        |(min, max): (Option<&Persona<'a>>, Option<&Persona<'a>>), persona| {
            let min = match min {
                Some(min) if persona.get_salario() > min.get_salario() || (persona.get_salario() == min.get_salario() && min.es_mayor(persona.get_edad())) => min,
                _ => persona,
            };
            let max = match max {
                Some(max)if persona.get_salario() < max.get_salario() || (persona.get_salario() == max.get_salario() && max.es_mayor(persona.get_edad())) => max,
                _ => persona,
            };
            (Some(min), Some(max))
        },
    );

    Some((menor.unwrap(), mayor.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ciudad() {
        let p = Persona::default();
        assert_eq!(p.get_ciudad(), "default");
    }

    #[test]
    fn test_get_edad() {
        let p = Persona::default();
        assert_eq!(p.get_edad(), 18);
    }

    #[test]
    fn test_get_salario() {
        let p = Persona::default();
        assert_eq!(p.get_salario(), 100.0);
    }

    #[test]
    fn test_es_salario_mayor() {
        let p = Persona::default();
        assert!(!p.es_salario_mayor(200.0));
        assert!(!p.es_salario_mayor(100.0));
        assert!(p.es_salario_mayor(50.0));
    }

    #[test]
    fn test_vive_en_ciudad() {
        let p = Persona::default();
        assert!(p.vive_en_ciudad("default"));
    }

    #[test]
    fn test_es_mayor() {
        let p = Persona::default();
        assert!(!p.es_mayor(20));
        assert!(!p.es_mayor(18));
        assert!(p.es_mayor(17));
    }

    #[test]
    fn test_persona_eq() {
        let p1 = Persona::default();
        let p2 = Persona::new(
            "test",
            "test",
            "test",
            "test",
            200.0,
            20
        );

        assert!(p1 == p1);
        assert!(p1 != p2);
    }

    #[test]
    fn test_salario_mayor() {
        let mut personas = vec![Persona::default(); 5];
        personas.push(Persona::new("test", "test", "test", "test", 200.0, 18));
        personas.push(Persona::new("test", "test", "test", "test", 300.0, 18));
        let salario_mayor = salario_mayor(&personas, 100.0);

        assert_eq!(salario_mayor.len(), 2);
        assert_eq!(salario_mayor[0].get_salario(), 200.0);
        assert_eq!(salario_mayor[1].get_salario(), 300.0);
    }

    #[test]
    fn test_viven_en_ciudad_edad() {
        let mut personas = vec![Persona::default(); 5];
        personas.push(Persona::new("test", "test", "test", "test", 200.0, 20));
        personas.push(Persona::new("test", "test", "test", "test", 300.0, 22));
        personas.push(Persona::new("test", "test", "test", "test", 300.0, 12));

        let viven_en_ciudad_edad = viven_en_ciudad_edad(&personas, "test", 18);
        assert_eq!(viven_en_ciudad_edad.len(), 2);
        assert_eq!(viven_en_ciudad_edad[0].get_edad(), 20);
        assert_eq!(viven_en_ciudad_edad[1].get_edad(), 22);
    }

    #[test]
    fn test_viven_todos_en_ciudad() {
        let mut personas = vec![Persona::default(); 5];
        assert!(viven_todas_en_ciudad(&personas, "default"));
        personas.push(Persona::new("test", "test", "test", "test", 200.0, 20));
        assert!(!viven_todas_en_ciudad(&personas, "default"));
    }

    #[test]
    fn test_vive_alguna_en_ciudad() {
        let mut personas = vec![Persona::default(); 5];
        assert!(!vive_alguna_en_ciudad(&personas, "test"));
        personas.push(Persona::new("test", "test", "test", "test", 200.0, 20));
        assert!(vive_alguna_en_ciudad(&personas, "test"));
    }

    #[test]
    fn test_existe_persona() {
        let mut personas = vec![Persona::default(); 5];
        let persona = Persona::new("test", "test", "test", "test", 200.0, 20);
        assert!(!existe_persona(&personas, &persona));
        personas.push(persona.clone());
        assert!(existe_persona(&personas, &persona));
    }

    #[test]
    fn test_obtener_edades() {
        let mut personas = vec![Persona::default(); 2];
        personas.push(Persona::new("test", "test", "test", "test", 200.0, 20));

        let edades = obtener_edades(&personas);

        assert_eq!(edades.len(), 3);
        assert_eq!(edades[0], 18);
        assert_eq!(edades[1], 18);
        assert_eq!(edades[2], 20);
    }

    #[test]
    fn test_mayor_menor_salario() {
        let personas = vec![
            Persona::new(
                "test",
                "test",
                "test",
                "test",
                50.0,
                18
            ),
            Persona::new(
                "test",
                "test",
                "test",
                "test",
                75.0,
                18
            ),
            Persona::new(
                "test",
                "test",
                "test",
                "test",
                100.0,
                18
            ),
            Persona::new(
                "test",
                "test",
                "test",
                "test",
                100.0,
                20
            )
        ];
        
        let mayor_menor = mayor_menor_salario(&personas);

        assert!(mayor_menor.is_some());
        assert_eq!(mayor_menor.unwrap().0.get_salario(), 50.0);
        assert_eq!(mayor_menor.unwrap().1.get_salario(), 100.0);
        assert_eq!(mayor_menor.unwrap().1.get_edad(), 20);
    }

    #[test]
    fn test_mayor_menor_salario_none() {
        let personas = vec![];
        
        let mayor_menor = mayor_menor_salario(&personas);

        assert!(mayor_menor.is_none());
    }
}