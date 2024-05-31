use std::{collections::HashMap, io::{Read, Write}};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::practica3::ej3::Fecha;

struct XYZ {
    file_name: String,
    usuarios: Vec<Usuario>,
    cotizaciones: HashMap<String, f64>,
    criptomonedas: Vec<CriptoMoneda>,
    transacciones: Vec<Transaccion>,
    // HashMap<dni, hashmap<criptomonedas, count>>
    balances: HashMap<String, HashMap<String, f64>>
}

struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
    // balance: HashMap<String, f64>,
    identidad: bool
}

struct CriptoMoneda {
    nombre: String,
    prefijo: String,
    blockchains: Vec<BlockChain>
}

struct BlockChain {
    nombre: String,
    prefijo: String
}

#[derive(Serialize, Deserialize)]
struct Transaccion {
    fecha: Fecha,
    tipo: TipoTransaccion,
    dni_usuario: String
}

#[derive(Serialize, Deserialize)]
enum TipoTransaccion {
    IngresoDinero { monto: f64 },
    CompraCripto { monto: f64, criptomoneda: String, cotizacion: f64 },
    VentaCripto { monto: f64, criptomoneda: String, cotizacion: f64 },
    RetiroCripto { monto: f64, criptomoneda: String, cotizacion: f64, blockchain: String, hash: String },
    RecepcionCripto { monto: f64, criptomoneda: String, blockchain: String, cotizacion: f64 },
    RetiroFiat {monto: f64, medio: MedioRetiro},
}

#[derive(Serialize, Deserialize)]
enum MedioRetiro {
    MercadoPago,
    TransferenciaBancaria
}

trait GestorMonedas {
    fn ingresar_dinero(&mut self, dni_usuario: &str, monto: f64) -> std::io::Result<()>;
    fn comprar_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str) -> std::io::Result<()>;
    fn vender_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str) -> std::io::Result<()>;
    fn retirar_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str, blockchain: &str) -> std::io::Result<()>;
    fn recibir_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str, blockchain: &str) -> std::io::Result<()>;
    fn retirar_dinero(&mut self, dni_usuario: &str, monto: f64, medio: MedioRetiro) -> std::io::Result<()>;   
}

trait GestorUsuarios {
    fn crear_usuario(&mut self, nombre: &str, apellido: &str, email: &str, dni: &str, identidad: bool) -> &Usuario;
    fn verificar_identidad(&self, dni_usuario: &str) -> bool;
    fn get_usuario(&self, dni_usuario: &str) -> Option<&Usuario>;
}

trait GestorTransacciones {
    fn crear_transaccion(&mut self, tipo: TipoTransaccion, dni_usuario: &str) -> std::io::Result<()>;
    fn escribir_archivo_transacciones(&self) -> std::io::Result<()>;
}

trait GestorBalances {
    fn get_balance(&self, dni_usuario: &str, moneda: &str) -> f64;
    fn add_balance(&mut self, dni_usuario: &str, moneda: &str, monto: f64) -> std::io::Result<()>;
    fn remove_balance(&mut self, dni_usuario: &str, moneda: &str, monto: f64) -> std::io::Result<()>;
    fn escribir_archivo_balance(&self) -> std::io::Result<()>;
}

trait Estadisticas {
    fn cripto_mas_ventas(&self) -> String;
    fn cripto_mas_compras(&self) -> String;
    fn cripto_mas_volumen_venta(&self) -> String;
    fn cripto_mas_volumen_compras(&self) -> String;
}

impl GestorMonedas for XYZ {
    fn ingresar_dinero(&mut self, dni_usuario: &str, monto: f64) -> std::io::Result<()>{
        if self.verificar_identidad(dni_usuario) {
            self.add_balance(dni_usuario, "fiat", monto)?;
            self.crear_transaccion(TipoTransaccion::IngresoDinero { monto }, dni_usuario)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Ha ocurrido un error en la transaccion"))
        }
    }

