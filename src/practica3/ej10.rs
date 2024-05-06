use std::collections::HashMap;
use super::ej3::Fecha;

struct Biblioteca {
    nombre: String,
    direccion: String,
    libros: HashMap<Libro, u32>,
    prestamos: Vec<Prestamo>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum EstadoPrestamo {
    Devuelto,
    EnPrestamo
}

#[derive(PartialEq, Eq, Debug, Clone)]
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

    /// Devuelve true si y solo si la fecha actual es exclusivamente mayor que la fecha de vencimiento 
    /// y el estado del prestamo es EstadoPrestamo::EnPrestamo
    fn vencio(&self, fecha_actual: &Fecha) -> bool {
        self.estado == EstadoPrestamo::EnPrestamo && fecha_actual.es_mayor(&self.fecha_vencimiento)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
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

    fn incrementar_cantidad_copias(&mut self, libro: Libro) {
        let find_libro = self.libros.get_mut(&libro);

        match find_libro {
            Some(cant) => *cant += 1,
            None => {
                self.libros.insert(libro, 1);
            },
        };
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

    fn devolver_libro(&mut self, libro: Libro, cliente: &Cliente, fecha_actual: Fecha) {
        let buscar_prestamo = self.buscar_prestamo(&libro, cliente);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vencio_prestamo() {
        let fecha_vencimiento = Fecha::new(10, 5, 2024);
        let fecha_actual = Fecha::new(11, 5, 2024);
        let prestamo = Prestamo::new(
            Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 300, Genero::Novela),
            Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string()),
            fecha_vencimiento,
            None,
            EstadoPrestamo::EnPrestamo
        );

        assert!(prestamo.vencio(&fecha_actual));
    }

    #[test]
    fn test_no_vencio_prestamo() {
        let fecha_vencimiento = Fecha::new(10, 5, 2024);
        let fecha_actual = Fecha::new(9, 5, 2024);
        let prestamo = Prestamo::new(
            Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 300, Genero::Novela),
            Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string()),
            fecha_vencimiento,
            None,
            EstadoPrestamo::EnPrestamo
        );

        assert!(!prestamo.vencio(&fecha_actual));
    }

    #[test]
    fn test_realizar_prestamo_exitoso() {
        let mut biblioteca = Biblioteca::new("Biblioteca".to_string(), "Calle Principal".to_string(), None, None);
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 300, Genero::Novela);
        let fecha_devolucion = Fecha::new(20, 5, 2024);

        biblioteca.incrementar_cantidad_copias(libro.clone());

        assert!(biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_devolucion));
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro), 0); // Se decrementó la cantidad de copias
        assert_eq!(biblioteca.prestamos.len(), 1); // Se agregó un préstamo
    }

    #[test]
    fn test_contar_prestamos_cliente() {
        let mut biblioteca = Biblioteca::new("Biblioteca".to_string(), "Calle Principal".to_string(), None, None);
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro1 = Libro::new("Libro1".to_string(), "Autor1".to_string(), 100, Genero::Novela);
        let libro2 = Libro::new("Libro2".to_string(), "Autor2".to_string(), 200, Genero::Infantil);
        let fecha_devolucion = Fecha::new(20, 5, 2024);

        biblioteca.incrementar_cantidad_copias(libro1.clone());
        biblioteca.incrementar_cantidad_copias(libro2.clone());

        // Se realiza un préstamo
        biblioteca.realizar_prestamo(libro1.clone(), cliente.clone(), fecha_devolucion.clone());
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro1), 0);
        // Se cuenta el préstamo realizado
        assert_eq!(biblioteca.contar_prestamos_cliente(&cliente), 1);

        // Se realiza otro préstamo
        biblioteca.realizar_prestamo(libro2.clone(), cliente.clone(), fecha_devolucion);
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro2), 0);
        // Se cuenta el segundo préstamo realizado
        assert_eq!(biblioteca.contar_prestamos_cliente(&cliente), 2);
    }

    #[test]
    fn test_obtener_prestamos_vencidos() {
        let mut biblioteca = Biblioteca::new("Biblioteca".to_string(), "Calle Principal".to_string(), None, None);
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 300, Genero::Novela);
        let fecha_vencimiento = Fecha::new(20, 5, 2023);
        let fecha_actual = Fecha::new(21, 5, 2023);
        let prestamo_vencido = Prestamo::new(libro.clone(), cliente.clone(), fecha_vencimiento, None, EstadoPrestamo::EnPrestamo);

        assert_eq!(biblioteca.obtener_prestamos_vencidos(&fecha_actual), vec![] as Vec<&Prestamo>);
        biblioteca.prestamos.push(prestamo_vencido.clone());
        assert_eq!(biblioteca.obtener_prestamos_vencidos(&fecha_actual), vec![&prestamo_vencido]);
    }

    #[test]
    fn test_obtener_prestamos_a_vencer() {
        let mut biblioteca = Biblioteca::new("Biblioteca".to_string(), "Calle Principal".to_string(), None, None);
        let fecha_actual = Fecha::new(19, 5, 2023);
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 300, Genero::Novela);
        let fecha_vencimiento = Fecha::new(20, 5, 2023);
        let prestamo_a_vencer = Prestamo::new(libro.clone(), cliente.clone(), fecha_vencimiento, None, EstadoPrestamo::EnPrestamo);

        assert_eq!(biblioteca.obtener_prestamos_a_vencer(fecha_actual.clone(), 2), vec![] as Vec<&Prestamo>);
        biblioteca.prestamos.push(prestamo_a_vencer.clone());
        assert_eq!(biblioteca.obtener_prestamos_a_vencer(fecha_actual, 2), vec![&prestamo_a_vencer]);
    }

    #[test]
    fn test_buscar_prestamo() {
        let mut biblioteca = Biblioteca::new("Biblioteca".to_string(), "Calle Principal".to_string(), None, None);
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 300, Genero::Novela);
        let fecha_devolucion = Fecha::new(20, 5, 2024);
        biblioteca.incrementar_cantidad_copias(libro.clone());
        biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_devolucion);

        assert!(biblioteca.buscar_prestamo(&libro, &cliente).is_some());
    }

    #[test]
    fn test_devolver_libro() {
        let mut biblioteca = Biblioteca::new("Biblioteca".to_string(), "Calle Principal".to_string(), None, None);
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 300, Genero::Novela);
        let fecha_devolucion = Fecha::new(20, 5, 2024);
        biblioteca.incrementar_cantidad_copias(libro.clone());
        biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_devolucion);

        // Se devuelve el libro
        biblioteca.devolver_libro(libro.clone(), &cliente, Fecha::new(21, 5, 2024));
        // Se incrementa la cantidad de copias
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro), 1);
        // Se marca el préstamo como devuelto
        assert_eq!(biblioteca.prestamos[0].estado, EstadoPrestamo::Devuelto);
    }
}
