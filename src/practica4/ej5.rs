struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
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