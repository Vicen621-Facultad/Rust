use std::collections::HashMap;
use rand::{thread_rng, Rng};

use crate::practica3::ej3::Fecha;

struct XYZ {
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

struct Transaccion {
    fecha: Fecha,
    tipo: TipoTransaccion,
    dni_usuario: String
}

enum TipoTransaccion {
    IngresoDinero { monto: f64 },
    CompraCripto { monto: f64, criptomoneda: String, cotizacion: f64 },
    VentaCripto { monto: f64, criptomoneda: String, cotizacion: f64 },
    RetiroCripto { monto: f64, criptomoneda: String, cotizacion: f64, blockchain: String, hash: String },
    RecepcionCripto { monto: f64, criptomoneda: String, blockchain: String, cotizacion: f64 },
    RetiroFiat {monto: f64, medio: MedioRetiro},
}

enum MedioRetiro {
    MercadoPago,
    TransferenciaBancaria
}

trait GestorMonedas {
    fn ingresar_dinero(&mut self, dni_usuario: &str, monto: f64);
    fn comprar_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str);
    fn vender_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str);
    fn retirar_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str, blockchain: &str);
    fn recibir_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str, blockchain: &str);
    fn retirar_dinero(&mut self, dni_usuario: &str, monto: f64, medio: MedioRetiro);   
}

trait GestorUsuarios {
    fn crear_usuario(&mut self, nombre: &str, apellido: &str, email: &str, dni: &str, identidad: bool) -> &Usuario;
    fn verificar_identidad(&self, dni_usuario: &str) -> bool;
    fn get_usuario(&self, dni_usuario: &str) -> Option<&Usuario>;
}

trait GestorTransacciones {
    fn crear_transaccion(&mut self, tipo: TipoTransaccion, dni_usuario: &str);
}

trait GestorBalances {
    fn get_balance(&self, dni_usuario: &str, moneda: &str) -> f64;
    fn add_balance(&mut self, dni_usuario: &str, moneda: &str, monto: f64);
    fn remove_balance(&mut self, dni_usuario: &str, moneda: &str, monto: f64);
}

trait Estadisticas {
    fn cripto_mas_ventas(&self) -> String;
    fn cripto_mas_compras(&self) -> String;
    fn cripto_mas_volumen_venta(&self) -> String;
    fn cripto_mas_volumen_compras(&self) -> String;
}

impl GestorMonedas for XYZ {
    fn ingresar_dinero(&mut self, dni_usuario: &str, monto: f64) {
        if self.verificar_identidad(dni_usuario) {
            self.add_balance(dni_usuario, "fiat", monto);
            self.crear_transaccion(TipoTransaccion::IngresoDinero { monto }, dni_usuario);
        }
    }

    fn comprar_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str) {
        if self.verificar_identidad(dni_usuario) && self.get_balance(dni_usuario, "fiat") >= monto {
            let cotizacion = self.get_cotizacion(&criptomoneda);
            self.add_balance(dni_usuario, &criptomoneda, monto / cotizacion);
            self.remove_balance(dni_usuario, "fiat", monto);
            self.crear_transaccion(TipoTransaccion::CompraCripto { monto, criptomoneda: criptomoneda.to_string(), cotizacion }, dni_usuario);
        }
    }

    fn vender_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str) {
        if self.verificar_identidad(dni_usuario) && self.get_balance(dni_usuario, criptomoneda) >= monto {
            let cotizacion: f64 = self.get_cotizacion(&criptomoneda);
            self.add_balance(dni_usuario, "fiat", monto * cotizacion);
            self.remove_balance(dni_usuario, &criptomoneda, monto);
            self.crear_transaccion(TipoTransaccion::VentaCripto { monto, criptomoneda: criptomoneda.to_string(), cotizacion }, dni_usuario);
        }
    }

    fn retirar_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str, blockchain: &str) {
        if self.verificar_identidad(dni_usuario) && self.get_balance(dni_usuario, criptomoneda) >= monto {
            let cotizacion: f64 = self.get_cotizacion(&criptomoneda);
            self.remove_balance(dni_usuario, &criptomoneda, monto);

            //TODO: Llamar a funcion de hash de blockchain
            self.crear_transaccion(TipoTransaccion::RetiroCripto { monto, criptomoneda: criptomoneda.to_string(), cotizacion, blockchain: blockchain.to_string(), hash: "".to_string() }, dni_usuario);
        }
    }

    fn recibir_cripto(&mut self, dni_usuario: &str, monto: f64, criptomoneda: &str, blockchain: &str) {
        if self.verificar_identidad(dni_usuario) {
            let cotizacion: f64 = self.get_cotizacion(&criptomoneda);
            self.add_balance(dni_usuario, &criptomoneda, monto);
            
            self.crear_transaccion(TipoTransaccion::RecepcionCripto { monto, criptomoneda: criptomoneda.to_string(), blockchain: blockchain.to_string(), cotizacion }, dni_usuario);
        }
    }

    fn retirar_dinero(&mut self, dni_usuario: &str, monto: f64, medio: MedioRetiro) {
        if self.verificar_identidad(dni_usuario) && self.get_balance(dni_usuario, "fiat") >= monto {
            self.remove_balance(dni_usuario, "fiat", monto);
            self.crear_transaccion(TipoTransaccion::RetiroFiat { monto, medio }, dni_usuario);
        }
    }
}

