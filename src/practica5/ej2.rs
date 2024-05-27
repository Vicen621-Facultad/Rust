use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    fn to_string(&self) -> String {
        format!(
            "Título: {}\nArtista: {}\nGénero: {}",
            self.titulo, self.artista, self.genero.to_string()
        )
    }

    fn equals(&self, other: &Cancion) -> bool {
        self.to_string() == other.to_string()
    }
}

struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>
}

impl Playlist {
    fn new(nombre: String, mut canciones: Vec<Cancion>) -> Playlist {
        let canciones = match std::fs::File::open("test/".to_owned() + nombre.as_str() + ".json") {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                let mut canciones_nuevas: Vec<Cancion> = serde_json::from_str(&buf).unwrap();
                canciones_nuevas.append(&mut canciones);
                canciones_nuevas
            },
            Err(_) => canciones
        };
        Playlist {
            nombre,
            canciones
        }
    }

    fn escribir_archivo(&self) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create("test/".to_owned() + self.nombre.as_str() + ".json")?;
        let serialized = serde_json::to_string(&self.canciones)?;
        file.write_all(&serialized.as_bytes())?;
        Ok(())
    }

    fn agregar_cancion(&mut self, cancion: Cancion) -> Result<(), std::io::Error> {
        self.canciones.push(cancion);
        self.escribir_archivo()
    }


    fn eliminar_cancion(&mut self, cancion: &Cancion) -> Result<(), std::io::Error> {
        let opt = self.obtener_pos_cancion(cancion);

        if let Some(index) = opt {
            self.canciones.remove(index);
            self.escribir_archivo()
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Canción no encontrada"))
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

    fn mover_cancion(&mut self, cancion: &Cancion, pos: usize) -> Result<(), std::io::Error>{
        let opt = self.obtener_pos_cancion(cancion);

        if let Some(index) = opt {
            let cancion = self.canciones.remove(index);
            self.canciones.insert(pos, cancion);
            self.escribir_archivo()
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Canción no encontrada"))
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

    fn vaciar(&mut self) -> Result<(), std::io::Error>{
        self.canciones.clear();
        self.escribir_archivo()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cancion() {
        let cancion = Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);

        assert_eq!(cancion.titulo, "Bohemian Rhapsody");
        assert_eq!(cancion.artista, "Queen");
        assert!(cancion.genero.equals(&Genero::Rock));
    }

    #[test]
    fn test_new_playlist() {
        let playlist = Playlist::new("test_new_playlist".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)
        ]);

        assert_eq!(playlist.nombre, "test_new_playlist");
        assert_eq!(playlist.canciones.len(), 2);
    }

    #[test]
    fn test_new_playlist_with_songs() {
        let playlist = Playlist::new("test_new_playlist_with_songs_1".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)
        ]);

        playlist.escribir_archivo().unwrap();

        let playlist = Playlist::new("test_new_playlist_with_songs_1".to_string(), vec![
            Cancion::new("Thriller".to_string(), "Micheal Jackson".to_string(), Genero::Pop),
        ]);

        assert_eq!(playlist.nombre, "test_new_playlist_with_songs_1");
        assert_eq!(playlist.canciones.len(), 3);
    }

    #[test]
    fn test_agregar_cancion() {
        let mut playlist = Playlist::new("test_agregar_cancion".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
        ]);

        let cancion = Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock);
        playlist.agregar_cancion(cancion.clone()).unwrap();

        assert_eq!(playlist.canciones.len(), 2);
        assert!(playlist.canciones[1].equals(&cancion));
    }

    #[test]
    fn test_eliminar_cancion() {
        let mut playlist = Playlist::new("test_eliminar_cancion".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)
        ]);

        let cancion = Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock);
        playlist.eliminar_cancion(&cancion).unwrap();

        assert_eq!(playlist.canciones.len(), 1);
    }

    #[test]
    fn test_mover_cancion() {
        let mut playlist = Playlist::new("test_mover_cancion".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock),
            Cancion::new("Hotel California".to_string(), "Eagles".to_string(), Genero::Rock)
        ]);

        let cancion = Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock);
        playlist.mover_cancion(&cancion, 0).unwrap();

        assert!(playlist.canciones[0].equals(&cancion));
    }

    #[test]
    fn test_buscar_cancion_por_nombre() {
        let playlist = Playlist::new("test_buscar_cancion_por_nombre".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock),
            Cancion::new("Hotel California".to_string(), "Eagles".to_string(), Genero::Rock)
        ]);

        let cancion = playlist.buscar_cancion_por_nombre("Stairway to Heaven".to_string());

        assert!(cancion.is_some());
        assert!(cancion.unwrap().equals(&Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)));
    }

    #[test]
    fn test_obtener_canciones_genero() {
        let playlist = Playlist::new("test_obtener_canciones_genero".to_string(), vec![
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
        let playlist = Playlist::new("test_obtener_canciones_artista".to_string(), vec![
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
        let mut playlist = Playlist::new("test_modificar_titulo".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)
        ]);

        playlist.modificar_titulo("Best of Rock".to_string());

        assert_eq!(playlist.nombre, "Best of Rock");
    }

    #[test]
    fn test_vaciar() {
        let mut playlist = Playlist::new("test_vaciar".to_string(), vec![
            Cancion::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), Genero::Rock),
            Cancion::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), Genero::Rock)
        ]);

        playlist.vaciar().unwrap();

        assert_eq!(playlist.canciones.len(), 0);
    }
}