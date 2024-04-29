struct Fecha {
    dia: u32,
    mes: u32,
    año: u32,
}

impl Fecha {
    fn new(dia: u32, mes: u32, año: u32) -> Fecha {
        Fecha {
            dia,
            mes,
            año,
        }
    }

    fn es_fecha_valida(&self) -> bool {
        self.obtener_dias_para_mes() <= self.dia && self.dia > 0
    }

    fn es_bisiesto(&self) -> bool {
        self.año % 4 == 0
    }

    /// Devuelve la cantidad de dias que tiene el mes actual
    fn obtener_dias_para_mes(&self) -> u32 {
        const DIAS_POR_MES: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let dias = DIAS_POR_MES[(self.mes - 1) as usize];
        // bool as u32 = if true { 1 } else { 0 }
        dias + (self.mes == 2 && self.es_bisiesto()) as u32
    }

    //TODO: Hacer
    fn sumar_dias(&mut self, _dias: i32) {

    }

    //TODO: Hacer
    fn restar_dias(&mut self, _dias: i32) {

    }

    //TODO: No funciona
    fn es_mayor(&self, una_fecha: &Fecha) -> bool {
        self.año > una_fecha.año && self.mes > una_fecha.mes && self.dia > una_fecha.dia
    }
}

#[test]
fn test_es_mayor() {
    let fecha1 = Fecha::new(7, 5, 2005);
    let fecha2 = Fecha::new(2, 11, 2009);

    assert!(fecha1.es_mayor(&fecha2));
    assert!(!fecha2.es_mayor(&fecha1));
}