#[derive(Debug, Clone, PartialEq, Eq)]
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

impl Cancion {
    fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion {
            titulo,
            artista,
            genero
        }
    }
}

struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>
}

impl Playlist {
    fn new(nombre: String, canciones: Vec<Cancion>) -> Playlist {
        Playlist {
            nombre,
            canciones
        }
    }

    fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
    }


    fn eliminar_cancion(&mut self, cancion: &Cancion) -> bool {
        let opt = self.obtener_pos_cancion(cancion);

        if let Some(index) = opt {
            self.canciones.remove(index);
            true
        } else {
            false
        }
    }

    fn obtener_pos_cancion(&self, cancion: &Cancion) -> Option<usize> {
        let mut index = None;

        for i in 0..self.canciones.len() {
            match self.canciones.get(i) {
                Some(cancion_comp) => if cancion_comp == cancion { index = Some(i); }
                _ => {}
            }
        }

        index
    }

    fn mover_cancion(&mut self, cancion: &Cancion, pos: usize) {
        let opt = self.obtener_pos_cancion(cancion);

        if let Some(index) = opt {
            let cancion = self.canciones.remove(index);
            self.canciones.insert(pos, cancion)
        }
    }

    fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<Cancion> {
        let mut opt = None;

        for cancion in &self.canciones {
            if cancion.titulo == nombre {
                opt = Some(cancion.clone());
            }
        }

        opt
    }

    fn obtener_canciones_genero(&self, genero: &Genero) -> Vec<Cancion> {
        let mut vec = vec![];

        for cancion in &self.canciones {
            if &cancion.genero == genero {
                vec.push(cancion.clone());
            }
        }

        vec
    }

    fn obtener_canciones_artista(&self, artista: String) -> Vec<Cancion> {
        let mut vec = vec![];

        for cancion in &self.canciones {
            if cancion.artista == artista {
                vec.push(cancion.clone());
            }
        }

        vec
    }

    fn modificar_titulo(&mut self, titulo: String) {
        //REVIEW: Preguntar por el to_owned o si es mejor String
        self.nombre = titulo;
    }

    fn vaciar(&mut self) {
        self.canciones.clear();
    }
}

#[test]
fn test_agregar_cancion() {
    let mut playlist = Playlist::new("test".to_owned(), vec![Cancion::new("Cirugia".to_owned(), "Dillom".to_owned(), Genero::Otros)]);
    playlist.agregar_cancion(Cancion::new("Ultimamente".to_owned(), "Dillom".to_owned(), Genero::Otros));
    assert_eq!(playlist.canciones.len(), 2);

    let cancion_playlist = playlist.canciones.pop();
    assert_eq!(cancion_playlist, Some(Cancion::new("Ultimamente".to_owned(), "Dillom".to_owned(), Genero::Otros)));
}

#[test]
fn test_eliminar_cancion() {
    let mut playlist = Playlist::new("test".to_owned(), vec![Cancion::new("Cirugia".to_owned(), "Dillom".to_owned(), Genero::Otros)]);
    let cancion = Cancion::new("Ultimamente".to_owned(), "Dillom".to_owned(), Genero::Otros);
    playlist.canciones.push(Cancion::new("Ultimamente".to_owned(), "Dillom".to_owned(), Genero::Otros));

    assert!(playlist.eliminar_cancion(&cancion));
    assert!(!playlist.eliminar_cancion(&cancion));
    assert_eq!(playlist.canciones.len(), 1);
}

#[test]
fn test_obtener_pos_cancion() {
    let mut playlist = Playlist::new("test".to_owned(), vec![Cancion::new("Cirugia".to_owned(), "Dillom".to_owned(), Genero::Otros)]);
    playlist.agregar_cancion(Cancion::new("La novia de mi amigo".to_owned(), "Dillom".to_owned(), Genero::Otros));
    playlist.agregar_cancion(Cancion::new("Ultimamente".to_owned(), "Dillom".to_owned(), Genero::Otros));
    playlist.agregar_cancion(Cancion::new("Mi peor enemigo".to_owned(), "Dillom".to_owned(), Genero::Otros));
    playlist.agregar_cancion(Cancion::new("La carie".to_owned(), "Dillom".to_owned(), Genero::Otros));

    assert_eq!(playlist.obtener_pos_cancion(&Cancion::new("Cirugia".to_owned(), "Dillom".to_owned(), Genero::Otros)), Some(0));
    assert_eq!(playlist.obtener_pos_cancion(&Cancion::new("La novia de mi amigo".to_owned(), "Dillom".to_owned(), Genero::Otros)), Some(1));
    assert_eq!(playlist.obtener_pos_cancion(&Cancion::new("Ultimamente".to_owned(), "Dillom".to_owned(), Genero::Otros)), Some(2));
    assert_eq!(playlist.obtener_pos_cancion(&Cancion::new("Mi peor enemigo".to_owned(), "Dillom".to_owned(), Genero::Otros)), Some(3));
    assert_eq!(playlist.obtener_pos_cancion(&Cancion::new("La carie".to_owned(), "Dillom".to_owned(), Genero::Otros)), Some(4));
    assert!(playlist.obtener_pos_cancion(&Cancion::new("Muñecas".to_owned(), "Dillom".to_owned(), Genero::Otros)).is_none());
}

