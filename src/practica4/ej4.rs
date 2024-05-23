//TODO: Terminar
use std::ops::Deref;
use crate::practica3::ej3::Fecha;

const DESCUENTO_NEWSLETTER: f32 = 0.1; // 10%

struct SistemaVentas<'a> {
    ventas: Vec<Venta<'a>>,
    vendedores: Vec<Vendedor>,
    clientes: Vec<Cliente>,
}

struct Venta<'a> {
    productos: Vec<Producto>,
    cliente: &'a Cliente,
    vendedor: &'a Vendedor,
    metodo_pago: MetodoPago,
    fecha: Fecha
}

enum MetodoPago {
    Efectivo,
    Credito  { numero_tarjeta: String, expiracion: String },
    Transferencia { cuenta: String},
    Debito  { numero_tarjeta: String, expiracion: String },
}

struct Producto {
    nombre: String,
    precio: f32,
    categoria: CategoriaProducto,
}

enum CategoriaProducto {
    Alimentos,
    Bebidas,
    Limpieza,
    Otros,
}

struct DatosPersona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: String,
}

struct Vendedor {
    datos: DatosPersona,
    legajo: u32
}


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
}

trait GestorVentas<'a> {
    fn crear_venta(&mut self, productos: Vec<Producto>, cliente: &'a Cliente, vendedor: &'a Vendedor, metodo_pago: MetodoPago, fecha: Fecha) -> &'a Venta;
}

impl<'a> GestorVendedores for SistemaVentas<'a> {
    fn crear_vendedor(&mut self, nombre: String, apellido: String, direccion: String, dni: String, legajo: u32) -> &Vendedor{
        self.vendedores.push(Vendedor::new(nombre, apellido, direccion, dni, legajo));
        self.vendedores.last().unwrap()
    }

    fn get_vendedor(&self, legajo: u32) -> Option<&Vendedor> {
        self.vendedores.iter().find(|v| v.get_legajo() == legajo)
    }
}

impl<'a> GestorClientes for SistemaVentas<'a> {
    fn crear_cliente(&mut self, nombre: String, apellido: String, direccion: String, dni: String) -> &Cliente{
        self.clientes.push(Cliente::new(nombre, apellido, direccion, dni));
        self.clientes.last().unwrap()
    }

    fn get_cliente(&self, dni: &str) -> Option<&Cliente> {
        self.clientes.iter().find(|c| c.get_dni() == dni)
    }
}

impl<'a> GestorVentas<'a> for SistemaVentas<'a> {
    fn crear_venta(&mut self, productos: Vec<Producto>, cliente: &'a Cliente, vendedor: &'a Vendedor, metodo_pago: MetodoPago, fecha: Fecha) -> &'a Venta{
        if (self.get_cliente(&cliente.dni)).is_none() {
            // self.crear_cliente(cliente.nombre, cliente.apellido, cliente.direccion, cliente.dni);
        }

        self.ventas.push(Venta::new(productos, cliente, &vendedor, metodo_pago, fecha));
        self.ventas.last().unwrap()
    }
}

impl<'a> SistemaVentas<'a> {
    fn new() -> Self {
        SistemaVentas {
            ventas: Vec::new(),
            vendedores: Vec::new(),
            clientes: Vec::new(),
        }
    }
}

trait CalcularPrecio {
    fn get_precio_final(&self) -> f32;
}

impl CalcularPrecio for Venta<'_> {
    fn get_precio_final(&self) -> f32 {
        let total = self.productos.iter().map(|p| p.get_precio_final()).sum();
        
        if self.cliente.esta_suscrito() {
            total * (1.0 - DESCUENTO_NEWSLETTER)
        } else {
            total
        }
    }
}

impl<'a> Venta<'a> {
    fn new(productos: Vec<Producto>, cliente: &'a Cliente, vendedor: &'a Vendedor, metodo_pago: MetodoPago, fecha: Fecha) -> Self {
        Venta {
            productos,
            cliente,
            vendedor,
            metodo_pago,
            fecha
        }
    }
}

impl CalcularPrecio for Producto {
    fn get_precio_final(&self) -> f32 {
        self.precio * (1.0 - self.categoria.get_descuento())
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