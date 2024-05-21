// Nombre: Vicente García Martí | DNI: 46.645.435 | Discord: Vicen621
#[derive(Debug, Clone)]
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros
}

#[derive(Debug, Clone)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

/// Si la cancion se encuentra en la primera posicion, el campo posicion sera igual a 0
struct CancionReporte {
    titulo: String,
    artista: String,
    posicion: u32,
}

struct Reporte {
    total_canciones: u32,
    canciones: Vec<CancionReporte>,
}

struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>
}

impl Genero {
    fn to_string(&self) -> String {
        match self {
            Genero::Rock => String::from("Rock"),
            Genero::Pop => String::from("Pop"),
            Genero::Rap => String::from("Rap"),
            Genero::Jazz => String::from("Jazz"),
            Genero::Otros => String::from("Otros"),
        }
    }

    fn equals(&self, other: &Genero) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Cancion {
    fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion {
            titulo,
            artista,
            genero
        }
    }

    fn to_string(&self) -> String {
        format!(
            "Titulo: {}\nArtista: {}\nGenero: {}",
            self.titulo, self.artista, self.genero.to_string()
        )
    }

    fn equals(&self, other: &Cancion) -> bool {
        self.to_string() == other.to_string()
    }
}

impl CancionReporte {
    fn new(titulo: String, artista: String, posicion: u32) -> CancionReporte {
        CancionReporte {
            titulo,
            artista,
            posicion
        }
    }

    fn to_string(&self) -> String {
        format!(
            "Titulo: {}\nArtista: {}\nPosicion: {}",
            self.titulo, self.artista, self.posicion
        )
    }

    fn equals(&self, other: &CancionReporte) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Reporte {
    fn new(total_canciones: u32, canciones: Vec<CancionReporte>) -> Reporte {
        Reporte {
            total_canciones,
            canciones
        }
    }
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
        let mut position = None;

        for i in 0..self.canciones.len() {
            if let Some(c) = self.canciones.get(i) {
                if c.equals(cancion) {
                    position = Some(i);
                    break;
                }
            }
        }


        position
    }

    fn mover_cancion(&mut self, cancion: &Cancion, pos: usize) {
        let opt = self.obtener_pos_cancion(cancion);

        if let Some(index) = opt {
            let cancion = self.canciones.remove(index);
            self.canciones.insert(pos, cancion)
        }
    }

    fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<&Cancion> {
        let mut ret = None;

        for cancion in &self.canciones {
            if cancion.titulo == nombre {
                ret = Some(cancion);
                break;
            }
        }

        ret
    }

    fn obtener_canciones_genero(&self, genero: &Genero) -> Vec<&Cancion> {
        let mut vec = vec![];

        for cancion in &self.canciones {
            if cancion.genero.equals(genero) {
                vec.push(cancion);
            }
        }

        vec
    }

    fn obtener_canciones_artista(&self, artista: String) -> Vec<&Cancion> {
        let mut vec = vec![];

        for cancion in &self.canciones {
            if cancion.artista == artista {
                vec.push(cancion);
            }
        }

        vec
    }

    fn modificar_titulo(&mut self, titulo: String) {
        self.nombre = titulo;
    }

    fn vaciar(&mut self) {
        self.canciones.clear();
    }

    fn generar_reporte_genero(&self, genero: Genero) -> Reporte {
        let mut vec = vec![];
        let mut pos = 0;

        for cancion in &self.canciones {
            if cancion.genero.equals(&genero) {
                vec.push(CancionReporte::new(
                    cancion.titulo.clone(), 
                    cancion.artista.clone(), 
                    pos
                ));
            }

            pos += 1;
        }

        Reporte::new(vec.len() as u32, vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genero_to_string() {
        assert_eq!(Genero::Rock.to_string(), "Rock");
        assert_eq!(Genero::Pop.to_string(), "Pop");
        assert_eq!(Genero::Jazz.to_string(), "Jazz");
    }

    #[test]
    fn test_genero_equals() {
        assert!(Genero::Rock.equals(&Genero::Rock));
        assert!(!Genero::Rock.equals(&Genero::Pop));
    }

    #[test]
    fn test_new_cancion() {
        let cancion = Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);

        assert_eq!(cancion.titulo, "Bohemian Rhapsody");
        assert_eq!(cancion.artista, "Queen");
        assert!(cancion.genero.equals(&Genero::Rock));
    }

    #[test]
    fn test_cancion_to_string() {
        let cancion = Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        assert_eq!(cancion.to_string(), "Titulo: Bohemian Rhapsody\nArtista: Queen\nGenero: Rock");
    }

    #[test]
    fn test_cancion_equals() {
        let cancion = Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        let cancion2 = Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock);

        assert!(cancion.equals(&cancion));
        assert!(!cancion.equals(&cancion2));
    }

