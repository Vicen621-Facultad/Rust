use std::collections::HashMap;
use super::ej3::Fecha;

struct Biblioteca {
    nombre: String,
    direccion: String,
    libros: HashMap<u32, u32>,
    prestamos: Vec<Prestamo>,
}

#[derive(Debug, Clone)]
enum EstadoPrestamo {
    Devuelto,
    EnPrestamo
}

impl EstadoPrestamo {
    fn to_string(&self) -> String {
        match self {
            EstadoPrestamo::Devuelto => String::from("devuelto"),
            EstadoPrestamo::EnPrestamo => String::from("en_prestamo")
        }
    }

    fn equals(&self, other: &EstadoPrestamo) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Debug, Clone)]
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

    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("Libro: {}\n", self.libro.isbn));
        result.push_str(&format!("Cliente: {}\n", self.cliente.to_string()));
        result.push_str(&format!("Fecha de vencimiento: {}\n", self.fecha_vencimiento.to_string()));
        result.push_str(&format!("Fecha de devolución: {}\n", 
            match &self.fecha_devolucion {
                Some(fecha) => fecha.to_string(),
                None => String::from("No devuelto aún"),
            }
        ));
        result.push_str(&format!("Estado: {}\n", self.estado.to_string()));
        result
    }

    fn equals(&self, other: &Prestamo) -> bool {
        self.to_string() == other.to_string()
    }

    /// Devuelve true si y solo si la fecha actual es exclusivamente mayor que la fecha de vencimiento 
    /// y el estado del prestamo es EstadoPrestamo::EnPrestamo
    fn vencio(&self, fecha_actual: &Fecha) -> bool {
        self.estado.equals(&EstadoPrestamo::EnPrestamo) && fecha_actual.es_mayor(&self.fecha_vencimiento)
    }
}

#[derive(Clone, Debug)]
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

    fn to_string(&self) -> String {
        format!("Nombre: {}\nTeléfono: {}\nEmail: {}", self.nombre, self.telefono, self.email)
    }

    fn equals(&self, other: &Cliente) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Debug, Clone)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros
}

impl Genero {
    fn to_string(&self) -> String {
        match self {
            Genero::Novela => String::from("novela"),
            Genero::Infantil => String::from("infantil"),
            Genero::Tecnico => String::from("tecnico"),
            Genero::Otros => String::from("otros"),
        }
    }

    fn equals(&self, other: &Genero) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Debug, Clone)]
struct Libro {
    titulo: String,
    autor: String,
    isbn: u32,
    numero_paginas: u32,
    genero: Genero
}

impl Libro {
    fn new(titulo: String, autor: String, isbn: u32, numero_paginas: u32, genero: Genero) -> Libro {
        Libro {
            titulo,
            autor,
            isbn,
            numero_paginas,
            genero
        }
    }

    fn equals(&self, other: &Libro) -> bool {
        self.isbn == other.isbn
    }
}

impl Biblioteca {
    fn new(nombre: String, direccion: String, libros: Option<HashMap<u32, u32>>, prestamos: Option<Vec<Prestamo>>) -> Biblioteca {
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
        let get_cantidad = self.libros.get(&libro.isbn);

        match get_cantidad {
            Some(cant) => *cant,
            None => 0
        }
    }

    fn incrementar_cantidad_copias(&mut self, libro: Libro) {
        let find_libro = self.libros.get_mut(&libro.isbn);

        match find_libro {
            Some(cant) => *cant += 1,
            None => {
                self.libros.insert(libro.isbn, 1);
            },
        };
    }

    fn decrementar_cantidad_copias(&mut self, libro: &Libro) {
        let find_libro = self.libros.get_mut(&libro.isbn);

        match find_libro {
            Some(cant) => *cant -= 1,
            None => {}
        }
    }

    fn contar_prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        let mut count = 0;

        for prestamo in &self.prestamos {
            if (prestamo.cliente.equals(cliente)) && (prestamo.estado.equals(&EstadoPrestamo::EnPrestamo)) {
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
        let mut ret = None;

        for prestamo in &mut self.prestamos {
            if prestamo.cliente.equals(cliente) && prestamo.libro.equals(libro) {
                ret = Some(prestamo);
                break;
            }
        }

        ret
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
            Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela),
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
            Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela),
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
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela);
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
        let libro1 = Libro::new("Libro1".to_string(), "Autor1".to_string(), 1, 100, Genero::Novela);
        let libro2 = Libro::new("Libro2".to_string(), "Autor2".to_string(), 2, 200, Genero::Infantil);
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
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela);
        let fecha_vencimiento = Fecha::new(20, 5, 2023);
        let fecha_actual = Fecha::new(21, 5, 2023);
        let prestamo_vencido = Prestamo::new(libro.clone(), cliente.clone(), fecha_vencimiento, None, EstadoPrestamo::EnPrestamo);

        assert!(biblioteca.obtener_prestamos_vencidos(&fecha_actual).is_empty());
        biblioteca.prestamos.push(prestamo_vencido.clone());
        let prestamos_vencidos = biblioteca.obtener_prestamos_vencidos(&fecha_actual);
        assert_eq!(prestamos_vencidos.len(), 1);
        assert!(prestamos_vencidos[0].equals(&prestamo_vencido));
    }

    #[test]
    fn test_obtener_prestamos_a_vencer() {
        let mut biblioteca = Biblioteca::new("Biblioteca".to_string(), "Calle Principal".to_string(), None, None);
        let fecha_actual = Fecha::new(19, 5, 2023);
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela);
        let fecha_vencimiento = Fecha::new(20, 5, 2023);
        let prestamo_a_vencer = Prestamo::new(libro.clone(), cliente.clone(), fecha_vencimiento, None, EstadoPrestamo::EnPrestamo);

        assert!(biblioteca.obtener_prestamos_a_vencer(fecha_actual.clone(), 2).is_empty());
        biblioteca.prestamos.push(prestamo_a_vencer.clone());
        let prestamos_vencidos = biblioteca.obtener_prestamos_a_vencer(fecha_actual.clone(), 2);
        assert_eq!(prestamos_vencidos.len(), 1);
        assert!(prestamos_vencidos[0].equals(&prestamo_a_vencer));
    }

    #[test]
    fn test_buscar_prestamo() {
        let mut biblioteca = Biblioteca::new("Biblioteca".to_string(), "Calle Principal".to_string(), None, None);
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela);
        let fecha_devolucion = Fecha::new(20, 5, 2024);
        biblioteca.incrementar_cantidad_copias(libro.clone());
        biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_devolucion);

        assert!(biblioteca.buscar_prestamo(&libro, &cliente).is_some());
    }

    #[test]
    fn test_devolver_libro() {
        let mut biblioteca = Biblioteca::new("Biblioteca".to_string(), "Calle Principal".to_string(), None, None);
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela);
        let fecha_devolucion = Fecha::new(20, 5, 2024);
        biblioteca.incrementar_cantidad_copias(libro.clone());
        biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_devolucion);

        // Se devuelve el libro
        biblioteca.devolver_libro(libro.clone(), &cliente, Fecha::new(21, 5, 2024));
        // Se incrementa la cantidad de copias
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro), 1);
        // Se marca el préstamo como devuelto
        assert!(biblioteca.prestamos[0].estado.equals(&EstadoPrestamo::Devuelto));
    }
}