impl GestorTransacciones for XYZ {
    fn crear_transaccion(&mut self, tipo: TipoTransaccion, dni_usuario: &str) {
        self.transacciones.push(Transaccion { fecha: Fecha::now(), tipo, dni_usuario: dni_usuario.to_string() });
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

    fn add_balance(&mut self, dni_usuario: &str, moneda: &str, monto: f64) {
        if self.get_usuario(dni_usuario).is_some() {
            let user = self.balances.entry(dni_usuario.to_string()).or_insert(HashMap::new());
            let current = user.get(moneda).unwrap_or(&0.0);
            user.insert(moneda.to_string(), current + monto);
        }
    }

    fn remove_balance(&mut self, dni_usuario: &str, moneda: &str, monto: f64) {
        if self.get_usuario(dni_usuario).is_some() {
            let user = self.balances.entry(dni_usuario.to_string()).or_insert(HashMap::new());
            let current = user.get(moneda).unwrap_or(&0.0);
            user.insert(moneda.to_string(), current - monto);
        }
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
        todo!("Preguntar que significa volumen de ventas");
    }

    fn cripto_mas_volumen_compras(&self) -> String {
        todo!("Preguntar que significa volumen de compras");
    }
}

impl XYZ {
    fn new() -> XYZ {
        XYZ {
            usuarios: Vec::new(),
            cotizaciones: HashMap::new(),
            criptomonedas: Vec::new(),
            transacciones: Vec::new(),
            balances: HashMap::new()
        }
    }

    fn add_cotizacion(&mut self, criptomoneda: String, cotizacion: f64) {
        self.cotizaciones.insert(criptomoneda, cotizacion);
    }

    fn add_criptomoneda(&mut self, criptomoneda: CriptoMoneda) {
        self.criptomonedas.push(criptomoneda);
    }

    fn get_criptomoneda(&self, nombre: &str) -> Option<&CriptoMoneda> {
        self.criptomonedas.iter().find(|c| c.nombre == nombre)
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
    fn test_usuario_set_identidad() {
        let mut usuario = Usuario::new("Jose".to_string(), "Maria".to_string(), "josemaria@test.com".to_string(), "12345678".to_string(), false); 

        assert!(!usuario.get_identidad());
        usuario.set_identidad(true);
        assert!(usuario.get_identidad());
    }

    #[test]
    fn test_add_criptomoneda() {
        let mut sistema = XYZ::new();
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
        let mut sistema = XYZ::new();
        let criptomoneda = CriptoMoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec![BlockChain { nombre: "Bitcoin".to_string(), prefijo: "BTC".to_string() }]
        };

        sistema.add_criptomoneda(criptomoneda);

        let found_criptomoneda = sistema.get_criptomoneda("Bitcoin");

        assert!(found_criptomoneda.is_some());
        assert_eq!(found_criptomoneda.unwrap().nombre, "Bitcoin");
    }

    #[test]
    fn test_ingresar_dinero() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario("Juan", "Garcia", "juan@example.com", "12345678", true);

        sistema.ingresar_dinero("12345678", 100.0);

        assert_eq!(sistema.get_balance("12345678", "fiat"), 100.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_comprar_cripto() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "Juan", "Garcia", "juan@example.com", "87654321", true);

        sistema.add_balance("87654321", "fiat", 50000.0);
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        sistema.comprar_cripto("87654321", 50000.0, "BTC");

        assert_eq!(sistema.get_balance("87654321", "fiat"), 0.0);
        assert_eq!(sistema.get_balance("87654321", "BTC"), 1.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_vender_cripto() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "Alice", "Wonder", "alice@example.com", "11223344", true);

        sistema.add_balance("11223344", "BTC", 0.02);
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        assert_eq!(sistema.get_balance("11223344", "BTC"), 0.02);

