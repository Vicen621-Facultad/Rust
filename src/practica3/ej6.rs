struct Examen {
    materia: String,
    nota: f32,
}

impl Examen {
    fn new(materia: &str, nota: f32) -> Examen {
        let materia = materia.to_owned();
        Examen {
            materia,
            nota
        }
    }
}

struct Estudiante {
    nombre: String,
    id: u32,
    examenes: Vec<Examen>,
}

impl Estudiante {
    fn new(nombre: &str, id: u32, examenes: Vec<Examen>) -> Estudiante {
        let nombre = nombre.to_owned();
        Estudiante {
            nombre,
            id,
            examenes
        }
    }

    fn obtener_promedio(&self) -> f32 {
        let mut total = 0.0;

        for examen in &self.examenes {
            total += examen.nota;
        }

        total / self.examenes.len() as f32
    }

    fn obtener_calificacion_mas_alta(&self) -> f32 {
        let mut max = 0.0;

        for examen in &self.examenes {
            if examen.nota > max {
                max = examen.nota;
            }
        }

        max
    }

    fn obtener_calificacion_mas_baja(&self) -> f32 {
        let mut min = 10.0;

        for examen in &self.examenes {
            if examen.nota < min {
                min = examen.nota;
            }
        }

        min
    }
}

#[test]
fn test_obtener_promedio() {
    let examenes = vec![
        Examen::new("CADP", 8.0), 
        Examen::new("OC", 6.0), 
        Examen::new("Mate1", 10.0),
        Examen::new("Taller", 8.0),
        Examen::new("Mate2", 10.0),
        Examen::new("Arqui", 8.0),
    ];
    let estudiante = Estudiante::new("Vicente Garcia Marti", 23025, examenes);

    assert_eq!(estudiante.obtener_promedio(), 8.333333);
}

#[test]
fn test_obtener_calificacion_mas_alta() {
    let examenes = vec![
        Examen::new("CADP", 8.0), 
        Examen::new("OC", 6.0), 
        Examen::new("Mate1", 10.0),
        Examen::new("Taller", 8.0),
        Examen::new("Mate2", 10.0),
        Examen::new("Arqui", 8.0),
    ];
    let estudiante = Estudiante::new("Vicente Garcia Marti", 23025, examenes);

    assert_eq!(estudiante.obtener_calificacion_mas_alta(), 10.0);
}

#[test]
fn test_obtener_calificacion_mas_baja() {
    let examenes = vec![
        Examen::new("CADP", 8.0), 
        Examen::new("OC", 6.0), 
        Examen::new("Mate1", 10.0),
        Examen::new("Taller", 8.0),
        Examen::new("Mate2", 10.0),
        Examen::new("Arqui", 8.0),
    ];
    let estudiante = Estudiante::new("Vicente Garcia Marti", 23025, examenes);

    assert_eq!(estudiante.obtener_calificacion_mas_baja(), 6.0);
}