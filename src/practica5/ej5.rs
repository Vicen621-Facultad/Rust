use std::{collections::HashMap, io::{Read, Write}};
use serde::{Deserialize, Serialize};

use crate::practica3::ej3::Fecha;

struct StreamingRust {
    file_name: String,
    usuarios: Vec<Usuario>,
    suscripciones: Vec<Suscripcion>
}

struct Usuario {
    id: u32,
    nombre: String,
    metodo_pago: MetodoPago,
}

#[derive(Serialize, Deserialize)]
struct Suscripcion {
    tipo_suscripcion: TipoSuscripcion,
    estado: EstadoSuscripcion,
    duracion_meses: u32,
    fecha_inicio: Fecha,
    id_usuario: u32
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
enum EstadoSuscripcion {
    Activa,
    Inactiva,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum MetodoPago {
    Efectivo,
    MercadoPago { id_cuenta: String },
    Credito { numero_tarjeta: String, expiracion: String },
    TransferenciaBancaria { cuenta_bancaria: String },
    Cripto { billetera: String },
}

impl StreamingRust {
    fn new(file_name: &str) -> Self {
        let suscripciones = match std::fs::File::open("test/".to_owned() + file_name + ".json") {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                let suscripciones: Vec<Suscripcion> = serde_json::from_str(&buf).unwrap();
                suscripciones
            },
            Err(_) => Vec::new()
        };

        StreamingRust {
            file_name: file_name.to_string(),
            usuarios: Vec::new(),
            suscripciones
        }
    }
}

trait GestorUsuarios {
    fn crear_usuario(&mut self, id_usuario: u32, nombre: String, metodo_pago: MetodoPago) -> &Usuario;
    fn get_usuario(&self, id_usuario: u32) -> Option<&Usuario>;
}

impl GestorUsuarios for StreamingRust {
    fn crear_usuario(&mut self, id_usuario: u32, nombre: String, metodo_pago: MetodoPago) -> &Usuario {
        let usuario = Usuario::new(id_usuario, nombre, metodo_pago);
        self.usuarios.push(usuario);
        self.usuarios.last().unwrap()
    }

    fn get_usuario(&self, id_usuario: u32) -> Option<&Usuario> {
        self.usuarios.iter().find(|usuario| usuario.id == id_usuario)   
    }
}

trait Estadisticas {
    fn metodo_pago_activo_mas_usado(&self) -> Option<MetodoPago>;
    fn tipo_suscripcion_activa_mas_usada(&self) -> Option<TipoSuscripcion>;
    fn metodo_pago_mas_usado(&self) -> Option<MetodoPago>;
    fn tipo_suscripcion_mas_usado(&self) -> Option<TipoSuscripcion>;
}

impl Estadisticas for StreamingRust {
    fn metodo_pago_activo_mas_usado(&self) -> Option<MetodoPago> {
        let mut metodo_pagos = HashMap::new();

        self.suscripciones.iter()
            .filter(|subscripcion| subscripcion.esta_activa())
            .map(|subscripcion| self.get_usuario(subscripcion.id_usuario))
            .filter(|usuario| usuario.is_some())
            .map(|usuario| usuario.unwrap())
            .for_each(|usuario| {
                *metodo_pagos.entry(usuario.metodo_pago.clone()).or_insert(0) += 1;
            });

        metodo_pagos.iter().max_by_key(|(_, count)| *count).map(|(tipo_suscripcion, _)| tipo_suscripcion.clone())
    }

    fn tipo_suscripcion_activa_mas_usada(&self) -> Option<TipoSuscripcion> {
        let mut suscripciones = HashMap::new();
        self.suscripciones.iter()
        .filter(|subscripcion| subscripcion.esta_activa())
        .for_each(|subscripcion| {
            *suscripciones.entry(subscripcion.tipo_suscripcion.clone()).or_insert(0) += 1;
        });

        suscripciones.iter()
        .max_by_key(|(_, count)| *count)
        .map(|(tipo_suscripcion, _)| tipo_suscripcion.clone())
    }

    fn metodo_pago_mas_usado(&self) -> Option<MetodoPago> {
        let mut metodo_pagos = HashMap::new();
        for usuario in &self.usuarios {
            *metodo_pagos.entry(usuario.metodo_pago.clone()).or_insert(0) += 1;
        }

        metodo_pagos.iter()
        .max_by_key(|(_, count)| *count)
        .map(|(metodo_pago, _)| metodo_pago.clone())
    }

