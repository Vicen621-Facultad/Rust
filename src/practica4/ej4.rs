//TODO: Terminar
use std::ops::Deref;
use crate::practica3::ej3::Fecha;

const DESCUENTO_NEWSLETTER: f32 = 0.1; // 10%

struct SistemaVentas {
    ventas: Vec<Venta>,
    vendedores: Vec<Vendedor>,
    clientes: Vec<Cliente>,
}

struct Venta {
    productos: Vec<Producto>,
    dni_cliente: String,
    legajo_vendedor: u32,
    metodo_pago: MetodoPago,
    fecha: Fecha
}

#[derive(Debug, PartialEq)]
enum MetodoPago {
    Efectivo,
    Credito  { numero_tarjeta: String, expiracion: String },
    Transferencia { cuenta: String},
    Debito  { numero_tarjeta: String, expiracion: String },
}

#[derive(Clone)]
struct Producto {
    nombre: String,
    precio: f32,
    categoria: CategoriaProducto,
}

#[derive(PartialEq, Clone)]
enum CategoriaProducto {
    Alimentos,
    Bebidas,
    Limpieza,
    Otros,
}

#[derive(Debug, PartialEq, Clone)]
struct DatosPersona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: String,
}

#[derive(Debug, PartialEq)]
struct Vendedor {
    datos: DatosPersona,
    legajo: u32
}

#[derive(Debug, PartialEq)]
struct Cliente {
    datos: DatosPersona,
    correo: Option<String>,
}

trait GestorVendedores {
    fn crear_vendedor(&mut self, nombre: String, apellido: String, direccion: String, dni: String, legajo: u32) -> &Vendedor;
    fn get_vendedor(&self, legajo: u32) -> Option<&Vendedor>;
}

trait GestorClientes {
    fn crear_cliente(&mut self, nombre: String, apellido: String, direccion: String, dni: String) -> &Cliente;
    fn get_cliente(&self, dni: &str) -> Option<&Cliente>;
    fn get_cliente_mut(&mut self, dni: &str) -> Option<&mut Cliente>;
}

trait GestorVentas {
    fn crear_venta(&mut self, productos: Vec<Producto>, datos_persona_cliente: DatosPersona, dni_cliente: String, datos_persona_vendedor: DatosPersona, legajo_vendedor: u32, metodo_pago: MetodoPago) -> &Venta;
    fn get_precio_final_venta(&self, venta: &Venta) -> f32;
}

trait Reporte {
    fn ventas_totales_vendedor(&self, legajo: u32) -> u32;
    fn ventas_totales_categoria(&self, categoria: CategoriaProducto) -> u32;
}

impl GestorVendedores for SistemaVentas {
    fn crear_vendedor(&mut self, nombre: String, apellido: String, direccion: String, dni: String, legajo: u32) -> &Vendedor{
        self.vendedores.push(Vendedor::new(nombre, apellido, direccion, dni, legajo));
        self.vendedores.last().unwrap()
    }

    fn get_vendedor(&self, legajo: u32) -> Option<&Vendedor> {
        self.vendedores.iter().find(|v| v.get_legajo() == legajo)
    }
}

impl GestorClientes for SistemaVentas {
    fn crear_cliente(&mut self, nombre: String, apellido: String, direccion: String, dni: String) -> &Cliente{
        self.clientes.push(Cliente::new(nombre, apellido, direccion, dni));
        self.clientes.last().unwrap()
    }

    fn get_cliente(&self, dni: &str) -> Option<&Cliente> {
        self.clientes.iter().find(|c| c.get_dni() == dni)
    }

    fn get_cliente_mut(&mut self, dni: &str) -> Option<&mut Cliente> {
        self.clientes.iter_mut().find(|c| c.get_dni() == dni)
    }
}

impl GestorVentas for SistemaVentas {
    fn crear_venta(&mut self, productos: Vec<Producto>, datos_persona_cliente: DatosPersona, dni_cliente: String, datos_persona_vendedor: DatosPersona, legajo_vendedor: u32, metodo_pago: MetodoPago) -> &Venta{
        if (self.get_cliente(&dni_cliente)).is_none() {
            self.crear_cliente(datos_persona_cliente.nombre, datos_persona_cliente.apellido, datos_persona_cliente.direccion, dni_cliente.clone());
        }
        if self.get_vendedor(legajo_vendedor).is_none() {
            self.crear_vendedor(datos_persona_vendedor.nombre, datos_persona_vendedor.apellido, datos_persona_vendedor.direccion, datos_persona_vendedor.dni, legajo_vendedor);
        }
        
        self.ventas.push(Venta::new(productos, dni_cliente, legajo_vendedor, metodo_pago));
        self.ventas.last().unwrap()
    }

