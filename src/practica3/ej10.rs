use std::collections::HashMap;

use super::ej3::Fecha;

struct Biblioteca {
    nombre: String,
    direccion: String,
    libros: HashMap<Libro, u32>,
    prestamos: Vec<Prestamo>,
}

#[derive(PartialEq, Eq)]
enum EstadoPrestamo {
    Devuelto,
    EnPrestamo
}

struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: EstadoPrestamo
}

impl Prestamo {
    fn new(libro: Libro, cliente: Cliente, fecha_vencimiento: Fecha, fecha_devolucion: Option<Fecha>, estado: EstadoPrestamo) -> Prestamo {
        Prestamo {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion,
            estado
        }
    }

    fn vencio(&self, fecha_actual: &Fecha) -> bool {
        self.estado == EstadoPrestamo::EnPrestamo && fecha_actual.es_mayor(&self.fecha_vencimiento)
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Cliente {
    nombre: String,
    telefono: String,
    email: String
}

impl Cliente {
    fn new(nombre: String, telefono: String, email: String) -> Cliente {
        Cliente {
            nombre,
            telefono,
            email
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Libro {
    titulo: String,
    autor: String,
    numero_paginas: u32,
    genero: Genero
}

impl Libro {
    fn new(titulo: String, autor: String, numero_paginas: u32, genero: Genero) -> Libro {
        Libro {
            titulo,
            autor,
            numero_paginas,
            genero
        }
    }
}

impl Biblioteca {
    fn new(nombre: String, direccion: String, libros: Option<HashMap<Libro, u32>>, prestamos: Option<Vec<Prestamo>>) -> Biblioteca {
        let libros = match libros {
            Some(hash_map) => hash_map,
            None => HashMap::new()
        };

        let prestamos = match prestamos {
            Some(vec) => vec,
            None => Vec::new()
        };

        Biblioteca {
            nombre,
            direccion,
            libros,
            prestamos
        }
    }

    fn obtener_cantidad_copias(&self, libro: &Libro) -> u32 {
        let get_cantidad = self.libros.get(libro);

        match get_cantidad {
            Some(cant) => *cant,
            None => 0
        }
    }

    fn incrementar_cantidad_copias(&mut self, libro: &Libro) {
        let find_libro = self.libros.get_mut(libro);

        match find_libro {
            Some(cant) => *cant += 1,
            None => {}
        }
    }

    fn decrementar_cantidad_copias(&mut self, libro: &Libro) {
        let find_libro = self.libros.get_mut(libro);

        match find_libro {
            Some(cant) => *cant -= 1,
            None => {}
        }
    }

    fn contar_prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        let mut count = 0;

        for prestamo in &self.prestamos {
            if (&prestamo.cliente == cliente) && (prestamo.estado == EstadoPrestamo::EnPrestamo) {
                count += 1;
            }
        }

        count
    }

    fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente, fecha_devolucion: Fecha) -> bool {
        if self.contar_prestamos_cliente(&cliente) > 5 {
            false
        } else if self.obtener_cantidad_copias(&libro) < 1 {
            false
        } else {
            self.decrementar_cantidad_copias(&libro);
            let prestamo = Prestamo::new(
                libro,
                cliente,
                fecha_devolucion,
                None,
                EstadoPrestamo::EnPrestamo
            );
            self.prestamos.push(prestamo);
            true
        }
    }

    fn obtener_prestamos_a_vencer(&self, mut fecha_actual: Fecha, dias: u32) -> Vec<&Prestamo> {
        let mut vec = vec![];
        fecha_actual.sumar_dias(dias);

        for prestamo in &self.prestamos {
            if prestamo.vencio(&fecha_actual) {
                vec.push(prestamo);
            }
        }

        vec
    }

    fn obtener_prestamos_vencidos(&self, fecha_actual: &Fecha) -> Vec<&Prestamo> {
        let mut vec = vec![];

        for prestamo in &self.prestamos {
            if prestamo.vencio(&fecha_actual) {
                vec.push(prestamo);
            }
        }

        vec
    }

    fn buscar_prestamo(&mut self, libro: &Libro, cliente: &Cliente) -> Option<&mut Prestamo> {
        self.prestamos.iter_mut().find(|prestamo| &prestamo.libro == libro && &prestamo.cliente == cliente)
    }

    fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha_actual: Fecha) {
        let buscar_prestamo = self.buscar_prestamo(libro, cliente);

        match buscar_prestamo {
            Some(prestamo) => {
                prestamo.estado = EstadoPrestamo::Devuelto;
                prestamo.fecha_devolucion = Some(fecha_actual);
                self.incrementar_cantidad_copias(libro);
            },
            _ => {}
        }
    }
}

//TODO: Hacer Tests
