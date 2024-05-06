use std::collections::VecDeque;
use crate::practica3::ej3::Fecha;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum Animal {
    Perro,
    Gato,
    Caballo,
    Otros
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
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
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
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
}

#[derive(PartialEq, Eq, Clone, Debug)]
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
        let position = self.cola.iter().position(|m| m == mascota);

        if let Some(index) = position {
            self.cola.remove(index);
        }
    }

    fn registrar_atencion(&mut self, atencion: AtencionRealizada) {
        self.atenciones.push(atencion);
    }

    fn buscar_atencion_mascota(&self, nombre: String) -> Option<&AtencionRealizada> {
        self.atenciones
            .iter()
            .find(|a| a.mascota.nombre == nombre)
    }

    fn buscar_atencion_duenio(&self, nombre: String) -> Option<&AtencionRealizada> {
        self.atenciones
            .iter()
            .find(|a| a.mascota.duenio.nombre == nombre)
    }

    fn buscar_atencion_telefono(&self, telefono: String) -> Option<&AtencionRealizada> {
        self.atenciones
            .iter()
            .find(|a| a.mascota.duenio.telefono == telefono)
    }

    fn modificar_diagnostico(&mut self, diagnostico: String, atencion: &AtencionRealizada) {
        let atencion = self.eliminar_atencion(atencion);

        if let Some(mut at) = atencion {
            at.diagnostico = diagnostico;
            self.registrar_atencion(at);
        }
    }

    fn modificar_fecha(&mut self, fecha: Option<Fecha>, atencion: &AtencionRealizada) {
        let atencion = self.eliminar_atencion(atencion);

        if let Some(mut at) = atencion {
            at.proxima_visita = fecha;
            self.registrar_atencion(at);
        }
    }

    fn eliminar_atencion(&mut self, atencion: &AtencionRealizada) -> Option<AtencionRealizada> {
        let position = self.atenciones.iter().position(|a| a == atencion);

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
        assert_eq!(vet.cola.front(), Some(&mascota));
    }
    
    #[test]
    fn test_agregar_mascota_prioridad() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let mascota1 = Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string()));
        let mascota2 = Mascota::new("Sol".to_string(), 3, Animal::Gato, Duenio::new("Maria".to_string(), "Calle B".to_string(), "987654321".to_string()));
        
        vet.agregar_mascota_prioridad(mascota1.clone());
        vet.agregar_mascota_prioridad(mascota2.clone());
        
        assert_eq!(vet.cola.len(), 2);
        assert_eq!(vet.cola.front(), Some(&mascota2));
    }
    
    #[test]
    fn test_atender_mascota() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let mascota = Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string()));
        
        vet.agregar_mascota(mascota.clone());
        
        let atendida = vet.atender_mascota();
        
        assert_eq!(atendida, Some(mascota));
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
        assert_eq!(vet.atenciones.get(0), Some(&atencion));
    }
    
    #[test]
    fn test_buscar_atencion_mascota() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let mascota = Mascota::new("Luna".to_string(), 5, Animal::Perro, Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string()));
        let atencion = AtencionRealizada::new(mascota.clone(), "Diagnóstico".to_string(), "Tratamiento".to_string(), None);
        
        vet.registrar_atencion(atencion.clone());
        
        let buscada = vet.buscar_atencion_mascota("Luna".to_string());
        
        assert_eq!(buscada, Some(&atencion));
    }
    
    #[test]
    fn test_buscar_atencion_duenio() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let duenio = Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string());
        let atencion = AtencionRealizada::new(Mascota::new("Luna".to_string(), 5, Animal::Perro, duenio.clone()), "Diagnóstico".to_string(), "Tratamiento".to_string(), None);
        
        vet.registrar_atencion(atencion.clone());
        
        let buscada = vet.buscar_atencion_duenio("Juan".to_string());
        
        assert_eq!(buscada, Some(&atencion));
    }
    
    #[test]
    fn test_buscar_atencion_telefono() {
        let mut vet = Veterinaria::new("Vet".to_string(), "Dirección".to_string(), 1, None, None);
        let duenio = Duenio::new("Juan".to_string(), "Calle A".to_string(), "123456789".to_string());
        let atencion = AtencionRealizada::new(Mascota::new("Luna".to_string(), 5, Animal::Perro, duenio.clone()), "Diagnóstico".to_string(), "Tratamiento".to_string(), None);
        
        vet.registrar_atencion(atencion.clone());
        
        let buscada = vet.buscar_atencion_telefono("123456789".to_string());
        
        assert_eq!(buscada, Some(&atencion));
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
        
        let nueva_fecha = Some(Fecha::new(2024, 5, 10)); // Cambiar por la fecha deseada
        
        vet.modificar_fecha(nueva_fecha.clone(), &atencion);
        
        let modificado = vet.buscar_atencion_mascota("Luna".to_string()).unwrap();
        
        assert_eq!(modificado.proxima_visita, nueva_fecha);
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