    fn comprar_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str) -> std::io::Result<()>{
        if self.verificar_identidad(dni_usuario) && self.get_balance(dni_usuario, "fiat") >= monto {
            let cotizacion = self.get_cotizacion(&criptomoneda);
            self.add_balance(dni_usuario, &criptomoneda, monto / cotizacion)?;
            self.remove_balance(dni_usuario, "fiat", monto)?;
            self.crear_transaccion(TipoTransaccion::CompraCripto { monto, criptomoneda: criptomoneda.to_string(), cotizacion }, dni_usuario)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Ha ocurrido un error en la transaccion"))
        }
    }

    fn vender_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str) -> std::io::Result<()>{
        if self.verificar_identidad(dni_usuario) && self.get_balance(dni_usuario, criptomoneda) >= monto {
            let cotizacion: f64 = self.get_cotizacion(&criptomoneda);
            self.add_balance(dni_usuario, "fiat", monto * cotizacion)?;
            self.remove_balance(dni_usuario, &criptomoneda, monto)?;
            self.crear_transaccion(TipoTransaccion::VentaCripto { monto, criptomoneda: criptomoneda.to_string(), cotizacion }, dni_usuario)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Ha ocurrido un error en la transaccion"))
        }
    }

    fn retirar_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str, blockchain: &str) -> std::io::Result<()>{
        if self.verificar_identidad(dni_usuario) && self.get_balance(dni_usuario, criptomoneda) >= monto {
            let cotizacion: f64 = self.get_cotizacion(&criptomoneda);
            self.remove_balance(dni_usuario, &criptomoneda, monto)?;

            let cripto = self.get_criptomoneda(criptomoneda).unwrap();
            let hash = cripto.blockchains.iter()
                .find(|b| b.nombre == blockchain)
                .unwrap().withdraw();

            self.crear_transaccion(TipoTransaccion::RetiroCripto { monto, criptomoneda: criptomoneda.to_string(), cotizacion, blockchain: blockchain.to_string(), hash }, dni_usuario)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Ha ocurrido un error en la transaccion"))
        }
    }

    fn recibir_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str, blockchain: &str) -> std::io::Result<()>{
        if self.verificar_identidad(dni_usuario) {
            let cotizacion: f64 = self.get_cotizacion(&criptomoneda);
            self.add_balance(dni_usuario, &criptomoneda, monto)?;
            self.crear_transaccion(TipoTransaccion::RecepcionCripto { monto, criptomoneda: criptomoneda.to_string(), blockchain: blockchain.to_string(), cotizacion }, dni_usuario)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Ha ocurrido un error en la transaccion"))
        }
    }

    fn retirar_dinero(&mut self, dni_usuario: &str, monto: f64, medio: MedioRetiro) -> std::io::Result<()>{
        if self.verificar_identidad(dni_usuario) && self.get_balance(dni_usuario, "fiat") >= monto {
            self.remove_balance(dni_usuario, "fiat", monto)?;
            self.crear_transaccion(TipoTransaccion::RetiroFiat { monto, medio }, dni_usuario)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Ha ocurrido un error en la transaccion"))
        
        }
    }
}

impl GestorTransacciones for XYZ {
    fn crear_transaccion(&mut self, tipo: TipoTransaccion, dni_usuario: &str) -> std::io::Result<()>{
        self.transacciones.push(Transaccion { fecha: Fecha::now(), tipo, dni_usuario: dni_usuario.to_string() });
        self.escribir_archivo_transacciones()
    }

    fn escribir_archivo_transacciones(&self) -> std::io::Result<()> {
        let mut file = std::fs::File::create("test/".to_owned() + &self.file_name + "_transacciones.json")?;
        let serialized = serde_json::to_string(&self.transacciones)?;
        file.write_all(&serialized.as_bytes())?;
        Ok(())
    }
}

impl GestorUsuarios for XYZ {
    fn crear_usuario(&mut self, nombre: &str, apellido: &str, email: &str, dni: &str, identidad: bool) -> &Usuario {
        let usuario = Usuario::new(nombre.to_string(), apellido.to_string(), email.to_string(), dni.to_string(), identidad);
        self.usuarios.push(usuario);
        self.balances.insert(dni.to_string(), HashMap::new());
        self.usuarios.last().unwrap()
    }

    fn verificar_identidad(&self, dni_usuario: &str) -> bool {
        let usuario = self.get_usuario(dni_usuario);
        match usuario {
            Some(usuario) => usuario.get_identidad(),
            None => false
        }
    }

    fn get_usuario(&self, dni_usuario: &str) -> Option<&Usuario> {
        self.usuarios.iter().find(|u| u.get_dni() == dni_usuario)
    }
}

