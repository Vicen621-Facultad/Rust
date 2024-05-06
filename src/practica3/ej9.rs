use std::collections::VecDeque;
use crate::practica3::ej3::Fecha;

struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    atenciones: Vec<AtencionRealizada>,
    cola: VecDeque<Mascota>
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Animal {
    Perro,
    Gato,
    Caballo,
    Otros
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Mascota {
    nombre: String,
    edad: u32, 
    tipo: Animal,
    dueño: Dueño,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Dueño {
    nombre: String,
    direccion: String,
    telefono: String
}

#[derive(PartialEq, Eq)]
struct AtencionRealizada {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>,
}

impl Veterinaria {
    fn new(nombre: String, direccion: String, id: u32, atenciones: Option<Vec<AtencionRealizada>>, cola: Option<VecDeque<Mascota>>) -> Veterinaria {
        let cola = if let Some(vec_deque) = cola {
            vec_deque
        } else {
            VecDeque::new()    
        };

        let atenciones = if let Some(vec) = atenciones {
            vec
        } else {
            Vec::new()    
        };

        Veterinaria {
            nombre,
            direccion,
            id,
            atenciones,
            cola
        }
    }

    fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola.push_back(mascota);
    }

    fn agregar_mascota_prioridad(&mut self, mascota: Mascota) {
        self.cola.push_front(mascota);
    }

    fn atender_mascota(&mut self) {
        self.cola.pop_front();
    }

    fn eliminar_mascota(&mut self, mascota: &Mascota) {
        let mut index = 0;

        for mascota_cola in &self.cola {
            if mascota_cola == mascota {
                break;
            } else {
                index += 1;
            }
        }

        self.cola.remove(index);
    }

    fn registrar_atencion(&mut self, atencion: AtencionRealizada) {
        self.atenciones.push(atencion);
    }

    fn buscar_atencion_mascota(&self, nombre: String) -> Option<&AtencionRealizada> {
        let mut index = 0;

        for atencion in &self.atenciones {
            if atencion.mascota.nombre == nombre {
                break;
            } else {
                index += 1;
            }
        }

        self.atenciones.get(index)
    }

    fn buscar_atencion_dueño(&self, nombre: String) -> Option<&AtencionRealizada> {
        let mut index = 0;

        for atencion in &self.atenciones {
            if atencion.mascota.dueño.nombre == nombre {
                break;
            } else {
                index += 1;
            }
        }

        self.atenciones.get(index)
    }

    fn buscar_atencion_telefono(&self, telefono: String) -> Option<&AtencionRealizada> {
        let mut index = 0;

        for atencion in &self.atenciones {
            if atencion.mascota.dueño.telefono == telefono {
                break;
            } else {
                index += 1;
            }
        }

        self.atenciones.get(index)
    }

    fn modificar_diagnostico(&mut self, diagnostico: String, atencion: &AtencionRealizada) {
        //TODO: No se como hacer esto
    }

    fn modificar_fecha(&mut self, fecha: Fecha, atencion: &AtencionRealizada) {
        //TODO: No se como hacer esto
    }

    fn eliminar_atencion(&mut self, atencion: &AtencionRealizada) {
        let mut index = 0;

        for atencion_realizada in &self.atenciones {
            if atencion_realizada == atencion {
                break;
            } else {
                index += 1;
            }
        }

        self.atenciones.remove(index);
    }
}