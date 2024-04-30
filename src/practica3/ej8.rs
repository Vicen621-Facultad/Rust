#[derive(Clone, PartialEq, Eq)]
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros
}

#[derive(Clone, PartialEq, Eq)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

impl Cancion {
    fn new(titulo: &str, artista: &str, genero: Genero) -> Cancion {
        let titulo = titulo.to_owned();
        let artista = artista.to_owned();
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
    fn new(nombre: &str, canciones: Option<Vec<Cancion>>) -> Playlist {
        let nombre = nombre.to_owned();
        let canciones = match canciones {
            Some(canciones) => canciones,
            None => vec![]
        };

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

    fn buscar_cancion_por_nombre(&self, nombre: &str) -> Option<Cancion> {
        let mut opt = None;

        for cancion in &self.canciones {
            if cancion.titulo == nombre {
                opt = Some(cancion.to_owned());
            }
        }

        opt
    }

    fn obtener_canciones_genero(&self, genero: Genero) -> Vec<Cancion> {
        let mut vec = vec![];

        for cancion in &self.canciones {
            if cancion.genero == genero {
                vec.push(cancion.to_owned());
            }
        }

        vec
    }

    fn obtener_canciones_artista(&self, artista: &str) -> Vec<Cancion> {
        let mut vec = vec![];

        for cancion in &self.canciones {
            if cancion.artista == artista {
                vec.push(cancion.to_owned());
            }
        }

        vec
    }

    fn modificar_titulo(&mut self, titulo: &str) {
        self.nombre = titulo.to_owned();
    }

    fn vaciar(&mut self) {
        self.canciones.clear();
    }
}

