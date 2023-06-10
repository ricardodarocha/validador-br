pub mod alfa {
    /// Define algumas funções básicas como `módulo_11` e `onze_menos_mod11`
    pub fn mod_11(value: u32) -> usize {
        match value % 11 {
            10 => 0,
            dv => dv as usize,
        }
    }

    /// Alguns documentos realizam a subtração 11 - mod11
    pub fn onze_menos_mod11(value: u32) -> u32 {
        let res = 11 - value % 11;
        if res > 9 {
            0
        } else {
            res
        }
    }
}

pub mod gama {
    /// Define métodos que são usados com frequência pelos algoritmos de verificação, como *cálculo do dígito verificador*, *cálculo usando mod11*, `somente_digitos`
    use crate::alfa::mod_11;

    /// Dada uma sequência de n dígitos qualquer, verificar se o último dígito corresponde ao módulo 11 dos demais para os multiplicadores dados
    /// ⚠ Importante observar a particularidade de cada doc.
    /// Por exemplo:
    /// * CPF os multiplicadores são sempre (10..=2) sendo dv1 função dos dígitos [0..9] e dv2 função dos dígitos [1..10]
    /// * IE os multiplicadores variam de estado para estado sendo dv1 = os dígitos [0..n-1] e dv2 = os dígitos [0..n]

    /// Definimos um método genérico para calcular o dígito verificador de um doc, recebendo como parâmetro um vetor com os multiplicadores
    /// Este método **não** calculará mod 11 automaticamente, devendo ser passada por parâmetro uma função ou closure para realizar a verificação 
    ///```rust
    /// # use validador_br::gama::calc_digito;
    ///assert!(calc_digito(vec![9, 5, 8, 7], vec![2, 3, 4, 5], |x| x % 10) == 0);
    ///```
    
    pub fn calc_digito(
        digitos: Vec<u32>,
        multiplicadores: Vec<u32>,
        funcao_modulo: fn(x: u32) -> u32,
    ) -> u32 {
        assert_eq!(digitos.len(), multiplicadores.len());

        let result: u32 = digitos
            .iter()
            .enumerate()
            .map(|(index, value)| multiplicadores[index] * value)
            .sum();

        funcao_modulo(result)
    }

    // Usa mod_11 simples, enquanto cpf/cnpj usa o mod_11 de (soma)*10
    pub fn calc_digito_mod11(digitos: Vec<u32>, multiplicadores: Vec<u32>) -> u32 {
        assert_eq!(digitos.len(), multiplicadores.len());

        let result: u32 = digitos
            .iter()
            .enumerate()
            .map(|(index, value)| multiplicadores[index] * value)
            .sum();

        mod_11(result) as u32
    }

    /// 326.911.358-70 => \[3,2,6,9,1,1,3,5,8,7,0\]
    pub fn somente_digitos(dor: &str, n: usize) -> Vec<u32> {
        dor.chars()
            .filter(|c| c.is_ascii_digit())
            .map(|x| x.to_digit(10).unwrap())
            .take(n)
            .collect()
    }
}
pub mod validador_ie {
    pub struct IeUf();
}

pub mod validador {
    use crate::alfa::{mod_11, onze_menos_mod11};
    use crate::gama::{calc_digito, calc_digito_mod11, somente_digitos};
    use crate::validador_ie::IeUf;