impl GestorBalances for XYZ {
    fn get_balance(&self, dni_usuario: &str, moneda: &str) -> f64 {
        if self.get_usuario(dni_usuario).is_some() {
            self.balances.get(dni_usuario).unwrap().get(moneda).unwrap_or(&0.0).clone()
        } else {
            0.0
        }
    }

    fn add_balance(&mut self, dni_usuario: &str, moneda: &str, monto: f64) -> std::io::Result<()> {
        match self.get_usuario(dni_usuario) {
            Some(_) => {
                let user = self.balances.entry(dni_usuario.to_string()).or_insert(HashMap::new());
                let current = user.get(moneda).unwrap_or(&0.0);
                user.insert(moneda.to_string(), current + monto);
                self.escribir_archivo_balance()
            },
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Usuario no encontrado"))
        }
    }

    fn remove_balance(&mut self, dni_usuario: &str, moneda: &str, monto: f64) -> std::io::Result<()>{
        match self.get_usuario(dni_usuario) {
            Some(_) => {
                let user = self.balances.entry(dni_usuario.to_string()).or_insert(HashMap::new());
                let current = user.get(moneda).unwrap_or(&0.0);
                user.insert(moneda.to_string(), current - monto);
                self.escribir_archivo_balance()
            },
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Usuario no encontrado"))
        }
    }

    fn escribir_archivo_balance(&self) -> std::io::Result<()> {
        let mut file = std::fs::File::create("test/".to_owned() + &self.file_name + "_balances.json")?;
        let serialized = serde_json::to_string(&self.balances)?;
        file.write_all(&serialized.as_bytes())?;
        Ok(())
    }
}

impl Estadisticas for XYZ {
    fn cripto_mas_compras(&self) -> String {
        self.transacciones.iter().filter(|t| match t.tipo {
            TipoTransaccion::CompraCripto { .. } => true,
            _ => false
        }).fold(HashMap::new(), |mut acc, t| {
            let criptomoneda = match &t.tipo {
                TipoTransaccion::CompraCripto { criptomoneda, .. } => criptomoneda,
                _ => ""
            };
            let entry = acc.entry(criptomoneda).or_insert(0);
            *entry += 1;
            acc
        }).iter().max_by_key(|(_, &v)| v).map(|(k, _)| k.to_string()).unwrap_or("".to_string())
    }

    fn cripto_mas_ventas(&self) -> String {
        self.transacciones.iter().filter(|t| match t.tipo {
            TipoTransaccion::VentaCripto { .. } => true,
            _ => false
        }).fold(HashMap::new(), |mut acc, t| {
            let criptomoneda = match &t.tipo {
                TipoTransaccion::VentaCripto { criptomoneda, .. } => criptomoneda,
                _ => ""
            };
            let entry = acc.entry(criptomoneda).or_insert(0);
            *entry += 1;
            acc
        }).iter().max_by_key(|(_, &v)| v).map(|(k, _)| k.to_string()).unwrap_or("".to_string())
    }

    fn cripto_mas_volumen_venta(&self) -> String {
        self.transacciones.iter().filter(|t| match t.tipo {
            TipoTransaccion::VentaCripto { .. } => true,
            _ => false
        }).fold(HashMap::new(), |mut acc, t| {
            let criptomoneda = match &t.tipo {
                TipoTransaccion::VentaCripto { criptomoneda, .. } => criptomoneda,
                _ => ""
            };
            let entry = acc.entry(criptomoneda).or_insert(0.0);
            let monto = match t.tipo {
                TipoTransaccion::VentaCripto { monto, .. } => monto,
                _ => 0.0
            };
            *entry += monto;
            acc
        }).iter()
        .max_by(|(_, &v1), (_, &v2)| v1.partial_cmp(&v2)
        .unwrap_or(std::cmp::Ordering::Equal))
        .map(|(k, _)| k.to_string())
        .unwrap_or("".to_string())
    }