    fn tipo_suscripcion_mas_usado(&self) -> Option<TipoSuscripcion> {
        let mut suscripciones = HashMap::new();
        for subscripcion in &self.suscripciones {
            *suscripciones.entry(subscripcion.tipo_suscripcion.clone()).or_insert(0) += 1;
        }

        suscripciones.iter()
        .max_by_key(|(_, count)| *count)
        .map(|(tipo_suscripcion, _)| tipo_suscripcion.clone())
    }
}
trait GestorSuscripciones {
    fn crear_subscripcion(&mut self, id_usuario: u32, nombre: String, tipo_suscripcion: TipoSuscripcion, duracion_meses: u32, metodo_pago: MetodoPago) -> Result<(), std::io::Error>;
    fn upgrade_subscripcion(&mut self, id_usuario: u32) -> Result<(), std::io::Error>;
    fn downgrade_subscripcion(&mut self, id_usuario: u32) -> Result<(), std::io::Error>;
    fn cancel_subscripcion(&mut self, id_usuario: u32) -> Result<(), std::io::Error>;
    fn get_subscripcion(&mut self, id_usuario: u32) -> Option<&mut Suscripcion>;
    fn escribir_archivo(&self) -> Result<(), std::io::Error>;
}

impl GestorSuscripciones for StreamingRust {
    fn crear_subscripcion(&mut self, id_usuario: u32, nombre: String, tipo_suscripcion: TipoSuscripcion, duracion_meses: u32, metodo_pago: MetodoPago) -> Result<(), std::io::Error> {
        if self.get_usuario(id_usuario).is_none() {
            self.crear_usuario(id_usuario, nombre.clone(), metodo_pago.clone());
        }

        match self.get_subscripcion(id_usuario) {
            None => {
                let usuario = self.get_usuario(id_usuario).unwrap();
                let subscripcion = Suscripcion::new(tipo_suscripcion, duracion_meses, usuario.id);
                self.suscripciones.push(subscripcion);
                self.escribir_archivo()
            },
            Some(_) => Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Ya existe la subscripcion"))
        }
    }

    fn upgrade_subscripcion(&mut self, id_usuario: u32) -> Result<(), std::io::Error> {
        let subscripcion: Option<&mut Suscripcion> = self.get_subscripcion(id_usuario);
        match subscripcion {
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No se encontro la subscripcion")),
            Some(subscripcion) => {
                subscripcion.upgrade();
                self.escribir_archivo()
            }
        }
    }

    fn downgrade_subscripcion(&mut self, id_usuario: u32) -> Result<(), std::io::Error> {
        let subscripcion = self.get_subscripcion(id_usuario);
        match subscripcion {
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No se encontro la subscripcion")),
            Some(subscripcion) => {
                subscripcion.downgrade();
                self.escribir_archivo()
            }
        }
    }

    fn cancel_subscripcion(&mut self, id_usuario: u32) -> Result<(), std::io::Error> {
        let subscripcion = self.get_subscripcion(id_usuario);
        match subscripcion {
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No se encontro la subscripcion")),
            Some(subscripcion) => {
                subscripcion.cancel();
                self.escribir_archivo()
            }
        }
    }

    fn get_subscripcion(&mut self, id_usuario: u32) -> Option<&mut Suscripcion> {
        self.suscripciones.iter_mut().find(|subscripcion| subscripcion.id_usuario == id_usuario)
    }

    fn escribir_archivo(&self) -> Result<(), std::io::Error>{
        let mut file = std::fs::File::create("test/".to_owned() + &self.file_name + ".json")?;
        let serialized = serde_json::to_string(&self.suscripciones)?;
        file.write_all(&serialized.as_bytes())?;
        Ok(())
    }
}

impl Usuario {
    fn new(id: u32, nombre: String, metodo_pago: MetodoPago) -> Self {
        Usuario {
            id,
            nombre,
            metodo_pago
        }
    }

}

impl Suscripcion {
    fn new(tipo_suscripcion: TipoSuscripcion, duracion_meses: u32, id_usuario: u32) -> Self {
        Suscripcion {
            estado: EstadoSuscripcion::Activa,
            fecha_inicio: Fecha::now(),
            tipo_suscripcion,
            duracion_meses,
            id_usuario
        }
    }

    fn upgrade(&mut self) {
        self.tipo_suscripcion = match self.tipo_suscripcion {
            TipoSuscripcion::Basic => TipoSuscripcion::Clasic,
            TipoSuscripcion::Clasic => TipoSuscripcion::Super,
            TipoSuscripcion::Super => TipoSuscripcion::Super, // No se puede mejorar más
        };
    }

    fn downgrade(&mut self) {
        self.tipo_suscripcion = match self.tipo_suscripcion {
            TipoSuscripcion::Super => TipoSuscripcion::Clasic,
            TipoSuscripcion::Clasic => TipoSuscripcion::Basic,
            TipoSuscripcion::Basic => {
                self.cancel();
                return;
            }
        };
    }

    fn cancel(&mut self) {
        self.estado = EstadoSuscripcion::Inactiva;
    }

    fn costo(&self) -> f64 {
        self.tipo_suscripcion.costo() * self.duracion_meses as f64
    }

    fn esta_activa(&self) -> bool {
        self.estado == EstadoSuscripcion::Activa
    }
}

impl TipoSuscripcion {
    fn costo(&self) -> f64 {
        match self {
            TipoSuscripcion::Basic => 10.0,
            TipoSuscripcion::Clasic => 20.0,
            TipoSuscripcion::Super => 30.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_streaming_con_datos() {
        let mut streaming = StreamingRust::new("test_crear_streaming_con_datos");
        assert_eq!(streaming.suscripciones.len(), 0);
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo).is_ok());
        assert_eq!(streaming.suscripciones.len(), 1);

        let streaming = StreamingRust::new("test_crear_streaming_con_datos");
        assert_eq!(streaming.suscripciones.len(), 1);
    }