    pub struct Cpf(pub &'static str);
    pub struct Cnpj(pub &'static str);
    pub struct Ie(pub IeUf);
    pub struct CartaoCredito(pub &'static str);
    pub struct TituloEleitor(pub &'static str);
    pub struct Cnh(pub &'static str);
    pub struct Rg(pub &'static str);
    pub struct Pis(pub &'static str);

    pub trait Validador {
        fn validar(&self) -> bool {
            true
        }
    }

    impl Validador for Cpf {
        fn validar(&self) -> bool {
            let multiplicadores = vec![10, 9, 8, 7, 6, 5, 4, 3, 2];
            let digitos = somente_digitos(self.0, 11);
            let part1 = digitos[0..9].to_vec();
            let dv1 = &digitos[9];
            let part2 = digitos[1..10].to_vec();
            let dv2 = &digitos[10];
            let decima = |x: u32| mod_11(10 * x) as u32;

            let digitos_verificadores: (u32, u32) = (
                calc_digito(part1, multiplicadores.clone(), decima),
                calc_digito(part2, multiplicadores, decima),
            );
            digitos_verificadores == (*dv1, *dv2)
        }
    }

    impl Validador for Cnpj {
        fn validar(&self) -> bool {
            let multiplicadores1 = vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
            let digitos = somente_digitos(self.0, 14);
            let part1 = digitos[0..12].to_vec();
            let dv1 = &digitos[12];
            let multiplicadores2 = vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
            let part2 = digitos[0..13].to_vec();
            let dv2 = &digitos[13];
            let decima = |x: u32| mod_11(10 * x) as u32;

            let digitos_verificadores: (u32, u32) = (
                calc_digito(part1, multiplicadores1, decima),
                calc_digito(part2, multiplicadores2, decima),
            );
            digitos_verificadores == (*dv1, *dv2)
        }
    }

    impl Validador for CartaoCredito {
        fn validar(&self) -> bool {
            let mut numbers = somente_digitos(self.0, self.0.len());
            numbers.reverse();
            let mut is_odd: bool = true;
            let mut odd_sum: u32 = 0;
            let mut even_sum: u32 = 0;
            for digit in numbers {
                if is_odd {
                    odd_sum += digit;
                } else {
                    even_sum += digit / 5 + (2 * digit) % 10;
                }
                is_odd = !is_odd
            }

            (odd_sum + even_sum) % 10 == 0
        }
    }
    
    impl Validador for TituloEleitor {
        fn validar(&self) -> bool {
            let multiplicadores1 = vec![2, 3, 4, 5, 6, 7, 8, 9];
            let digitos = somente_digitos(self.0, 12);
            let part1 = digitos[0..8].to_vec();
            let dv1 = &digitos[10];
            let multiplicadores2 = vec![7, 8, 9];
            let part2 = digitos[8..11].to_vec();
            let dv2 = &digitos[11];

            let digitos_verificadores: (u32, u32) = (
                calc_digito_mod11(part1, multiplicadores1),
                calc_digito_mod11(part2, multiplicadores2),
            );
            digitos_verificadores == (*dv1, *dv2)
        }
    }

    impl Validador for Cnh {
        fn validar(&self) -> bool {
            let multiplicadores1 = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
            let digitos = somente_digitos(self.0, 11);
            let part1 = digitos[0..9].to_vec();
            let dv1 = &digitos[9];
            let multiplicadores2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let part2 = digitos[0..9].to_vec();
            let dv2 = &digitos[10];

            let mut verifica1 = calc_digito(part1, multiplicadores1, |x| x);
            let delta: u32 = if verifica1 % 11 == 10 { 2 } else { 0 };
            verifica1 = mod_11(verifica1) as u32;

            let verificador_final = |soma: u32| {
                let mod11 = soma % 11;
                let res = match Some(mod11 - delta) {
                    Some(x) => x,
                    None => 11 + mod11 - delta,
                };
                if res > 9 {
                    0
                } else {
                    res
                }
            };

            let mut verifica2 = calc_digito(part2, multiplicadores2, |x| x);
            verifica2 = verificador_final(verifica2);

            (verifica1, verifica2) == (*dv1, *dv2)
        }
    }

    impl Validador for Rg {
        fn validar(&self) -> bool {
            let multiplicadores1 = vec![2, 3, 4, 5, 6, 7, 8, 9];
            let digitos = somente_digitos(self.0, 9);
            let part1 = digitos[0..8].to_vec();
            let dv1 = &digitos[8];
            let digitos_verificadores = calc_digito(part1, multiplicadores1, onze_menos_mod11);
            digitos_verificadores == *dv1
        }
    }

    impl Validador for Pis {
        fn validar(&self) -> bool {
            let multiplicadores1 = vec![3,2,9,8,7,6,5,4,3,2];
            let mut digitos = somente_digitos(self.0, 11);
            let dv1 = digitos.split_off(10);
            println!("{:?}", dv1);
            println!("{:?}", digitos);

            dv1[0] == calc_digito(digitos, multiplicadores1, onze_menos_mod11)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::alfa::mod_11;
    use crate::gama::{calc_digito, somente_digitos};
    use crate::validador::{CartaoCredito, Cnh, Cnpj, Cpf, Pis, Rg, TituloEleitor, Validador};

    #[test]
    fn test_mod_11() {
        fn test(input: u32, output: usize) {
            let result = mod_11(input);
            assert_eq!(result, output);
        }

        test(810, 7);
        test(820, 6);
        test(830, 5);
        test(840, 4);
        test(850, 3);
        test(860, 2);
        test(870, 1);
        test(880, 0);
        test(890, 0);
        test(900, 9);
        test(910, 8);
        test(920, 7);
        test(930, 6);
        test(940, 5);
        test(950, 4);
        test(960, 3);
        test(970, 2);
        test(980, 1);
        test(990, 0);
        test(1000, 0);
        test(1010, 9);
        test(1020, 8);
        test(1030, 7);
        test(1040, 6);
        test(1050, 5);
        test(1060, 4);
        test(1070, 3);
        test(1080, 2);
        test(2510, 2);
    }

    #[test]
    fn test_calc_digito() {
        fn test(input: &str, multiplicadores: Vec<u32>) {
            let split_size = multiplicadores.len();
            let mut digitos = somente_digitos(input, split_size + 1);
            let output = digitos.split_off(split_size);
            let result = calc_digito(digitos, multiplicadores, |x| mod_11(10 * x) as u32);
            assert_eq!(result, output[0]);
        }

        test("1556748620", vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
        test("0155674862", vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
        test("26.911.358-70", vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
        test("326.911.358-7", vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
        test("883.926.778-6", vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
        test("83.926.778-62", vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
        test("0155674862", vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
    }
    #[test]
    fn test_somente_digitos() {
        fn test(input: &str, output: Vec<u32>) {
            let result = somente_digitos(input, input.len());
            assert_eq!(result, output);
        }

        test(&"0", vec![0]);
        test(&"1", vec![1]);
        test(&"5", vec![5]);
        test(&"9", vec![9]);
        test(&"10", vec![1, 0]);
        test(&"123", vec![1, 2, 3]);
        test(&"1.23-0", vec![1, 2, 3, 0]);
        test(&"1.23/001-0", vec![1, 2, 3, 0, 0, 1, 0]);
    }

    #[test]
    fn test_valid_cpf() {
        assert!(Cpf(&"085.668.830-47").validar());
        assert!(Cpf(&"712.926.512-45").validar());
        assert!(Cpf(&"974.749.266-01").validar());
        assert!(Cpf(&"201.859.154-18").validar());
        assert!(Cpf(&"274.047.203-03").validar());
        assert!(Cpf(&"864.148.641-02").validar());
    }

    #[test]
    fn test_valid_cnpj() {
        assert!(Cnpj(&"14.572.457.0001-85").validar());
        assert!(Cnpj(&"76.553.412/0001-10").validar());
        assert!(Cnpj(&"45.184.338/0001-89").validar());
        assert!(Cnpj(&"37.645.328/0001-75").validar());
        assert!(Cnpj(&"55.131.839/0001-50").validar());
        assert!(Cnpj(&"44.122.045/0001-04").validar());
        assert!(Cnpj(&"41.768.737/0001-36").validar());
        assert!(Cnpj(&"89.267.553/0001-19").validar());
        assert!(Cnpj(&"40.357.317/0001-02").validar());
        assert!(Cnpj(&"46.922.782/0001-17").validar());
        assert!(Cnpj(&"86.107.547/0001-06").validar());
    }

    #[test]
    fn test_valid_cc() {
        assert!(CartaoCredito(&"5312 8338 4531 6765").validar());
        assert!(CartaoCredito(&"5440 4970 4224 4678").validar());
        assert!(CartaoCredito(&"6011 1122 7847 6822").validar());
        assert!(CartaoCredito(&"3479 467653 71543").validar());
        assert!(CartaoCredito(&"2014 4799698 0942").validar());
        assert!(CartaoCredito(&"6062 8271 2873 9719").validar());
    }

    #[test]
    fn test_valid_titulo_eleitor() {
        assert!(TituloEleitor(&"00435687 09-06").validar());
        assert!(TituloEleitor(&"781613810175").validar());
        assert!(TituloEleitor(&"8645582519 29").validar());
        assert!(TituloEleitor(&"272.20357 20-20").validar());
    }

    #[test]
    fn test_valid_cnh() {
        assert!(Cnh(&"81814756744").validar());
        assert!(Cnh(&"28851304391").validar());
        assert!(Cnh(&"72887311600").validar());
        assert!(Cnh(&"86033265137").validar());
        assert!(Cnh(&"40386768760").validar());
    }

    #[test]
    fn test_valid_rg() {
        assert!(Rg(&"14.176.381-4").validar());
        assert!(Rg(&"28.530.378-8").validar());
        assert!(Rg(&"28.346.393-4").validar());
        assert!(Rg(&"39.371.494-9").validar());
        assert!(Rg(&"24.657.777-0").validar());
        assert!(Rg(&"17.227.771-1").validar());
    }

    #[test]
    fn test_valid_pis() {
        assert!(Pis(&"608.37951.54-6").validar());
        assert!(Pis(&"317.73180.80-0").validar());
    }
}
