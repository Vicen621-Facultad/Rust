use std::collections::HashMap;
use crate::practica3::ej3::Fecha;

struct StreamingRust {
    usuarios: Vec<Usuario>,
    suscripciones: Vec<Suscripcion>
}

struct Usuario {
    id: u32,
    nombre: String,
    metodo_pago: MetodoPago,
}

struct Suscripcion {
    tipo_suscripcion: TipoSuscripcion,
    estado: EstadoSuscripcion,
    duracion_meses: u32,
    fecha_inicio: Fecha,
    id_usuario: u32
}

#[derive(Eq, PartialEq, Debug)]
enum EstadoSuscripcion {
    Activa,
    Inactiva,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
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
    fn new() -> Self {
        StreamingRust {
            usuarios: Vec::new(),
            suscripciones: Vec::new()
        }
    }
}

trait GestorUsuarios {
    fn crear_subscripcion(&mut self, id_usuario: u32, nombre: String, tipo_suscripcion: TipoSuscripcion, duracion_meses: u32, metodo_pago: MetodoPago);
    fn crear_usuario(&mut self, id_usuario: u32, nombre: String, metodo_pago: MetodoPago) -> &Usuario;
    fn get_usuario(&self, id_usuario: u32) -> Option<&Usuario>;
}

impl GestorUsuarios for StreamingRust {
    fn crear_subscripcion(&mut self, id_usuario: u32, nombre: String, tipo_suscripcion: TipoSuscripcion, duracion_meses: u32, metodo_pago: MetodoPago) {
        if self.get_usuario(id_usuario).is_none() {
            self.crear_usuario(id_usuario, nombre.clone(), metodo_pago.clone());
        }

        if self.get_subscripcion(id_usuario).is_none() {
            let usuario = self.get_usuario(id_usuario).unwrap();
            let subscripcion = Suscripcion::new(tipo_suscripcion, duracion_meses, usuario.id);
            self.suscripciones.push(subscripcion);
        }
    }

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
    //TODO: Preguntar a ver si esta bien esto o se deberia hacer de otra manera
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
    fn upgrade_subscripcion(&mut self, id_usuario: u32);
    fn downgrade_subscripcion(&mut self, id_usuario: u32);
    fn cancel_subscripcion(&mut self, id_usuario: u32);
    fn get_subscripcion(&mut self, id_usuario: u32) -> Option<&mut Suscripcion>;
}

impl GestorSuscripciones for StreamingRust {
    fn upgrade_subscripcion(&mut self, id_usuario: u32) {
        let subscripcion: Option<&mut Suscripcion> = self.get_subscripcion(id_usuario);
        if let Some(subscripcion) = subscripcion {
            subscripcion.upgrade();
        }
    }

    fn downgrade_subscripcion(&mut self, id_usuario: u32) {
        let subscripcion = self.get_subscripcion(id_usuario);
        if let Some(subscripcion) = subscripcion {
            subscripcion.downgrade();
        }
    }

    fn cancel_subscripcion(&mut self, id_usuario: u32) {
        let subscripcion = self.get_subscripcion(id_usuario);
        if let Some(subscripcion) = subscripcion {
            subscripcion.cancel();
        }
    }

    fn get_subscripcion(&mut self, id_usuario: u32) -> Option<&mut Suscripcion> {
        self.suscripciones.iter_mut().find(|subscripcion| subscripcion.id_usuario == id_usuario)
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
    fn test_subscripcion_cost() {
        let subscripcion = Suscripcion::new(TipoSuscripcion::Basic, 3, 1);
        assert_eq!(subscripcion.costo(), 30.0);
    }

    #[test]
    fn test_crear_subscripcion() {
        let mut streaming = StreamingRust::new();
        streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo);
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
        let mut streaming = StreamingRust::new();
        streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo);
        streaming.upgrade_subscripcion(1);
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.tipo_suscripcion, TipoSuscripcion::Clasic);
    }

    #[test]
    fn test_upgrade_subscripcion_super() {
        let mut streaming = StreamingRust::new();
        streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.upgrade_subscripcion(1);
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.tipo_suscripcion, TipoSuscripcion::Super);
    }

    #[test]
    fn test_downgrade_subscripcion() {
        let mut streaming = StreamingRust::new();
        streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.downgrade_subscripcion(1);
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.tipo_suscripcion, TipoSuscripcion::Clasic);
    }

    #[test]
    fn test_downgrade_subscripcion_cancel() {
        let mut streaming = StreamingRust::new();
        streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo);
        streaming.downgrade_subscripcion(1);
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.estado, EstadoSuscripcion::Inactiva);
    }

    #[test]
    fn test_cancel_subscripcion() {
        let mut streaming = StreamingRust::new();
        streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.cancel_subscripcion(1);
        let subscripcion = streaming.suscripciones.first().unwrap();
        assert_eq!(subscripcion.estado, EstadoSuscripcion::Inactiva);
    }

    #[test]
    fn test_most_used_active_metodo_pago() {
        let mut streaming = StreamingRust::new();
        streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(2, "Pedro".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(3, "Pablo".to_string(), TipoSuscripcion::Super, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() });
        streaming.crear_subscripcion(4, "Jose".to_string(), TipoSuscripcion::Super, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() });
        streaming.crear_subscripcion(5, "Pepe".to_string(), TipoSuscripcion::Super, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() });
        streaming.cancel_subscripcion(4);
        streaming.cancel_subscripcion(5);
        let metodo_pago = streaming.metodo_pago_activo_mas_usado().unwrap();
        assert_eq!(metodo_pago, MetodoPago::Efectivo);
    }

    #[test]
    fn test_most_popular_active_subscripcion() {
        let mut streaming = StreamingRust::new();
        streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(2, "Pedro".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(3, "Pablo".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(4, "Jose".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(5, "Pepe".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo);
        streaming.cancel_subscripcion(4);
        streaming.cancel_subscripcion(5);
        let tipo_suscripcion = streaming.tipo_suscripcion_activa_mas_usada().unwrap();
        assert_eq!(tipo_suscripcion, TipoSuscripcion::Super);
    }

    #[test]
    fn test_most_used_metodo_pago() {
        let mut streaming = StreamingRust::new();
        streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(2, "Pedro".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(3, "Pablo".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(4, "Jose".to_string(), TipoSuscripcion::Super, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() });
        streaming.crear_subscripcion(5, "Pepe".to_string(), TipoSuscripcion::Super, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() });
        streaming.cancel_subscripcion(1);
        streaming.cancel_subscripcion(2);
        let metodo_pago = streaming.metodo_pago_mas_usado().unwrap();
        assert_eq!(metodo_pago, MetodoPago::Efectivo);
    }

    #[test]
    fn test_most_popular_subscripcion() {
        let mut streaming = StreamingRust::new();
        streaming.crear_subscripcion(1, "Juan".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(2, "Pedro".to_string(), TipoSuscripcion::Super, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(3, "Pablo".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(4, "Jose".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo);
        streaming.crear_subscripcion(5, "Pepe".to_string(), TipoSuscripcion::Basic, 3, MetodoPago::Efectivo);
        streaming.cancel_subscripcion(4);
        streaming.cancel_subscripcion(5);
        let tipo_suscripcion = streaming.tipo_suscripcion_mas_usado().unwrap();
        assert_eq!(tipo_suscripcion, TipoSuscripcion::Basic);
    }
}