#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Fecha {
    dia: u32,
    mes: u32,
    año: u32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, año: u32) -> Fecha {
        Fecha {
            dia,
            mes,
            año,
        }
    }

    pub fn es_fecha_valida(&self) -> bool {
        self.dia <= self.obtener_dias_para_mes() && self.dia > 0
    }

    pub fn es_bisiesto(&self) -> bool {
        self.año % 4 == 0
    }

    /// Devuelve la cantidad de dias que tiene el mes actual
    fn obtener_dias_para_mes(&self) -> u32 {
        if self.mes > 12 || self.mes < 1 {
            return 0;
        }

        const DIAS_POR_MES: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let dias = DIAS_POR_MES[(self.mes - 1) as usize];
        // bool as u32 = if true { 1 } else { 0 }
        dias + (self.mes == 2 && self.es_bisiesto()) as u32
    }

    pub fn sumar_dias(&mut self, dias: u32) {
        let mut dias_restantes = dias;
        while dias_restantes > 0 {
            let dias_en_mes = self.obtener_dias_para_mes();
            // Se suma 1 ya que tengo que contar el dia actual
            let dias_hasta_fin_de_mes = dias_en_mes - self.dia + 1;

            if dias_hasta_fin_de_mes > dias_restantes {
                self.dia += dias_restantes;
                dias_restantes = 0;
            } else {
                dias_restantes -= dias_hasta_fin_de_mes;
                self.mes += 1;
                if self.mes > 12 {
                    self.mes = 1;
                    self.año += 1;
                }
                self.dia = 1;
            }
        }
    }

    pub fn restar_dias(&mut self, dias: u32) {
        let mut dias_restantes = dias;
        while dias_restantes > 0 {
            if self.dia > dias_restantes {
                self.dia -= dias_restantes;
                dias_restantes = 0;
            } else {
                dias_restantes -= self.dia;
                self.mes -= 1;
                if self.mes == 0 {
                    self.mes = 12;
                    self.año -= 1;
                }
                self.dia = self.obtener_dias_para_mes();
            }
        }
    }

    pub fn es_mayor(&self, una_fecha: &Fecha) -> bool {
        (self.año > una_fecha.año) || 
            (self.año == una_fecha.año && self.mes > una_fecha.mes) || 
            (self.año == una_fecha.año && self.mes == una_fecha.mes && self.dia > una_fecha.dia)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_fecha_valida() {
        // Fecha válida
        let fecha_valida = Fecha::new(15, 6, 2024);
        assert!(fecha_valida.es_fecha_valida());

        // Fecha inválida (día fuera de rango)
        let fecha_invalida_dia = Fecha::new(32, 6, 2024);
        assert!(!fecha_invalida_dia.es_fecha_valida());

        // Fecha inválida (mes fuera de rango)
        let fecha_invalida_mes = Fecha::new(15, 13, 2024);
        assert!(!fecha_invalida_mes.es_fecha_valida());

        // Fecha inválida (febrero en año no bisiesto)
        let fecha_invalida_febrero_no_bisiesto = Fecha::new(29, 2, 2023);
        assert!(!fecha_invalida_febrero_no_bisiesto.es_fecha_valida());

        // Fecha válida (febrero en año bisiesto)
        let fecha_valida_febrero_bisiesto = Fecha::new(29, 2, 2024);
        assert!(fecha_valida_febrero_bisiesto.es_fecha_valida());
    }

    #[test]
    fn test_es_bisiesto() {
        // Año bisiesto
        let fecha_bisiesto = Fecha::new(1, 1, 2024);
        assert!(fecha_bisiesto.es_bisiesto());

        // Año no bisiesto
        let fecha_no_bisiesto = Fecha::new(1, 1, 2023);
        assert!(!fecha_no_bisiesto.es_bisiesto());
    }

    #[test]
    fn test_sumar_dias() {
        let mut fecha = Fecha::new(1, 1, 2024);
        fecha.sumar_dias(365);
        assert_eq!(fecha, Fecha::new(31, 12, 2024));
        fecha.sumar_dias(1);
        assert_eq!(fecha, Fecha::new(1, 1, 2025));
        fecha.sumar_dias(5);
        assert_eq!(fecha, Fecha::new(6, 1, 2025));
    }

    #[test]
    fn test_restar_dias() {
        let mut fecha = Fecha::new(31, 12, 2024);
        fecha.restar_dias(365);
        assert_eq!(fecha, Fecha::new(1, 1, 2024));
        fecha.restar_dias(1);
        assert_eq!(fecha, Fecha::new(31, 12, 2023));
        fecha.restar_dias(5);
        assert_eq!(fecha, Fecha::new(26, 12, 2023));
    }

    #[test]
    fn test_es_mayor() {
        let fecha1 = Fecha::new(5, 3, 2024);
        let fecha2 = Fecha::new(5, 3, 2023);
        assert!(fecha1.es_mayor(&fecha2));

        let fecha3 = Fecha::new(5, 3, 2023);
        let fecha4 = Fecha::new(5, 3, 2024);
        assert!(!fecha3.es_mayor(&fecha4));

        let fecha5 = Fecha::new(5, 4, 2024);
        let fecha6 = Fecha::new(5, 3, 2024);
        assert!(fecha5.es_mayor(&fecha6));

        let fecha7 = Fecha::new(5, 3, 2024);
        let fecha8 = Fecha::new(5, 4, 2024);
        assert!(!fecha7.es_mayor(&fecha8));

        let fecha9 = Fecha::new(6, 3, 2024);
        let fecha10 = Fecha::new(5, 3, 2024);
        assert!(fecha9.es_mayor(&fecha10));

        let fecha11 = Fecha::new(5, 3, 2024);
        let fecha12 = Fecha::new(6, 3, 2024);
        assert!(!fecha11.es_mayor(&fecha12));
    }
}