    fn get_precio_final_venta(&self, venta: &Venta) -> f32 {
        if (self.get_cliente(&venta.dni_cliente)).unwrap().esta_suscrito() {
            venta.get_precio_final(DESCUENTO_NEWSLETTER)
        } else {
            venta.get_precio_final(0.0)
        }
    }
}

impl Reporte for SistemaVentas {
    fn ventas_totales_vendedor(&self, legajo: u32) -> u32 {
        self.ventas.iter().filter(|v| v.legajo_vendedor == legajo).count() as u32
    }

    fn ventas_totales_categoria(&self, categoria: CategoriaProducto) -> u32 {
        self.ventas.iter().map(|v: &Venta| v.productos.iter().filter(|p| p.get_categoria() == &categoria).count() as u32).sum()
    }
}

impl SistemaVentas {
    fn new() -> Self {
        SistemaVentas {
            ventas: Vec::new(),
            vendedores: Vec::new(),
            clientes: Vec::new(),
        }
    }
}

impl Venta {
    fn new(productos: Vec<Producto>, dni_cliente: String, legajo_vendedor: u32, metodo_pago: MetodoPago) -> Self {
        Venta {
            productos,
            dni_cliente,
            legajo_vendedor,
            metodo_pago,
            fecha: Fecha::now()
        }
    }

    fn get_precio_final(&self, descuento: f32) -> f32 {
        let total: f32 = self.productos.iter().map(|p| p.get_precio_final()).sum();
        total * (1.0 - descuento)
    }
}

impl Producto {
    fn new(nombre: String, precio: f32, categoria: CategoriaProducto) -> Self {
        Producto {
            nombre,
            precio,
            categoria
        }
    }

    fn get_precio_final(&self) -> f32 {
        self.precio * (1.0 - self.categoria.get_descuento())
    }

    fn get_nombre(&self) -> &String {
        &self.nombre
    }

    fn get_categoria(&self) -> &CategoriaProducto {
        &self.categoria
    }

    fn get_precio(&self) -> f32 {
        self.precio
    }
}

impl CategoriaProducto {
    fn get_descuento(&self) -> f32 {
        match self {
            CategoriaProducto::Alimentos => 0.1,    // 10%
            CategoriaProducto::Bebidas => 0.05,     // 5%
            CategoriaProducto::Limpieza => 0.15,    // 15%
            CategoriaProducto::Otros => 0.0,        // 0%
        }
    }
}

impl DatosPersona {
    fn new(nombre: String, apellido: String, direccion: String, dni: String) -> Self {
        DatosPersona {
            nombre,
            apellido,
            direccion,
            dni
        }
    }

    fn get_nombre(&self) -> &String {
        &self.nombre
    }

    fn get_apellido(&self) -> &String {
        &self.apellido
    }

    fn get_direccion(&self) -> &String {
        &self.direccion
    }

    fn get_dni(&self) -> &String {
        &self.dni
    }
}

impl Deref for Vendedor {
    type Target = DatosPersona;

    fn deref(&self) -> &Self::Target {
        &self.datos
    }

}

impl Vendedor {
    fn new(nombre: String, apellido: String, direccion: String, dni: String, legajo: u32) -> Self {
        Vendedor {
            datos: DatosPersona::new(nombre, apellido, direccion, dni),
            legajo
        }
    }

    fn get_legajo(&self) -> u32 {
        self.legajo
    }
}

impl Deref for Cliente {
    type Target = DatosPersona;

    fn deref(&self) -> &Self::Target {
        &self.datos
    }

}

impl Cliente {
    fn new(nombre: String, apellido: String, direccion: String, dni: String) -> Self {
        Cliente {
            datos: DatosPersona::new(nombre, apellido, direccion, dni),
            correo: None
        }
    }

    fn get_newsletter(&self) -> Option<&String> {
        self.correo.as_ref()
    }