    #[test]
    fn test_subscripcion_cost() {
        let subscripcion = Suscripcion::new(TipoSuscripcion::Basic, 3, 1);
        assert_eq!(subscripcion.costo(), 30.0);
    }

    #[test]
    fn test_crear_subscripcion() {
        let mut streaming = StreamingRust::new("test_crear_subscripcion");
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo).is_ok());
        let usuario = streaming.get_usuario(1).unwrap();
        assert_eq!(usuario.nombre, "Juan");
        assert_eq!(usuario.metodo_pago, MetodoPago::Efectivo);
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.tipo_suscripcion, TipoSuscripcion::Basic);
        assert_eq!(subscripcion.duracion_meses, 3);
        assert_eq!(subscripcion.id_usuario, 1);
    }

    #[test]
    fn test_upgrade_subscripcion() {
        let mut streaming = StreamingRust::new("test_upgrade_subscripcion");
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.upgrade_subscripcion(1).is_ok());
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.tipo_suscripcion, TipoSuscripcion::Clasic);
    }

    #[test]
    fn test_upgrade_subscripcion_super() {
        let mut streaming = StreamingRust::new("test_upgrade_subscripcion_super");
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.upgrade_subscripcion(1).is_ok());
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.tipo_suscripcion, TipoSuscripcion::Super);
    }

    #[test]
    fn test_downgrade_subscripcion() {
        let mut streaming = StreamingRust::new("test_downgrade_subscripcion");
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.downgrade_subscripcion(1).is_ok());
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.tipo_suscripcion, TipoSuscripcion::Clasic);
    }

    #[test]
    fn test_downgrade_subscripcion_cancel() {
        let mut streaming = StreamingRust::new("test_downgrade_subscripcion_cancel");
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.downgrade_subscripcion(1).is_ok());
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.estado, EstadoSuscripcion::Inactiva);
    }

    #[test]
    fn test_cancel_subscripcion() {
        let mut streaming = StreamingRust::new("test_cancel_subscripcion");
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.cancel_subscripcion(1).is_ok());
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.estado, EstadoSuscripcion::Inactiva);
    }

    #[test]
    fn test_most_used_active_metodo_pago() {
        let mut streaming = StreamingRust::new("test_most_used_active_metodo_pago");
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(2, "Pedro".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(3, "Pablo".to_string(), TipoSuscripcion::Super, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() }).is_ok());
        assert!(streaming.crear_subscripcion(4, "Jose".to_string(), TipoSuscripcion::Super, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() }).is_ok());
        assert!(streaming.crear_subscripcion(5, "Pepe".to_string(), TipoSuscripcion::Super, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() }).is_ok());
        assert!(streaming.cancel_subscripcion(4).is_ok());
        assert!(streaming.cancel_subscripcion(5).is_ok());
        let metodo_pago = streaming.metodo_pago_activo_mas_usado().unwrap();
        assert_eq!(metodo_pago, MetodoPago::Efectivo);
    }

    #[test]
    fn test_most_popular_active_subscripcion() {
        let mut streaming = StreamingRust::new("test_most_popular_active_subscripcion");
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(2, "Pedro".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(3, "Pablo".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(4, "Jose".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(5, "Pepe".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.cancel_subscripcion(4).is_ok());
        assert!(streaming.cancel_subscripcion(5).is_ok());
        let tipo_suscripcion = streaming.tipo_suscripcion_activa_mas_usada().unwrap();
        assert_eq!(tipo_suscripcion, TipoSuscripcion::Super);
    }

    #[test]
    fn test_most_used_metodo_pago() {
        let mut streaming = StreamingRust::new("test_most_used_metodo_pago");
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(2, "Pedro".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(3, "Pablo".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(4, "Jose".to_string(), TipoSuscripcion::Super, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() }).is_ok());
        assert!(streaming.crear_subscripcion(5, "Pepe".to_string(), TipoSuscripcion::Super, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() }).is_ok());
        assert!(streaming.cancel_subscripcion(1).is_ok());
        assert!(streaming.cancel_subscripcion(2).is_ok());
        let metodo_pago = streaming.metodo_pago_mas_usado().unwrap();
        assert_eq!(metodo_pago, MetodoPago::Efectivo);
    }

    #[test]
    fn test_most_popular_subscripcion() {
        let mut streaming = StreamingRust::new("test_most_popular_subscripcion");
        assert!(streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(2, "Pedro".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(3, "Pablo".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(4, "Jose".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.crear_subscripcion(5, "Pepe".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo).is_ok());
        assert!(streaming.cancel_subscripcion(4).is_ok());
        assert!(streaming.cancel_subscripcion(5).is_ok());
        let tipo_suscripcion = streaming.tipo_suscripcion_mas_usado().unwrap();
        assert_eq!(tipo_suscripcion, TipoSuscripcion::Basic);
    }
}