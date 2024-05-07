use std::collections::VecDeque;
use super::ej3::Fecha;

#[derive(Debug, Clone)]
enum Animal {
    Perro,
    Gato,
    Caballo,
    Otros
}

impl Animal {
    fn to_string(&self) -> String {
        match self {
            Animal::Perro => String::from("perro"),
            Animal::Gato => String::from("gato"),
            Animal::Caballo => String::from("caballo"),
            Animal::Otros => String::from("otros"),
        }
    }

    fn equals(&self, other: &Animal) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Debug, Clone)]
struct Mascota {
    nombre: String,
    edad: u32, 
    tipo: Animal,
    duenio: Duenio,
}

impl Mascota {
    fn new(nombre: String, edad: u32, tipo: Animal, duenio: Duenio) -> Mascota {
        Mascota {
            nombre,
            edad,
            tipo,
            duenio
        }
    }

    fn to_string(&self) -> String {
        format!("Nombre: {}\nEdad: {}\nTipo: {}\nDueño: {}", self.nombre, self.edad, self.tipo.to_string(), self.duenio.to_string())
    }

    fn equals(&self, other: &Mascota) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Debug, Clone)]
struct Duenio {
    nombre: String,
    direccion: String,
    telefono: String
}

impl Duenio {
    fn new(nombre: String, direccion: String, telefono: String) -> Duenio {
        Duenio {
            nombre,
            direccion,
            telefono
        }
    }

    fn to_string(&self) -> String {
        format!("Nombre: {}\nDirección: {}\nTeléfono: {}", self.nombre, self.direccion, self.telefono)
    }

    fn equals(&self, other: &Duenio) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Clone, Debug)]
struct AtencionRealizada {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>,
}

impl AtencionRealizada {
    fn new(mascota: Mascota, diagnostico: String, tratamiento: String, proxima_visita: Option<Fecha>) -> AtencionRealizada {
        AtencionRealizada {
            mascota,
            diagnostico,
            tratamiento,
            proxima_visita
        }
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("Mascota:\n{}\n", self.mascota.to_string()));
        result.push_str(&format!("Diagnóstico: {}\n", self.diagnostico));
        result.push_str(&format!("Tratamiento: {}\n", self.tratamiento));
        result.push_str(&format!("Próxima visita: {}\n", 
            match &self.proxima_visita {
                Some(fecha) => fecha.to_string(),
                None => String::from("No programada"),
            }
        ));
        result
    }

    fn equals(&self, other: &AtencionRealizada) -> bool {
        self.to_string() == other.to_string()
    }
}

struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    atenciones: Vec<AtencionRealizada>,
    cola: VecDeque<Mascota>
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

    fn atender_mascota(&mut self) -> Option<Mascota> {
        self.cola.pop_front()
    }

    fn eliminar_mascota(&mut self, mascota: &Mascota) {
        let mut position = None;

        for i in 0..self.cola.len() {
            if let Some(m) = self.cola.get(i) {
                if m.equals(mascota) {
                    position = Some(i);
                    break;
                }
            }
        }


        if let Some(index) = position {
            self.cola.remove(index);
        }
    }

    fn registrar_atencion(&mut self, atencion: AtencionRealizada) {
        self.atenciones.push(atencion);
    }

    fn buscar_atencion_mascota(&self, nombre: String) -> Option<&AtencionRealizada> {
        let mut ret = None;

        for atencion in &self.atenciones {
            if atencion.mascota.nombre == nombre {
                ret = Some(atencion);
                break;
            }
        }

        ret
    }

    fn buscar_atencion_duenio(&self, nombre: String) -> Option<&AtencionRealizada> {
        let mut ret = None;

        for atencion in &self.atenciones {
            if atencion.mascota.duenio.nombre == nombre {
                ret = Some(atencion);
                break;
            }
        }

        ret
    }

    fn buscar_atencion_telefono(&self, telefono: String) -> Option<&AtencionRealizada> {
        let mut ret = None;

        for atencion in &self.atenciones {
            if atencion.mascota.duenio.telefono == telefono {
                ret = Some(atencion);
                break;
            }
        }

        ret
    }

    fn modificar_diagnostico(&mut self, diagnostico: String, atencion: &AtencionRealizada) {
        for at in &mut self.atenciones {
            if at.equals(atencion) {
                at.diagnostico = diagnostico;
                break;
            }
        }
    }

    fn modificar_fecha(&mut self, fecha: Option<Fecha>, atencion: &AtencionRealizada) {
        for at in &mut self.atenciones {
            if at.equals(atencion) {
                at.proxima_visita = fecha;
                break;
            }
        }
    }

    fn eliminar_atencion(&mut self, atencion: &AtencionRealizada) -> Option<AtencionRealizada> {
        let mut position = None;

        for i in 0..self.atenciones.len() {
            if let Some(at) = self.atenciones.get(i) {
                if at.equals(atencion) {
                    position = Some(i);
                    break;
                }
            }
        }

        if let Some(index) = position {
            Some(self.atenciones.remove(index))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agregar_mascota() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let mascota = Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string()));
        
        vet.agregar_mascota(mascota.clone());
        
        assert_eq!(vet.cola.len(), 1);
        let front = vet.cola.front();
        assert!(front.is_some());
        assert!(front.unwrap().equals(&mascota));
    }
    
    #[test]
    fn test_agregar_mascota_prioridad() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let mascota1 = Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string()));
        let mascota2 = Mascota::new("Sol".to_string(), 3, Animal::Gato, Duenio::new("Maria".to_string(), "Calle B".to_string(), "987654321".to_string()));
        
        vet.agregar_mascota_prioridad(mascota1.clone());
        vet.agregar_mascota_prioridad(mascota2.clone());
        
        assert_eq!(vet.cola.len(), 2);
        let front = vet.cola.front();
        assert!(front.is_some());
        assert!(front.unwrap().equals(&mascota2));
    }
    
    #[test]
    fn test_atender_mascota() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let mascota = Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string()));
        
        vet.agregar_mascota(mascota.clone());
        
        let atendida = vet.atender_mascota();

        assert!(atendida.is_some());
        assert!(atendida.unwrap().equals(&mascota));
        assert_eq!(vet.cola.len(), 0);
    }
    
    #[test]
    fn test_eliminar_mascota() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let mascota = Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string()));
        
        vet.agregar_mascota(mascota.clone());
        
        vet.eliminar_mascota(&mascota);
        
        assert_eq!(vet.cola.len(), 0);
    }
    
    #[test]
    fn test_registrar_atencion() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let atencion = AtencionRealizada::new(Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string())), "Diagnóstico".to_string(), "Tratamiento".to_string(), None);
        
        vet.registrar_atencion(atencion.clone());
        
        assert_eq!(vet.atenciones.len(), 1);
        let get_0 = vet.atenciones.get(0);
        assert!(get_0.is_some());
        assert!(get_0.unwrap().equals(&atencion));
    }
    
    #[test]
    fn test_buscar_atencion_mascota() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let mascota = Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string()));
        let atencion = AtencionRealizada::new(mascota.clone(), "Diagnóstico".to_string(), "Tratamiento".to_string(), None);
        
        vet.registrar_atencion(atencion.clone());
        
        let buscada = vet.buscar_atencion_mascota("Luna".to_string());
        assert!(buscada.is_some());
        assert!(buscada.unwrap().equals(&atencion));
    }
    
    #[test]
    fn test_buscar_atencion_duenio() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let duenio = Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string());
        let atencion = AtencionRealizada::new(Mascota::new("Luna".to_string(), 5, Animal::Perro, duenio.clone()), "Diagnóstico".to_string(), "Tratamiento".to_string(), None);
        
        vet.registrar_atencion(atencion.clone());
        
        let buscada = vet.buscar_atencion_duenio("Juan".to_string());
        assert!(buscada.is_some());
        assert!(buscada.unwrap().equals(&atencion));
    }
    
    #[test]
    fn test_buscar_atencion_telefono() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let duenio = Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string());
        let atencion = AtencionRealizada::new(Mascota::new("Luna".to_string(), 5, Animal::Perro, duenio.clone()), "Diagnóstico".to_string(), "Tratamiento".to_string(), None);
        
        vet.registrar_atencion(atencion.clone());
        
        let buscada = vet.buscar_atencion_telefono("123456789".to_string());
        assert!(buscada.is_some());
        assert!(buscada.unwrap().equals(&atencion));
    }
    
    #[test]
    fn test_modificar_diagnostico() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let atencion = AtencionRealizada::new(Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string())), "Diagnóstico".to_string(), "Tratamiento".to_string(), None);
        
        vet.registrar_atencion(atencion.clone());
        
        vet.modificar_diagnostico("Nuevo diagnóstico".to_string(), &atencion);
        
        let modificado = vet.buscar_atencion_mascota("Luna".to_string()).unwrap();
        
        assert_eq!(modificado.diagnostico, "Nuevo diagnóstico");
    }
    
    #[test]
    fn test_modificar_fecha() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let atencion = AtencionRealizada::new(Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string())), "Diagnóstico".to_string(), "Tratamiento".to_string(), None);
        
        vet.registrar_atencion(atencion.clone());
        
        let nueva_fecha = Fecha::new(10, 5, 2024); // Cambiar por la fecha deseada
        
        vet.modificar_fecha(Some(nueva_fecha.clone()), &atencion);
        
        let modificado = vet.buscar_atencion_mascota("Luna".to_string()).unwrap();
        
        assert!(modificado.proxima_visita.is_some());
        assert!(modificado.proxima_visita.as_ref().unwrap().equals(&nueva_fecha));
    }

    #[test]
    fn test_eliminar_atencion() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let atencion = AtencionRealizada::new(Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string())), "Diagnóstico".to_string(), "Tratamiento".to_string(), None);
        
        vet.registrar_atencion(atencion.clone());
        
        vet.eliminar_atencion(&atencion);
        
        assert_eq!(vet.atenciones.len(), 0);
    }
}