    fn cripto_mas_volumen_compras(&self) -> String {
        self.transacciones.iter().filter(|t| match t.tipo {
            TipoTransaccion::CompraCripto { .. } => true,
            _ => false
        }).fold(HashMap::new(), |mut acc, t| {
            let criptomoneda = match &t.tipo {
                TipoTransaccion::CompraCripto { criptomoneda, .. } => criptomoneda,
                _ => ""
            };
            let entry = acc.entry(criptomoneda).or_insert(0.0);
            let monto = match t.tipo {
                TipoTransaccion::CompraCripto { monto, .. } => monto,
                _ => 0.0
            };
            *entry += monto;
            acc
        }).iter()
        .max_by(|(_, &v1), (_, &v2)| v1.partial_cmp(&v2)
        .unwrap_or(std::cmp::Ordering::Equal))
        .map(|(k, _)| k.to_string())
        .unwrap_or("".to_string())
    }
}

impl XYZ {
    fn new(file_name: &str) -> XYZ {
        let balances = match std::fs::File::open("test/".to_owned() + file_name + "_balances.json") {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                let suscripciones: HashMap<String, HashMap<String, f64>> = serde_json::from_str(&buf).unwrap();
                suscripciones
            },
            Err(_) => HashMap::new()
        };

        let transacciones = match std::fs::File::open("test/".to_owned() + file_name + "_transacciones.json") {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                let transacciones: Vec<Transaccion> = serde_json::from_str(&buf).unwrap();
                transacciones
            },
            Err(_) => Vec::new()
        };

        XYZ {
            file_name: file_name.to_string(),
            usuarios: Vec::new(),
            cotizaciones: HashMap::new(),
            criptomonedas: Vec::new(),
            transacciones,
            balances
        }
    }

    fn add_cotizacion(&mut self, criptomoneda: String, cotizacion: f64) {
        self.cotizaciones.insert(criptomoneda, cotizacion);
    }

    fn add_criptomoneda(&mut self, criptomoneda: CriptoMoneda) {
        self.criptomonedas.push(criptomoneda);
    }

    fn get_criptomoneda(&self, prefijo: &str) -> Option<&CriptoMoneda> {
        self.criptomonedas.iter().find(|c| c.prefijo == prefijo)
    }

    fn get_cotizacion(&self, criptomoneda: &str) -> f64 {
        *self.cotizaciones.get(criptomoneda).unwrap_or(&0.0)
    }
}

impl Usuario {
    fn new(nombre: String, apellido: String, email: String, dni: String, identidad: bool) -> Usuario {
        Usuario {
            nombre,
            apellido,
            email,
            dni,
            identidad
        }
    }

    fn get_dni(&self) -> &str {
        &self.dni
    }

    fn set_identidad(&mut self, identidad: bool) {
        self.identidad = identidad;
    }

    fn get_identidad(&self) -> bool {
        self.identidad
    }
}

impl BlockChain {
    fn withdraw(&self) -> String {
        self.nombre.clone() + thread_rng().gen_range(0..100).to_string().as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_withdraw() {
        let blockchain = BlockChain { nombre: "Bitcoin".to_string(), prefijo: "BTC".to_string() };
        let hash = blockchain.withdraw();

        assert!(hash.starts_with("Bitcoin"));
    }

    #[test]
    fn test_new_xyz_con_datos() {
        let mut sistema = XYZ::new("test_new_XYZ_con_datos");

        assert_eq!(sistema.balances.len(), 0);
        assert_eq!(sistema.transacciones.len(), 0);

        sistema.crear_usuario( "Juan", "Garcia", "juan@example.com", "87654321", true);
        assert!(sistema.add_balance("87654321", "fiat", 50000.0).is_ok());
        assert!(sistema.ingresar_dinero("87654321", 100.0).is_ok());
        assert_eq!(sistema.balances.len(), 1);
        assert_eq!(sistema.transacciones.len(), 1);

        let sistema = XYZ::new("test_new_XYZ_con_datos");
        assert_eq!(sistema.balances.len(), 1);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_usuario_set_identidad() {
        let mut usuario = Usuario::new("Jose".to_string(), "Maria".to_string(), "josemaria@test.com".to_string(), "12345678".to_string(), false); 

        assert!(!usuario.get_identidad());
        usuario.set_identidad(true);
        assert!(usuario.get_identidad());
    }

    #[test]
    fn test_add_criptomoneda() {
        let mut sistema = XYZ::new("test_add_criptomoneda");
        let criptomoneda = CriptoMoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec![BlockChain { nombre: "Bitcoin".to_string(), prefijo: "BTC".to_string() }]
        };

        sistema.add_criptomoneda(criptomoneda);

        assert_eq!(sistema.criptomonedas.len(), 1);
    }

    #[test]
    fn test_get_criptomoneda() {
        let mut sistema = XYZ::new("test_get_criptomoneda");
        let criptomoneda = CriptoMoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec![BlockChain { nombre: "Bitcoin".to_string(), prefijo: "BTC".to_string() }]
        };

        sistema.add_criptomoneda(criptomoneda);

        let found_criptomoneda = sistema.get_criptomoneda("BTC");

        assert!(found_criptomoneda.is_some());
        assert_eq!(found_criptomoneda.unwrap().nombre, "Bitcoin");
    }

