#[derive(Clone)]
struct Examen {
    materia: String,
    nota: f32,
}

impl Examen {
    fn new(materia: String, nota: f32) -> Examen {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_estudiante() {
        let examenes = vec![
            Examen::new("Matemáticas".to_string(), 8.5),
            Examen::new("Historia".to_string(), 7.0),
            Examen::new("Ciencias".to_string(), 9.2),
        ];
        let estudiante = Estudiante::new("Juan", 1, examenes.clone());
        
        assert_eq!(estudiante.nombre, "Juan");
        assert_eq!(estudiante.id, 1);

        for i in 0..examenes.len() {
            let examen = &examenes[i];
            let examen_estudiante = &estudiante.examenes[i];
            assert_eq!(examen.materia, examen_estudiante.materia);
            assert_eq!(examen.nota, examen_estudiante.nota);
        }
    }

    #[test]
    fn test_obtener_promedio() {
        let examenes = vec![
            Examen::new("Matemáticas".to_string(), 8.5),
            Examen::new("Historia".to_string(), 7.0),
            Examen::new("Ciencias".to_string(), 9.2),
        ];
        let estudiante = Estudiante::new("Juan", 1, examenes);
        
        assert_eq!(estudiante.obtener_promedio().round(), 8.0);
    }

    #[test]
    fn test_obtener_calificacion_mas_alta() {
        let examenes = vec![
            Examen::new("Matemáticas".to_string(), 8.5),
            Examen::new("Historia".to_string(), 7.0),
            Examen::new("Ciencias".to_string(), 9.2),
        ];
        let estudiante = Estudiante::new("Juan", 1, examenes);
        
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), 9.2);
    }

    #[test]
    fn test_obtener_calificacion_mas_baja() {
        let examenes = vec![
            Examen::new("Matemáticas".to_string(), 8.5),
            Examen::new("Historia".to_string(), 7.0),
            Examen::new("Ciencias".to_string(), 9.2),
        ];
        let estudiante = Estudiante::new("Juan", 1, examenes);
        
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), 7.0);
    }
}
