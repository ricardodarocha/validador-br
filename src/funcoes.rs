//! Define métodos que são usados com frequência pelos algoritmos de verificação, como *cálculo do dígito verificador*, *cálculo usando mod11*, `somente_digitos`

/// Define algumas funções básicas como `módulo_11`
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

/// Dada uma sequência de n dígitos qualquer, verificar se o último dígito corresponde ao módulo 11 dos demais para os multiplicadores dados
/// ⚠ Importante observar a particularidade de cada doc.
/// Por exemplo:
/// * CPF os multiplicadores são sempre (10..=2) sendo dv1 função dos dígitos [0..9] e dv2 função dos dígitos [1..10]
/// * IE os multiplicadores variam de estado para estado sendo dv1 = os dígitos [0..n-1] e dv2 = os dígitos [0..n]

/// Definimos um método genérico para calcular o dígito verificador de um doc, recebendo como parâmetro um vetor com os multiplicadores
/// Este método **não** calculará mod 11 automaticamente, devendo ser passada por parâmetro uma função ou closure para realizar a verificação
///```rust
/// # use validador_br::funcoes::calc_digito;
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
pub fn somente_digitos(doc: &str, n: usize) -> Vec<u32> {
    doc.chars()
        .filter(|c| c.is_ascii_digit())
        .map(|x| x.to_digit(10).unwrap())
        .take(n)
        .collect()
}

pub fn completa_esquerda(numero: &mut Vec<u32>, n: usize) {
    if numero.len() < n {
        for _ in 0..(n - numero.len()) {
            numero.insert(0, 0);
        }
    };
    // println!("{:?}", numero);
}