use crate::funcoes::{calc_digito, calc_digito_mod11, completa_esquerda, mod_11, 
onze_menos_mod11, somente_digitos,
};

use crate::types::{CartaoCredito, Cnh, Cnpj, Cns, CodigoBarrasGs1, Cpf, Pis, Renavam, 
Rg, TipoCns, TituloEleitor,
};

pub trait Validador {
    fn is_valid(_numero: &str) -> bool {
        false
    }

    fn validar(&self) -> bool;
}

impl Validador for Cpf<'_> {
    fn is_valid(numero: &str) -> bool {
        let multiplicadores = vec![10, 9, 8, 7, 6, 5, 4, 3, 2];
        let digitos = somente_digitos(numero, 11);
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

    fn validar(&self) -> bool {
        Self::is_valid(self.0)
    }
}

impl Validador for Cnpj<'_> {
    fn is_valid(numero: &str) -> bool {
        let multiplicadores1 = vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let digitos = somente_digitos(numero, 14);
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

    fn validar(&self) -> bool {
        Self::is_valid(self.0)
    }
}

impl Validador for CartaoCredito<'_> {
    fn is_valid(numero: &str) -> bool {
        let mut numbers = somente_digitos(numero, numero.len());
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

    fn validar(&self) -> bool {
        Self::is_valid(self.0)
    }
}

impl Validador for TituloEleitor<'_> {
    fn is_valid(numero: &str) -> bool {
        let multiplicadores1 = vec![2, 3, 4, 5, 6, 7, 8, 9];
        let digitos = somente_digitos(numero, 12);
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

    fn validar(&self) -> bool {
        Self::is_valid(self.0)
    }
}

impl Validador for Cnh<'_> {
    fn is_valid(numero: &str) -> bool {
        let multiplicadores1 = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let digitos = somente_digitos(numero, 11);
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

    fn validar(&self) -> bool {
        Self::is_valid(self.0)
    }
}

impl Validador for Renavam<'_> {
    fn is_valid(numero: &str) -> bool {
        let multiplicadores1 = vec![3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let mut digitos = somente_digitos(numero, 11);
        completa_esquerda(&mut digitos, 11);
        let dv1 = digitos.split_off(10);

        dv1[0] == calc_digito(digitos, multiplicadores1, onze_menos_mod11)
    }

    fn validar(&self) -> bool {
        Self::is_valid(self.0)
    }
}

impl Validador for Rg<'_> {
    fn is_valid(numero: &str) -> bool {
        let multiplicadores1 = vec![2, 3, 4, 5, 6, 7, 8, 9];
        let digitos = somente_digitos(numero, 9);
        let part1 = digitos[0..8].to_vec();
        let dv1 = &digitos[8];
        let digitos_verificadores = calc_digito(part1, multiplicadores1, onze_menos_mod11);
        digitos_verificadores == *dv1
    }

    fn validar(&self) -> bool {
        Self::is_valid(self.0)
    }
}

impl Validador for Cns<'_> {
    fn is_valid(numero: &str) -> bool {
        fn valida_1_2(numero: &str) -> bool {
            let multiplicadores1 = vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5];
            let mut digitos = somente_digitos(numero, 15);
            let dv = digitos.split_off(11);
            let mut soma = calc_digito(digitos, multiplicadores1, |x| x);
            let resto = soma % 11;
            let calculo = 11 - resto;
            let mut digito_anterior = 0;
            let calculo = if calculo == 11 {
                0
            } else if calculo == 10 {
                soma += 2;
                let resto = soma % 11;
                digito_anterior += 1;
                11 - resto
            } else {
                calculo
            };
            println!("{:?}", calculo);

            //retorno
            dv == vec![0, 0, digito_anterior, calculo]
        }
        fn valida_7_8_9(numero: &str) -> bool {
            let multiplicadores1 = vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
            let mut digitos = somente_digitos(numero, 15);

            completa_esquerda(&mut digitos, 15);
            let calculo = calc_digito(digitos, multiplicadores1, |x| x % 11);
            calculo == 0
        }

        let primeiro_digito = somente_digitos(numero, 1);
        match primeiro_digito.first() {
            Some(1) | Some(2) => valida_1_2(numero),
            Some(7) | Some(8) | Some(9) => valida_7_8_9(numero),
            _ => false,
        }
    }

    fn validar(&self) -> bool {
        Self::is_valid(self.0)
    }
}

/// Se for número provisório usa o número do Pis
impl Validador for TipoCns<'_> {
    fn validar(&self) -> bool {
        match self {
            TipoCns::Cns(value) => value.validar(),
            TipoCns::Provisorio(value) => value.validar(),
        }
    }
}

impl Validador for CodigoBarrasGs1<'_> {
    fn is_valid(numero: &str) -> bool {
        let multiplicadores1 = vec![3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
        let mut digitos = somente_digitos(numero, 18);
        completa_esquerda(&mut digitos, 18);
        let dv1 = &digitos.split_off(17);
        let digitos_verificadores =
            calc_digito(digitos, multiplicadores1, |x| (x / 10 + 1) * 10 - x);
        digitos_verificadores == dv1[0]
    }

    fn validar(&self) -> bool {
        Self::is_valid(self.0)
    }
}

impl Validador for Pis<'_> {
    fn is_valid(numero: &str) -> bool {
        let multiplicadores1 = vec![3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let mut digitos = somente_digitos(numero, 11);
        let dv1 = digitos.split_off(10);

        dv1[0] == calc_digito(digitos, multiplicadores1, onze_menos_mod11)
    }

    fn validar(&self) -> bool {
        Self::is_valid(self.0)
    }
}