        sistema.vender_cripto("11223344", 0.01, "BTC");

        // assert_eq!(sistema.get_balance("11223344", "BTC"), 0.01);
        assert_eq!(sistema.get_balance("11223344", "fiat"), 500.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_retirar_cripto() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "Bob", "Smith", "bob@example.com", "44556677", true);

        sistema.add_balance("44556677", "BTC", 0.02);
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        sistema.retirar_cripto("44556677", 0.01, "BTC", "Bitcoin");

        assert_eq!(sistema.get_balance("44556677", "BTC"), 0.01);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_recibir_cripto() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "Carol", "Danvers", "carol@example.com", "55667788", true);

        sistema.add_cotizacion("ETH".to_string(), 2500.0);
        sistema.recibir_cripto("55667788", 2.0, "ETH", "Ethereum");

        assert_eq!(sistema.get_balance("55667788", "ETH"), 2.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_retirar_dinero() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "David", "Beckham", "david@example.com", "66554433", true);

        sistema.add_balance("66554433", "fiat", 1000.0);

        sistema.retirar_dinero("66554433", 500.0, MedioRetiro::MercadoPago);

        assert_eq!(sistema.get_balance("66554433", "fiat"), 500.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_crear_usuario() {
        let mut sistema = XYZ::new();

        let usuario = sistema.crear_usuario("Eva", "Green", "eva@example.com", "99887766", false);

        assert_eq!(usuario.nombre, "Eva");
        assert_eq!(usuario.apellido, "Green");
        assert_eq!(usuario.email, "eva@example.com");
        assert_eq!(usuario.dni, "99887766");
        assert_eq!(sistema.usuarios.len(), 1);
    }

    #[test]
    fn test_verificar_identidad() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "Frank", "Ocean", "frank@example.com", "77665544", true);

        assert!(sistema.verificar_identidad("77665544"));
    }

    #[test]
    fn test_get_usuario() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario("test", "test", "test@test.com", "55443322", false);

        let found_user = sistema.get_usuario("55443322");

        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().dni, "55443322");
    }

    #[test]
    fn test_get_balance() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "Hank", "Moody", "hank@example.com", "33445566", true);

        sistema.add_balance("33445566", "fiat", 2000.0);

        assert_eq!(sistema.get_balance("33445566", "fiat"), 2000.0);
    }

    #[test]
    fn test_add_balance() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "Hank", "Moody", "hank@example.com", "33445566", true);

        sistema.add_balance("33445566", "fiat", 2000.0);

        assert_eq!(sistema.get_balance("33445566", "fiat"), 2000.0);
    }

    #[test]
    fn test_remove_balance() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "Hank", "Moody", "hank@example.com", "33445566", true);

        sistema.add_balance("33445566", "fiat", 2000.0);
        sistema.remove_balance("33445566", "fiat", 1000.0);

        assert_eq!(sistema.get_balance("33445566", "fiat"), 1000.0);
    }

    #[test]
    fn test_cripto_mas_compras() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "Ivan", "Reitman", "ivan@example.com", "12344321", true);

        sistema.crear_transaccion(TipoTransaccion::CompraCripto { monto: 500.0, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "12344321");
        sistema.crear_transaccion(TipoTransaccion::CompraCripto { monto: 300.0, criptomoneda: "ETH".to_string(), cotizacion: 2500.0 }, "12344321");
        sistema.crear_transaccion(TipoTransaccion::CompraCripto { monto: 200.0, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "12344321");

        assert_eq!(sistema.cripto_mas_compras(), "BTC");
    }

    #[test]
    fn test_cripto_mas_ventas() {
        let mut sistema = XYZ::new();
        sistema.crear_usuario( "Jack", "Nicholson", "jack@example.com", "43211234", true);

        sistema.crear_transaccion(TipoTransaccion::VentaCripto { monto: 0.01, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "43211234");
        sistema.crear_transaccion(TipoTransaccion::VentaCripto { monto: 0.02, criptomoneda: "ETH".to_string(), cotizacion: 2500.0 }, "43211234");
        sistema.crear_transaccion(TipoTransaccion::VentaCripto { monto: 0.01, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, "43211234");

        assert_eq!(sistema.cripto_mas_ventas(), "BTC");
    }

    // Tests for methods not yet implemented
    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_cripto_mas_volumen_venta() {
        let sistema = XYZ::new();
        sistema.cripto_mas_volumen_venta();
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_cripto_mas_volumen_compras() {
        let sistema = XYZ::new();
        sistema.cripto_mas_volumen_compras();
    }
}