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

struct AtencionRealizada {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>,
}

impl Veterinaria {
    fn new(nombre: String, direccion: String, id: u32, cola: Option<VecDeque<Mascota>>) -> Veterinaria {
        let cola = if let Some(heap) = cola {
            heap
        } else {
            VecDeque::new()    
        };

        Veterinaria {
            nombre,
            direccion,
            id,
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

    fn eliminar_mascota(&mut self, mascota: Mascota) {

    }

    fn registrar_atencion(&mut self, )
}