    #[test]
    fn test_cancion_reporte_to_string() {
        let cancion = CancionReporte::new(
            "Stairway to Heaven".to_owned(), 
            "Led Zeppelin".to_owned(), 
            0
        );

        assert_eq!(cancion.to_string(), "Titulo: Stairway to Heaven\nArtista: Led Zeppelin\nPosicion: 0");
    }

    #[test]
    fn test_cancion_reporte_equals() {
        let cancion = CancionReporte::new(
            "Stairway to Heaven".to_owned(), 
            "Led Zeppelin".to_owned(), 
            0
        );
        let cancion2 = CancionReporte::new(
            "Bohemian Rhapsody".to_owned(), 
            "Queen".to_owned(), 
            1
        );

        assert!(cancion.equals(&cancion));
        assert!(!cancion.equals(&cancion2));
    }

    #[test]
    fn test_new_playlist() {
        let playlist = Playlist::new("Rock Classics".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)
        ]);

        assert_eq!(playlist.nombre, "Rock Classics");
        assert_eq!(playlist.canciones.len(), 2);
    }

    #[test]
    fn test_agregar_cancion() {
        let mut playlist = Playlist::new("Rock Classics".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
        ]);

        let cancion = Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock);
        playlist.agregar_cancion(cancion.clone());

        assert_eq!(playlist.canciones.len(), 2);
        assert!(playlist.canciones[1].equals(&cancion));
    }

    #[test]
    fn test_eliminar_cancion() {
        let mut playlist = Playlist::new("Rock Classics".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)
        ]);

        let cancion = Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        playlist.eliminar_cancion(&cancion);

        assert_eq!(playlist.canciones.len(), 1);
    }

    #[test]
    fn test_mover_cancion() {
        let mut playlist = Playlist::new("Rock Classics".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock),
            Cancion::new("Hotel California".to_string(), "Eagles".to_string(), Genero::Rock)
        ]);

        let cancion = Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock);
        playlist.mover_cancion(&cancion, 0);

        assert!(playlist.canciones[0].equals(&cancion));
    }

    #[test]
    fn test_buscar_cancion_por_nombre() {
        let playlist = Playlist::new("Rock Classics".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock),
            Cancion::new("Hotel California".to_string(), "Eagles".to_string(), Genero::Rock)
        ]);

        let cancion = playlist.buscar_cancion_por_nombre("Stairway to Heaven".to_string());

        assert!(cancion.is_some());
        assert!(cancion.unwrap().equals(&Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)))
    }

    #[test]
    fn test_obtener_canciones_genero() {
        let playlist = Playlist::new("Rock Classics".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Thriller".to_string(), "Michael Jackson".to_string(), Genero::Pop),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock),
            Cancion::new("Billie Jean".to_string(), "Michael Jackson".to_string(), Genero::Pop),
        ]);

        let canciones_rock = playlist.obtener_canciones_genero(&Genero::Rock);
        let canciones_pop = playlist.obtener_canciones_genero(&Genero::Pop);

        assert_eq!(canciones_rock.len(), 2);
        assert_eq!(canciones_pop.len(), 2);
    }

    #[test]
    fn test_obtener_canciones_artista() {
        let playlist = Playlist::new("Rock Classics".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Thriller".to_string(), "Michael Jackson".to_string(), Genero::Pop),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock),
            Cancion::new("Billie Jean".to_string(), "Michael Jackson".to_string(), Genero::Pop),
        ]);

        let canciones_queen = playlist.obtener_canciones_artista("Queen".to_string());
        let canciones_mj = playlist.obtener_canciones_artista("Michael Jackson".to_string());

        assert_eq!(canciones_queen.len(), 1);
        assert_eq!(canciones_mj.len(), 2);
    }

    #[test]
    fn test_modificar_titulo() {
        let mut playlist = Playlist::new("Rock Classics".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)
        ]);

        playlist.modificar_titulo("Best of Rock".to_string());

        assert_eq!(playlist.nombre, "Best of Rock");
    }

    #[test]
    fn test_vaciar() {
        let mut playlist = Playlist::new("Rock Classics".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)
        ]);

        playlist.vaciar();

        assert_eq!(playlist.canciones.len(), 0);
    }

    #[test]
    fn test_generar_reporte_genero() {
        let playlist = Playlist::new("Rock Classics".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Thriller".to_string(), "Michael Jackson".to_string(), Genero::Pop),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock),
            Cancion::new("Billie Jean".to_string(), "Michael Jackson".to_string(), Genero::Pop),
        ]);

        let reporte = playlist.generar_reporte_genero(Genero::Rock);

        assert_eq!(reporte.total_canciones, reporte.canciones.len() as u32);
        assert_eq!(reporte.total_canciones, 2);
        assert!(reporte.canciones[0].equals(&CancionReporte::new(
            "Bohemian Rhapsody".to_owned(), 
            "Queen".to_owned(), 
            0
        )));
        assert!(reporte.canciones[1].equals(&CancionReporte::new(
            "Stairway to Heaven".to_owned(), 
            "Led Zeppelin".to_owned(), 
            2
        )))
    }
}