    fn esta_suscrito(&self) -> bool {
        self.correo.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_vendedor() {
        let mut sistema = SistemaVentas::new();
        let vendedor = sistema.crear_vendedor("Juan".to_string(), "Perez".to_string(), "Calle falsa 123".to_string(), "12345678".to_string(), 1);

        assert_eq!(vendedor.get_nombre(), "Juan");
        assert_eq!(vendedor.get_apellido(), "Perez");
        assert_eq!(vendedor.get_direccion(), "Calle falsa 123");
        assert_eq!(vendedor.get_dni(), "12345678");
        assert_eq!(vendedor.get_legajo(), 1);
    }

    #[test]
    fn test_get_vendedor() {
        let mut sistema = SistemaVentas::new();
        sistema.crear_vendedor("Juan".to_string(), "Perez".to_string(), "Calle falsa 123".to_string(), "12345678".to_string(), 1);
        let get_vendedor = sistema.get_vendedor(1);
        assert!(get_vendedor.is_some());
        let vendedor = get_vendedor.unwrap();
        assert_eq!(vendedor.get_nombre(), "Juan");
        assert_eq!(vendedor.get_apellido(), "Perez");
        assert_eq!(vendedor.get_direccion(), "Calle falsa 123");
        assert_eq!(vendedor.get_dni(), "12345678");
        assert_eq!(vendedor.get_legajo(), 1);
    }

    #[test]
    fn test_crear_cliente() {
        let mut sistema = SistemaVentas::new();
        let cliente = sistema.crear_cliente("Juan".to_string(), "Perez".to_string(), "Calle falsa 123".to_string(), "12345678".to_string());

        assert_eq!(cliente.get_nombre(), "Juan");
        assert_eq!(cliente.get_apellido(), "Perez");
        assert_eq!(cliente.get_direccion(), "Calle falsa 123");
        assert_eq!(cliente.get_dni(), "12345678");
    }

    #[test]
    fn test_get_cliente() {
        let mut sistema = SistemaVentas::new();
        sistema.crear_cliente("Juan".to_string(), "Perez".to_string(), "Calle falsa 123".to_string(), "12345678".to_string());
        let get_cliente = sistema.get_cliente("12345678");
        assert!(get_cliente.is_some());
        let cliente = get_cliente.unwrap();
        assert_eq!(cliente.get_nombre(), "Juan");
        assert_eq!(cliente.get_apellido(), "Perez");
        assert_eq!(cliente.get_direccion(), "Calle falsa 123");
        assert_eq!(cliente.get_dni(), "12345678");
    }

    #[test]
    fn test_crear_venta() {
        let mut sistema = SistemaVentas::new();
        let productos = vec![
            Producto::new("Pan".to_string(), 100.0, CategoriaProducto::Alimentos),
            Producto::new("Coca".to_string(), 150.0, CategoriaProducto::Bebidas),
        ];
        let datos_persona_cliente = DatosPersona::new("Juan".to_string(), "Perez".to_string(), "Calle falsa 123".to_string(), "12345678".to_string());
        let datos_persona_vendedor = DatosPersona::new("Pedro".to_string(), "Gomez".to_string(), "Calle falsa 456".to_string(), "87654321".to_string());
        sistema.crear_venta(productos, datos_persona_cliente, "12345678".to_string(), datos_persona_vendedor, 1, MetodoPago::Efectivo);

        assert_eq!(sistema.ventas.len(), 1);
        let venta = &sistema.ventas[0];
        assert_eq!(venta.productos.len(), 2);
        assert_eq!(venta.dni_cliente, "12345678");
        assert_eq!(venta.legajo_vendedor, 1);
        assert_eq!(venta.metodo_pago, MetodoPago::Efectivo);
    }
    
    #[test]
    fn test_ventas_totales_vendedor() {
        let mut sistema = SistemaVentas::new();
        let productos = vec![
            Producto::new("Pan".to_string(), 100.0, CategoriaProducto::Alimentos),
            Producto::new("Coca".to_string(), 150.0, CategoriaProducto::Bebidas),
        ];
        let datos_persona_cliente = DatosPersona::new("Juan".to_string(), "Perez".to_string(), "Calle falsa 123".to_string(), "12345678".to_string());
        let datos_persona_vendedor = DatosPersona::new("Pedro".to_string(), "Gomez".to_string(), "Calle falsa 456".to_string(), "87654321".to_string());
        sistema.crear_venta(productos.clone(), datos_persona_cliente.clone(), "12345678".to_string(), datos_persona_vendedor.clone(), 1, MetodoPago::Efectivo);
        sistema.crear_venta(productos.clone(), datos_persona_cliente.clone(), "12345678".to_string(), datos_persona_vendedor.clone(), 1, MetodoPago::Efectivo);
        sistema.crear_venta(productos.clone(), datos_persona_cliente.clone(), "12345678".to_string(), datos_persona_vendedor.clone(), 2, MetodoPago::Efectivo);

        assert_eq!(sistema.ventas_totales_vendedor(1), 2);
        assert_eq!(sistema.ventas_totales_vendedor(2), 1);
        assert_eq!(sistema.ventas_totales_vendedor(3), 0);
    }

    #[test]
    fn test_ventas_totales_categoria() {
        let mut sistema = SistemaVentas::new();
        let productos = vec![
            Producto::new("Pan".to_string(), 100.0, CategoriaProducto::Alimentos),
            Producto::new("Coca".to_string(), 150.0, CategoriaProducto::Bebidas),
        ];
        let datos_persona_cliente = DatosPersona::new("Juan".to_string(), "Perez".to_string(), "Calle falsa 123".to_string(), "12345678".to_string());
        let datos_persona_vendedor = DatosPersona::new("Pedro".to_string(), "Gomez".to_string(), "Calle falsa 456".to_string(), "87654321".to_string());
        sistema.crear_venta(productos.clone(), datos_persona_cliente.clone(), "12345678".to_string(), datos_persona_vendedor.clone(), 1, MetodoPago::Efectivo);
        sistema.crear_venta(productos.clone(), datos_persona_cliente.clone(), "12345678".to_string(), datos_persona_vendedor.clone(), 1, MetodoPago::Efectivo);
        sistema.crear_venta(productos.clone(), datos_persona_cliente.clone(), "12345678".to_string(), datos_persona_vendedor.clone(), 2, MetodoPago::Efectivo);

        assert_eq!(sistema.ventas_totales_categoria(CategoriaProducto::Alimentos), 3);
        assert_eq!(sistema.ventas_totales_categoria(CategoriaProducto::Bebidas), 3);
        assert_eq!(sistema.ventas_totales_categoria(CategoriaProducto::Limpieza), 0);
        assert_eq!(sistema.ventas_totales_categoria(CategoriaProducto::Otros), 0);
    }

    #[test]
    fn test_get_precio_final_venta() {
        let mut sistema = SistemaVentas::new();
        let productos = vec![
            Producto::new("Pan".to_string(), 100.0, CategoriaProducto::Alimentos),
            Producto::new("Coca".to_string(), 150.0, CategoriaProducto::Bebidas),
        ];
        let datos_persona_cliente = DatosPersona::new("Juan".to_string(), "Perez".to_string(), "Calle falsa 123".to_string(), "12345678".to_string());
        let datos_persona_vendedor = DatosPersona::new("Pedro".to_string(), "Gomez".to_string(), "Calle falsa 456".to_string(), "87654321".to_string());
        sistema.crear_venta(productos.clone(), datos_persona_cliente.clone(), "12345678".to_string(), datos_persona_vendedor.clone(), 1, MetodoPago::Efectivo);
        assert_eq!(sistema.get_precio_final_venta(&sistema.ventas[0]), 232.5);

        sistema.get_cliente_mut("12345678").unwrap().correo = Some("test@example.com".to_string());

        assert_eq!(sistema.get_precio_final_venta(&sistema.ventas[0]), 209.25);
    }

    #[test]
    fn test_get_precio_final() {
        let productos = vec![
            Producto::new("Pan".to_string(), 100.0, CategoriaProducto::Alimentos),
            Producto::new("Coca".to_string(), 150.0, CategoriaProducto::Bebidas),
        ];
        let venta = Venta::new(productos, "12345678".to_string(), 1, MetodoPago::Efectivo);
        assert_eq!(venta.get_precio_final(0.0), 232.5);
        assert_eq!(venta.get_precio_final(0.1), 225.0);
    }

    #[test]
    fn test_get_precio_final_producto() {
        let producto = Producto::new("Pan".to_string(), 100.0, CategoriaProducto::Alimentos);
        assert_eq!(producto.get_precio_final(), 90.0);
    }

    #[test]
    fn test_get_descuento_categoria() {
        assert_eq!(CategoriaProducto::Alimentos.get_descuento(), 0.1);
        assert_eq!(CategoriaProducto::Bebidas.get_descuento(), 0.05);
        assert_eq!(CategoriaProducto::Limpieza.get_descuento(), 0.15);
        assert_eq!(CategoriaProducto::Otros.get_descuento(), 0.0);
    }
}