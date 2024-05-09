trait EsPrimo {
    fn es_primo(&self) -> bool;
}

impl EsPrimo for i32 {
    fn es_primo(&self) -> bool {
        let mut count = 0;
        for i in 1..(self + 1) {
            if self % i == 0 {
                count += 1;
            }

            if count > 2 {
                break;
            }
        }

        count <= 2 && count > 1
    }
}

fn contar_primos(vector: &Vec<i32>) -> u32 {
    vector.iter().filter(|i| i.es_primo()).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_es_primo() {
        let num = 97;
        assert!(num.es_primo());
    }
    
    #[test]
    fn test_false_es_primo() {
        let num = 15;
        assert!(!num.es_primo());
    }

    #[test]
    fn test_contar_primos() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(contar_primos(&vec), 4);
    }
}