    #[test]
    fn test_ingresar_dinero() {
        let mut sistema = XYZ::new("test_ingresar_dinero");
        sistema.crear_usuario("Juan", "Garcia", "juan@example.com", "12345678", true);

        assert!(sistema.ingresar_dinero("12345678", 100.0).is_ok());

        assert_eq!(sistema.get_balance("12345678", "fiat"), 100.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_ingresar_dinero_sin_identidad() {
        let mut sistema = XYZ::new("test_ingresar_dinero_sin_identidad");
        sistema.crear_usuario("Juan", "Garcia", "juan@example.com", "12345678", false);

        assert!(sistema.ingresar_dinero("12345678", 100.0).is_err());
        assert_eq!(sistema.transacciones.len(), 0);
    }

    #[test]
    fn test_comprar_cripto() {
        let mut sistema = XYZ::new("test_comprar_cripto");
        sistema.crear_usuario( "Juan", "Garcia", "juan@example.com", "87654321", true);

        assert!(sistema.add_balance("87654321", "fiat", 50000.0).is_ok());
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        assert!(sistema.comprar_cripto("87654321", 50000.0, "BTC").is_ok());

        assert_eq!(sistema.get_balance("87654321", "fiat"), 0.0);
        assert_eq!(sistema.get_balance("87654321", "BTC"), 1.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_comprar_cripto_sin_identidad() {
        let mut sistema = XYZ::new("test_comprar_cripto_sin_identidad");
        sistema.crear_usuario( "Juan", "Garcia", "juan@example.com", "87654321", false);

        assert!(sistema.add_balance("87654321", "fiat", 50000.0).is_ok());
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        assert!(sistema.comprar_cripto("87654321", 50000.0, "BTC").is_err());

        assert_eq!(sistema.get_balance("87654321", "fiat"), 50000.0);
        assert_eq!(sistema.transacciones.len(), 0);
    }

    #[test]
    fn test_vender_cripto() {
        let mut sistema = XYZ::new("test_vender_cripto");
        sistema.crear_usuario( "Alice", "Wonder", "alice@example.com", "11223344", true);

        assert!(sistema.add_balance("11223344", "BTC", 0.02).is_ok());
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        assert_eq!(sistema.get_balance("11223344", "BTC"), 0.02);

        assert!(sistema.vender_cripto("11223344", 0.01, "BTC").is_ok());

        assert_eq!(sistema.get_balance("11223344", "BTC"), 0.01);
        assert_eq!(sistema.get_balance("11223344", "fiat"), 500.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_vender_cripto_sin_identidad() {
        let mut sistema = XYZ::new("test_vender_cripto_sin_identidad");
        sistema.crear_usuario( "Alice", "Wonder", "alice@example.com", "11223344", false);

        assert!(sistema.add_balance("11223344", "BTC", 0.02).is_ok());
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        assert_eq!(sistema.get_balance("11223344", "BTC"), 0.02);

        assert!(sistema.vender_cripto("11223344", 0.01, "BTC").is_err());

        assert_eq!(sistema.get_balance("11223344", "BTC"), 0.02);
        assert_eq!(sistema.transacciones.len(), 0);
    }

    #[test]
    fn test_retirar_cripto() {
        let mut sistema = XYZ::new("test_retirar_cripto");
        sistema.crear_usuario( "Bob", "Smith", "bob@example.com", "44556677", true);

        assert!(sistema.add_balance("44556677", "BTC", 0.02).is_ok());
        sistema.add_criptomoneda(CriptoMoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec![BlockChain { nombre: "Bitcoin".to_string(), prefijo: "BTC".to_string() }]
        });
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        assert!(sistema.retirar_cripto("44556677", 0.01, "BTC", "Bitcoin").is_ok());
        assert_eq!(sistema.get_balance("44556677", "BTC"), 0.01);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_retirar_cripto_sin_identidad() {
        let mut sistema = XYZ::new("test_retirar_cripto_sin_identidad");
        sistema.crear_usuario( "Bob", "Smith", "bob@example.com", "44556677", false);

        assert!(sistema.add_balance("44556677", "BTC", 0.02).is_ok());
        sistema.add_criptomoneda(CriptoMoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec![BlockChain { nombre: "Bitcoin".to_string(), prefijo: "BTC".to_string() }]
        });
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        assert!(sistema.retirar_cripto("44556677", 0.01, "BTC", "Bitcoin").is_err());
        assert_eq!(sistema.get_balance("44556677", "BTC"), 0.02);
        assert_eq!(sistema.transacciones.len(), 0);
    }

    #[test]
    fn test_recibir_cripto() {
        let mut sistema = XYZ::new("test_recibir_cripto");
        sistema.crear_usuario( "Carol", "Danvers", "carol@example.com", "55667788", true);

        sistema.add_cotizacion("ETH".to_string(), 2500.0);
        assert!(sistema.recibir_cripto("55667788", 2.0, "ETH", "Ethereum").is_ok());

        assert_eq!(sistema.get_balance("55667788", "ETH"), 2.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_recibir_cripto_sin_identidad() {
        let mut sistema = XYZ::new("test_recibir_cripto_sin_identidad");
        sistema.crear_usuario( "Carol", "Danvers", "carol@example.com", "55667788", false);

        sistema.add_cotizacion("ETH".to_string(), 2500.0);
        assert!(sistema.recibir_cripto("55667788", 2.0, "ETH", "Ethereum").is_err());

        assert_eq!(sistema.transacciones.len(), 0);
    }

    #[test]
    fn test_retirar_dinero() {
        let mut sistema = XYZ::new("test_retirar_dinero");
        sistema.crear_usuario( "David", "Beckham", "david@example.com", "66554433", true);

        assert!(sistema.add_balance("66554433", "fiat", 1000.0).is_ok());

        assert!(sistema.retirar_dinero("66554433", 500.0, MedioRetiro::MercadoPago).is_ok());

        assert_eq!(sistema.get_balance("66554433", "fiat"), 500.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_retirar_dinero_sin_identidad() {
        let mut sistema = XYZ::new("test_retirar_dinero_sin_identidad");
        sistema.crear_usuario( "David", "Beckham", "david@example.com", "66554433", false);

        assert!(sistema.add_balance("66554433", "fiat", 1000.0).is_ok());

        assert!(sistema.retirar_dinero("66554433", 500.0, MedioRetiro::MercadoPago).is_err());

        assert_eq!(sistema.get_balance("66554433", "fiat"), 1000.0);
        assert_eq!(sistema.transacciones.len(), 0);
    }

    #[test]
    fn test_crear_usuario() {
        let mut sistema = XYZ::new("test_crear_usuario");

        let usuario = sistema.crear_usuario("Eva", "Green", "eva@example.com", "99887766", false);

        assert_eq!(usuario.nombre, "Eva");
        assert_eq!(usuario.apellido, "Green");
        assert_eq!(usuario.email, "eva@example.com");
        assert_eq!(usuario.dni, "99887766");
        assert_eq!(sistema.usuarios.len(), 1);
    }

    #[test]
    fn test_verificar_identidad() {
        let mut sistema = XYZ::new("test_verificar_identidad");
        sistema.crear_usuario( "Frank", "Ocean", "frank@example.com", "77665544", true);

        assert!(sistema.verificar_identidad("77665544"));
    }

    #[test]
    fn test_get_usuario() {
        let mut sistema = XYZ::new("test_get_usuario");
        sistema.crear_usuario("test", "test", "test@test.com", "55443322", false);

        let found_user = sistema.get_usuario("55443322");

        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().dni, "55443322");
    }

    #[test]
    fn test_get_balance() {
        let mut sistema = XYZ::new("test_get_balance");
        sistema.crear_usuario( "Hank", "Moody", "hank@example.com", "33445566", true);

        assert!(sistema.add_balance("33445566", "fiat", 2000.0).is_ok());

        assert_eq!(sistema.get_balance("33445566", "fiat"), 2000.0);
    }

    #[test]
    fn test_add_balance() {
        let mut sistema = XYZ::new("test_add_balance");
        sistema.crear_usuario( "Hank", "Moody", "hank@example.com", "33445566", true);

        assert!(sistema.add_balance("33445566", "fiat", 2000.0).is_ok());

        assert_eq!(sistema.get_balance("33445566", "fiat"), 2000.0);
    }

    #[test]
    fn test_remove_balance() {
        let mut sistema = XYZ::new("test_remove_balance");
        sistema.crear_usuario( "Hank", "Moody", "hank@example.com", "33445566", true);

        assert!(sistema.add_balance("33445566", "fiat", 2000.0).is_ok());
        assert!(sistema.remove_balance("33445566", "fiat", 1000.0).is_ok());

        assert_eq!(sistema.get_balance("33445566", "fiat"), 1000.0);
    }

    #[test]
    fn test_cripto_mas_compras() {
        let mut sistema = XYZ::new("test_cripto_mas_compras");
        sistema.crear_usuario( "Ivan", "Reitman", "ivan@example.com", "12344321", true);

        assert!(sistema.crear_transaccion(TipoTransaccion::CompraCripto { monto: 500.0, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "12344321").is_ok());
        assert!(sistema.crear_transaccion(TipoTransaccion::CompraCripto { monto: 300.0, criptomoneda: "ETH".to_string(), cotizacion: 2500.0 }, "12344321").is_ok());
        assert!(sistema.crear_transaccion(TipoTransaccion::CompraCripto { monto: 200.0, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "12344321").is_ok());

        assert_eq!(sistema.cripto_mas_compras(), "BTC");
    }

    #[test]
    fn test_cripto_mas_ventas() {
        let mut sistema = XYZ::new("test_cripto_mas_ventas");
        sistema.crear_usuario( "Jack", "Nicholson", "jack@example.com", "43211234", true);

        assert!(sistema.crear_transaccion(TipoTransaccion::VentaCripto { monto: 0.01, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "43211234").is_ok());
        assert!(sistema.crear_transaccion(TipoTransaccion::VentaCripto { monto: 0.02, criptomoneda: "ETH".to_string(), cotizacion: 2500.0 }, "43211234").is_ok());
        assert!(sistema.crear_transaccion(TipoTransaccion::VentaCripto { monto: 0.01, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "43211234").is_ok());

        assert_eq!(sistema.cripto_mas_ventas(), "BTC");
    }

    #[test]
    fn test_cripto_mas_volumen_venta() {
        let mut sistema = XYZ::new("test_cripto_mas_volumen_venta");
        sistema.crear_usuario( "Jack", "Nicholson", "jack@example.com", "43211234", true);

        assert!(sistema.crear_transaccion(TipoTransaccion::VentaCripto { monto: 0.01, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "43211234").is_ok());
        assert!(sistema.crear_transaccion(TipoTransaccion::VentaCripto { monto: 0.02, criptomoneda: "ETH".to_string(), cotizacion: 2500.0 }, "43211234").is_ok());
        assert!(sistema.crear_transaccion( TipoTransaccion::VentaCripto { monto: 0.03, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "43211234").is_ok());

        assert_eq!(sistema.cripto_mas_volumen_venta(), "BTC");
    }

    #[test]
    fn test_cripto_mas_volumen_compras() {
        let mut sistema = XYZ::new("test_cripto_mas_volumen_compras");
        sistema.crear_usuario( "Jack", "Nicholson", "jack@example.com", "43211234", true);

        assert!(sistema.crear_transaccion(TipoTransaccion::CompraCripto { monto: 500.0, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "43211234").is_ok());
        assert!(sistema.crear_transaccion(TipoTransaccion::CompraCripto { monto: 300.0, criptomoneda: "ETH".to_string(), cotizacion: 2500.0 }, "43211234").is_ok());
        assert!(sistema.crear_transaccion(TipoTransaccion::CompraCripto { monto: 700.0, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "43211234").is_ok());

        assert_eq!(sistema.cripto_mas_volumen_compras(), "BTC");
    }
}