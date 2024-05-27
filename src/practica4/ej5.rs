/*use std::collections::HashMap;
use crate::practica3::ej3::Fecha;

struct XYZ<'a> {
    usuarios: Vec<Usuario>,
    cotizaciones: HashMap<String, f64>,
    criptomonedas: Vec<CriptoMoneda>,
    transacciones: Vec<Transaccion<'a>>,
}

struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
    balance: HashMap<String, f64>,
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

struct Transaccion<'a> {
    fecha: Fecha,
    tipo: TipoTransaccion,
    usuario: &'a Usuario
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

trait GestorMonedas<'a> {
    fn ingresar_dinero(&mut self, usuario: &'a mut Usuario, monto: f64);
    fn comprar_cripto(&mut self, usuario: &'a mut Usuario, monto: f64, criptomoneda: String);
    fn vender_cripto(&mut self, usuario: &'a mut Usuario, monto: f64, criptomoneda: String);
    fn retirar_cripto(&mut self, usuario: &'a mut Usuario, monto: f64, criptomoneda: String, blockchain: String);
    fn recibir_cripto(&mut self, usuario: &'a mut Usuario, monto: f64, criptomoneda: String, blockchain: String);
    fn retirar_dinero(&mut self, usuario: &'a mut Usuario, monto: f64, medio: MedioRetiro);   
}

trait GestorUsuarios {
    fn crear_usuario(&mut self, nombre: String, apellido: String, email: String, dni: String) -> &Usuario;
    fn verificar_identidad(&self, usuario: &Usuario) -> bool;
    fn get_usuario(&self, dni: &str) -> Option<&Usuario>;
    fn get_balance_fiat(&self, usuario: &Usuario) -> f64;
}

trait GestorTransacciones<'a> {
    fn crear_transaccion(&mut self, fecha: Fecha, tipo: TipoTransaccion, usuario: &'a Usuario);
}

trait Estadisticas {
    fn cripto_mas_ventas(&self) -> String;
    fn cripto_mas_compras(&self) -> String;
    fn cripto_mas_volumen_venta(&self) -> String;
    fn cripto_mas_volumen_compras(&self) -> String;
}

impl<'a> GestorMonedas<'a> for XYZ<'a> {
    fn ingresar_dinero(&mut self, usuario: &'a mut Usuario, monto: f64) {
        if self.verificar_identidad(usuario) {
            usuario.add_balance("fiat", monto);
            self.crear_transaccion(Fecha::now(), TipoTransaccion::IngresoDinero { monto }, usuario);
        }
    }

    fn comprar_cripto(&mut self, usuario: &'a mut Usuario, monto: f64, criptomoneda: String) {
        if self.verificar_identidad(usuario) && self.get_balance_fiat(usuario) >= monto {
            let cotizacion = self.get_cotizacion(&criptomoneda);
            usuario.add_balance(&criptomoneda, monto / cotizacion);
            usuario.remove_balance("fiat", monto);
            self.crear_transaccion(Fecha::now(), TipoTransaccion::CompraCripto { monto, criptomoneda, cotizacion }, usuario);
        }
    }

    fn vender_cripto(&mut self, usuario: &'a mut Usuario, monto: f64, criptomoneda: String) {
        if self.verificar_identidad(usuario) && usuario.get_balance(&criptomoneda) >= monto {
            let cotizacion: f64 = self.get_cotizacion(&criptomoneda);
            usuario.add_balance("fiat", monto * cotizacion);
            usuario.remove_balance(&criptomoneda, monto);
            self.crear_transaccion(Fecha::now(), TipoTransaccion::VentaCripto { monto, criptomoneda, cotizacion }, usuario);
        }
    }

    fn retirar_cripto(&mut self, usuario: &'a mut Usuario, monto: f64, criptomoneda: String, blockchain: String) {
        if self.verificar_identidad(usuario) && usuario.get_balance(&criptomoneda) >= monto {
            let cotizacion: f64 = self.get_cotizacion(&criptomoneda);
            usuario.remove_balance(&criptomoneda, monto);
            self.crear_transaccion(Fecha::now(), TipoTransaccion::RetiroCripto { monto, criptomoneda, cotizacion, blockchain, hash: "".to_string() }, usuario);
        }
    }

    fn recibir_cripto(&mut self, usuario: &'a mut Usuario, monto: f64, criptomoneda: String, blockchain: String) {
        if self.verificar_identidad(usuario) {
            let cotizacion: f64 = self.get_cotizacion(&criptomoneda);
            usuario.add_balance(&criptomoneda, monto);
            self.crear_transaccion(Fecha::now(), TipoTransaccion::RecepcionCripto { monto, criptomoneda, blockchain, cotizacion }, usuario);
        }
    }

    fn retirar_dinero(&mut self, usuario: &'a mut Usuario, monto: f64, medio: MedioRetiro) {
        if self.verificar_identidad(usuario) && usuario.get_balance("fiat") >= monto {
            usuario.remove_balance("fiat", monto);
            self.crear_transaccion(Fecha::now(), TipoTransaccion::RetiroFiat { monto, medio }, usuario);
        }
    }
}

impl<'a> GestorTransacciones<'a> for XYZ<'a> {
    fn crear_transaccion(&mut self, fecha: Fecha, tipo: TipoTransaccion, usuario: &'a Usuario) {
        self.transacciones.push(Transaccion { fecha, tipo, usuario });
    }
}

impl<'a> GestorUsuarios for XYZ<'a> {
    fn crear_usuario(&mut self, nombre: String, apellido: String, email: String, dni: String) -> &Usuario {
        self.usuarios.push(Usuario::new(nombre, apellido, email, dni));
        self.usuarios.last().unwrap()
    }

    fn verificar_identidad(&self, usuario: &Usuario) -> bool {
        usuario.get_identidad()
    }

    fn get_usuario(&self, dni: &str) -> Option<&Usuario> {
        self.usuarios.iter().find(|u| u.get_dni() == dni)
    }

    fn get_balance_fiat(&self, usuario: &Usuario) -> f64 {
        usuario.get_balance("fiat")
    }
}

impl<'a> Estadisticas for XYZ<'a> {
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

impl<'a> XYZ<'a> {
    fn new() -> XYZ<'a> {
        XYZ {
            usuarios: Vec::new(),
            cotizaciones: HashMap::new(),
            criptomonedas: Vec::new(),
            transacciones: Vec::new()
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
    fn new(nombre: String, apellido: String, email: String, dni: String) -> Usuario {
        Usuario {
            nombre,
            apellido,
            email,
            dni,
            balance: HashMap::new(),
            identidad: false
        }
    }

    fn add_balance(&mut self, moneda: &str, monto: f64) {
        let entry = self.balance.entry(moneda.to_string()).or_insert(0.0);
        *entry += monto;
    }

    fn remove_balance(&mut self, moneda: &str, monto: f64) {
        let entry = self.balance.entry(moneda.to_string()).or_insert(0.0);
        *entry -= monto;
    }

    fn get_dni(&self) -> &str {
        &self.dni
    }

    fn get_balance(&self, moneda: &str) -> f64 {
        *self.balance.get(moneda).unwrap_or(&0.0)
    }

    fn set_identidad(&mut self, identidad: bool) {
        self.identidad = identidad;
    }

    fn get_identidad(&self) -> bool {
        self.identidad
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a default user with identity verified
    fn create_user_with_identity(nombre: &str, apellido: &str, email: &str, dni: &str) -> Usuario {
        let mut user = Usuario::new(nombre.to_string(), apellido.to_string(), email.to_string(), dni.to_string());
        user.set_identidad(true);
        user
    }

    #[test]
    fn test_ingresar_dinero() {
        let mut sistema = XYZ::new();
        let mut usuario = create_user_with_identity("John", "Doe", "john@example.com", "12345678");

        sistema.ingresar_dinero(&mut usuario, 100.0);

        assert_eq!(usuario.get_balance("fiat"), 100.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_comprar_cripto() {
        let mut sistema = XYZ::new();
        let mut usuario = create_user_with_identity("Jane", "Doe", "jane@example.com", "87654321");

        usuario.add_balance("fiat", 1000.0);
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        sistema.comprar_cripto(&mut usuario, 500.0, "BTC".to_string());

        assert_eq!(usuario.get_balance("fiat"), 500.0);
        assert_eq!(usuario.get_balance("BTC"), 0.01);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_vender_cripto() {
        let mut sistema = XYZ::new();
        let mut usuario = create_user_with_identity("Alice", "Wonder", "alice@example.com", "11223344");

        usuario.add_balance("BTC", 0.02);
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        sistema.vender_cripto(&mut usuario, 0.01, "BTC".to_string());

        assert_eq!(usuario.get_balance("BTC"), 0.01);
        assert_eq!(usuario.get_balance("fiat"), 500.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_retirar_cripto() {
        let mut sistema = XYZ::new();
        let mut usuario = create_user_with_identity("Bob", "Smith", "bob@example.com", "44556677");

        usuario.add_balance("BTC", 0.02);
        sistema.add_cotizacion("BTC".to_string(), 50000.0);

        sistema.retirar_cripto(&mut usuario, 0.01, "BTC".to_string(), "Bitcoin".to_string());

        assert_eq!(usuario.get_balance("BTC"), 0.01);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_recibir_cripto() {
        let mut sistema = XYZ::new();
        let mut usuario = create_user_with_identity("Carol", "Danvers", "carol@example.com", "55667788");

        sistema.add_cotizacion("ETH".to_string(), 2500.0);
        let eth = usuario.get_balance("ETH");
        sistema.recibir_cripto(&mut usuario, 2.0, "ETH".to_string(), "Ethereum".to_string());

        assert_eq!(eth, 2.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_retirar_dinero() {
        let mut sistema = XYZ::new();
        let mut usuario = create_user_with_identity("David", "Beckham", "david@example.com", "66554433");

        usuario.add_balance("fiat", 1000.0);

        sistema.retirar_dinero(&mut usuario, 500.0, MedioRetiro::MercadoPago);

        assert_eq!(usuario.get_balance("fiat"), 500.0);
        assert_eq!(sistema.transacciones.len(), 1);
    }

    #[test]
    fn test_crear_usuario() {
        let mut sistema = XYZ::new();

        let usuario = sistema.crear_usuario("Eva".to_string(), "Green".to_string(), "eva@example.com".to_string(), "99887766".to_string());

        assert_eq!(usuario.nombre, "Eva");
        assert_eq!(usuario.apellido, "Green");
        assert_eq!(usuario.email, "eva@example.com");
        assert_eq!(usuario.dni, "99887766");
        assert_eq!(sistema.usuarios.len(), 1);
    }

    #[test]
    fn test_verificar_identidad() {
        let sistema = XYZ::new();
        let usuario = create_user_with_identity("Frank", "Ocean", "frank@example.com", "77665544");

        assert!(sistema.verificar_identidad(&usuario));
    }

    #[test]
    fn test_get_usuario() {
        let mut sistema = XYZ::new();
        let usuario = sistema.crear_usuario("test".to_string(), "test".to_string(), "test@test.com".to_string(), "55443322".to_string());

        let found_user = sistema.get_usuario("55443322");

        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().dni, usuario.dni);
    }

    #[test]
    fn test_get_balance_fiat() {
        let sistema = XYZ::new();
        let mut usuario = create_user_with_identity("Hank", "Moody", "hank@example.com", "33445566");

        usuario.add_balance("fiat", 2000.0);

        assert_eq!(sistema.get_balance_fiat(&usuario), 2000.0);
    }

    #[test]
    fn test_cripto_mas_compras() {
        let mut sistema = XYZ::new();
        let usuario = create_user_with_identity("Ivan", "Reitman", "ivan@example.com", "12344321");

        sistema.crear_transaccion(Fecha::now(), TipoTransaccion::CompraCripto { monto: 500.0, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, &usuario);
        sistema.crear_transaccion(Fecha::now(), TipoTransaccion::CompraCripto { monto: 300.0, criptomoneda: "ETH".to_string(), cotizacion: 2500.0 }, &usuario);
        sistema.crear_transaccion(Fecha::now(), TipoTransaccion::CompraCripto { monto: 200.0, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, &usuario);

        assert_eq!(sistema.cripto_mas_compras(), "BTC");
    }

    #[test]
    fn test_cripto_mas_ventas() {
        let mut sistema = XYZ::new();
        let usuario = create_user_with_identity("Jack", "Nicholson", "jack@example.com", "43211234");

        sistema.crear_transaccion(Fecha::now(), TipoTransaccion::VentaCripto { monto: 0.01, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, &usuario);
        sistema.crear_transaccion(Fecha::now(), TipoTransaccion::VentaCripto { monto: 0.02, criptomoneda: "ETH".to_string(), cotizacion: 2500.0 }, &usuario);
        sistema.crear_transaccion(Fecha::now(), TipoTransaccion::VentaCripto { monto: 0.01, criptomoneda: "BTC".to_string(), cotizacion: 50000.0 }, &usuario);

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
}*/