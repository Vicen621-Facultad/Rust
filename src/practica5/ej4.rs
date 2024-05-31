use std::{collections::HashMap, io::{Read, Write}};

use serde::{Deserialize, Serialize};

use crate::practica3::ej3::Fecha;

struct Biblioteca {
    nombre: String,
    direccion: String,
    // key: libro.isbn, value: cant de ese libro
    libros: HashMap<u32, u32>,
    prestamos: Vec<Prestamo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn new(nombre: String, direccion: String) -> Biblioteca {
        let libros = match std::fs::File::open("test/".to_owned() + nombre.as_str() + "_libros.json") {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                let atenciones: HashMap<u32, u32> = serde_json::from_str(&buf).unwrap();
                atenciones
            },
            Err(_) => HashMap::new()
        };

        let prestamos = match std::fs::File::open("test/".to_owned() + nombre.as_str() + "_prestamos.json") {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                let atenciones: Vec<Prestamo> = serde_json::from_str(&buf).unwrap();
                atenciones
            },
            Err(_) => Vec::new()
        };

        Biblioteca {
            nombre,
            direccion,
            libros,
            prestamos
        }
    }

    fn escribir_archivo_libros(&self) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create("test/".to_owned() + self.nombre.as_str() + "_libros.json")?;
        let serialized = serde_json::to_string(&self.libros)?;
        file.write_all(&serialized.as_bytes())?;
        Ok(())
    }

    fn escribir_archivo_prestamos(&self) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create("test/".to_owned() + self.nombre.as_str() + "_prestamos.json")?;
        let serialized = serde_json::to_string(&self.prestamos)?;
        file.write_all(&serialized.as_bytes())?;
        Ok(())
    }

    fn obtener_cantidad_copias(&self, libro: &Libro) -> u32 {
        let get_cantidad = self.libros.get(&libro.isbn);

        match get_cantidad {
            Some(cant) => *cant,
            None => 0
        }
    }

    fn incrementar_cantidad_copias(&mut self, libro: Libro) -> Result<(), std::io::Error> {
        let find_libro = self.libros.get_mut(&libro.isbn);

        match find_libro {
            Some(cant) => *cant += 1,
            None => {
                self.libros.insert(libro.isbn, 1);
            },
        };

        self.escribir_archivo_libros()
    }

    fn decrementar_cantidad_copias(&mut self, libro: &Libro) -> Result<(), std::io::Error>{
        let find_libro = self.libros.get_mut(&libro.isbn);

        match find_libro {
            Some(cant) => {
                *cant -= 1;
                self.escribir_archivo_libros()
            },
            None => Err(std::io::Error::new(std::io::ErrorKind::Other, "No se encontró el libro"))
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

    fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente, fecha_devolucion: Fecha) -> Result<(), std::io::Error>{
        if self.contar_prestamos_cliente(&cliente) > 5 {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "El cliente ya tiene 5 libros prestados"))
        } else if self.obtener_cantidad_copias(&libro) < 1 {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "No hay copias disponibles del libro"))
        } else {
            self.decrementar_cantidad_copias(&libro)?;
            let prestamo = Prestamo::new(
                libro,
                cliente,
                fecha_devolucion,
                None,
                EstadoPrestamo::EnPrestamo
            );
            self.prestamos.push(prestamo);
            self.escribir_archivo_prestamos()
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

    fn devolver_libro(&mut self, libro: Libro, cliente: &Cliente, fecha_actual: Fecha) -> Result<(), std::io::Error>{
        let buscar_prestamo = self.buscar_prestamo(&libro, cliente);

        match buscar_prestamo {
            Some(prestamo) => {
                prestamo.estado = EstadoPrestamo::Devuelto;
                prestamo.fecha_devolucion = Some(fecha_actual);
                self.incrementar_cantidad_copias(libro)?;
                self.escribir_archivo_prestamos()
            },
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "No se encontró el préstamo"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genero_to_string() {
        assert_eq!(Genero::Novela.to_string(), "novela");
        assert_eq!(Genero::Infantil.to_string(), "infantil");
        assert_eq!(Genero::Tecnico.to_string(), "tecnico");
        assert_eq!(Genero::Otros.to_string(), "otros");
    }

    fn test_genero_equals() {
        assert!(Genero::Novela.equals(&Genero::Novela));
        assert!(!Genero::Novela.equals(&Genero::Infantil));
    }

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
    fn test_new_biblioteca_con_datos() {
        let mut biblio1 = Biblioteca::new("test_new_biblioteca_con_datos".to_string(), "Calle Principal".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela);
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        assert_eq!(biblio1.libros.len(), 0);
        assert_eq!(biblio1.prestamos.len(), 0);

        assert!(biblio1.incrementar_cantidad_copias(libro.clone()).is_ok());
        assert!(biblio1.realizar_prestamo(libro.clone(), cliente.clone(), Fecha::now()).is_ok());

        let mut biblio2 = Biblioteca::new("test_new_biblioteca_con_datos".to_string(), "Calle Principal".to_string());
        assert_eq!(biblio2.libros.len(), 1);
        assert_eq!(biblio2.obtener_cantidad_copias(&libro), 0);
        assert_eq!(biblio2.prestamos.len(), 1);
        assert!(biblio2.buscar_prestamo(&libro, &cliente).is_some());
    }

    #[test]
    fn test_realizar_prestamo_exitoso() {
        let mut biblioteca = Biblioteca::new("test_realizar_prestamo_exitoso".to_string(), "Calle Principal".to_string());
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela);
        let fecha_devolucion = Fecha::new(20, 5, 2024);

        assert!(biblioteca.incrementar_cantidad_copias(libro.clone()).is_ok());
        assert!(biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_devolucion).is_ok());
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro), 0); // Se decrementó la cantidad de copias
        assert_eq!(biblioteca.prestamos.len(), 1); // Se agregó un préstamo
    }

    #[test]
    fn test_contar_prestamos_cliente() {
        let mut biblioteca = Biblioteca::new("test_contar_prestamos_cliente".to_string(), "Calle Principal".to_string());
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro1 = Libro::new("Libro1".to_string(), "Autor1".to_string(), 1, 100, Genero::Novela);
        let libro2 = Libro::new("Libro2".to_string(), "Autor2".to_string(), 2, 200, Genero::Infantil);
        let fecha_devolucion = Fecha::new(20, 5, 2024);

        assert!(biblioteca.incrementar_cantidad_copias(libro1.clone()).is_ok());
        assert!(biblioteca.incrementar_cantidad_copias(libro2.clone()).is_ok());

        // Se realiza un préstamo
        assert!(biblioteca.realizar_prestamo(libro1.clone(), cliente.clone(), fecha_devolucion.clone()).is_ok());
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro1), 0);
        // Se cuenta el préstamo realizado
        assert_eq!(biblioteca.contar_prestamos_cliente(&cliente), 1);

        // Se realiza otro préstamo
        assert!(biblioteca.realizar_prestamo(libro2.clone(), cliente.clone(), fecha_devolucion).is_ok());
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro2), 0);
        // Se cuenta el segundo préstamo realizado
        assert_eq!(biblioteca.contar_prestamos_cliente(&cliente), 2);
    }

    #[test]
    fn test_obtener_prestamos_vencidos() {
        let mut biblioteca = Biblioteca::new("test_obtener_prestamos_vencidos".to_string(), "Calle Principal".to_string());
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
        let mut biblioteca = Biblioteca::new("test_obtener_prestamos_a_vencer".to_string(), "Calle Principal".to_string());
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
        let mut biblioteca = Biblioteca::new("test_buscar_prestamo".to_string(), "Calle Principal".to_string());
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela);
        let fecha_devolucion = Fecha::new(20, 5, 2024);
        assert!(biblioteca.incrementar_cantidad_copias(libro.clone()).is_ok());
        assert!(biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_devolucion).is_ok());

        assert!(biblioteca.buscar_prestamo(&libro, &cliente).is_some());
    }

    #[test]
    fn test_devolver_libro() {
        let mut biblioteca = Biblioteca::new("test_devolver_libro".to_string(), "Calle Principal".to_string());
        let cliente = Cliente::new("John Doe".to_string(), "123456789".to_string(), "john@example.com".to_string());
        let libro = Libro::new("Harry Potter".to_string(), "J.K. Rowling".to_string(), 1, 300, Genero::Novela);
        let fecha_devolucion = Fecha::new(20, 5, 2024);
        assert!(biblioteca.incrementar_cantidad_copias(libro.clone()).is_ok());
        assert!(biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_devolucion).is_ok());

        // Se devuelve el libro
        assert!(biblioteca.devolver_libro(libro.clone(), &cliente, Fecha::new(21, 5, 2024)).is_ok());
        // Se incrementa la cantidad de copias
        assert_eq!(biblioteca.obtener_cantidad_copias(&libro), 1);
        // Se marca el préstamo como devuelto
        assert!(biblioteca.prestamos[0].estado.equals(&EstadoPrestamo::Devuelto));
    }
}