#[test]
fn test_buscar_cancion_por_nombre() {
    let mut playlist = Playlist::new("test".to_owned(), vec![Cancion::new("Cirugia".to_owned(), "Dillom".to_owned(), Genero::Otros)]);
    playlist.agregar_cancion(Cancion::new("La novia de mi amigo".to_owned(), "Dillom".to_owned(), Genero::Otros));
    playlist.agregar_cancion(Cancion::new("Ultimamente".to_owned(), "Dillom".to_owned(), Genero::Otros));
    playlist.agregar_cancion(Cancion::new("Mi peor enemigo".to_owned(), "Dillom".to_owned(), Genero::Otros));
    playlist.agregar_cancion(Cancion::new("La carie".to_owned(), "Dillom".to_owned(), Genero::Otros));

    let cancion = playlist.buscar_cancion_por_nombre("Mi peor enemigo".to_owned());
    let none = playlist.buscar_cancion_por_nombre("Muñecas".to_owned());

    assert_eq!(cancion, Some(Cancion::new("Mi peor enemigo".to_owned(), "Dillom".to_owned(), Genero::Otros)));
    assert!(none.is_none());
}

#[test]
fn test_obtener_canciones_genero() {
    let mut playlist = Playlist::new("test".to_owned(), vec![Cancion::new("Cirugia".to_owned(), "Dillom".to_owned(), Genero::Otros)]);
    playlist.agregar_cancion(Cancion::new("Ya no sos igual".to_owned(), "2 minutos".to_owned(), Genero::Rock));
    playlist.agregar_cancion(Cancion::new("Brain Damage".to_owned(), "Pink Floyd".to_owned(), Genero::Rock));
    playlist.agregar_cancion(Cancion::new("Hola".to_owned(), "Miranda!".to_owned(), Genero::Pop));

    let rock = playlist.obtener_canciones_genero(&Genero::Rock);
    
    assert_eq!(rock.len(), 2);
    assert!(rock.contains(&Cancion::new("Ya no sos igual".to_owned(), "2 minutos".to_owned(), Genero::Rock)));
    assert!(rock.contains(&Cancion::new("Brain Damage".to_owned(), "Pink Floyd".to_owned(), Genero::Rock)));
}

#[test]
fn test_obtener_canciones_artista() {
    let mut playlist = Playlist::new("test".to_owned(), vec![Cancion::new("Cirugia".to_owned(), "Dillom".to_owned(), Genero::Otros)]);
    playlist.agregar_cancion(Cancion::new("Ya no sos igual".to_owned(), "2 minutos".to_owned(), Genero::Rock));
    playlist.agregar_cancion(Cancion::new("Ultimamente".to_owned(), "Dillom".to_owned(), Genero::Otros));
    playlist.agregar_cancion(Cancion::new("Hola".to_owned(), "Miranda!".to_owned(), Genero::Pop));

    let rock = playlist.obtener_canciones_artista("Dillom".to_owned());
    
    assert_eq!(rock.len(), 2);
    assert!(rock.contains(&Cancion::new("Cirugia".to_owned(), "Dillom".to_owned(), Genero::Otros)));
    assert!(rock.contains(&Cancion::new("Ultimamente".to_owned(), "Dillom".to_owned(), Genero::Otros)));
}

#[test]
fn test_modificar_titulo() {
    let mut playlist = Playlist::new("test".to_owned(), vec![Cancion::new("Cirugia".to_owned(), "Dillom".to_owned(), Genero::Otros)]);
    assert_eq!(playlist.nombre, "test");
    playlist.modificar_titulo("test2".to_owned());
    assert_eq!(playlist.nombre, "test2");
}

#[test]
fn test_vaciar() {
    let mut playlist = Playlist::new("test".to_owned(), vec![Cancion::new("Cirugia".to_owned(), "Dillom".to_owned(), Genero::Otros)]);
    playlist.agregar_cancion(Cancion::new("La novia de mi amigo".to_owned(), "Dillom".to_owned(), Genero::Otros));
    playlist.agregar_cancion(Cancion::new("Ultimamente".to_owned(), "Dillom".to_owned(), Genero::Otros));
    playlist.agregar_cancion(Cancion::new("Mi peor enemigo".to_owned(), "Dillom".to_owned(), Genero::Otros));
    playlist.agregar_cancion(Cancion::new("La carie".to_owned(), "Dillom".to_owned(), Genero::Otros));

    assert_eq!(playlist.canciones.len(), 5);
    playlist.vaciar();
    assert_eq!(playlist.canciones.len(), 0);
    assert!(playlist.canciones.is_empty());
}