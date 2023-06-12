use validador_br::funcoes::mod_11;
use validador_br::funcoes::{calc_digito, completa_esquerda, somente_digitos};
use validador_br::types::{CartaoCredito, Cnh, Cnpj, Cpf, Cns, Pis, Renavam, Rg, 
    TituloEleitor, CodigoBarrasGs1,
};
use validador_br::validador::Validador;

#[test]
fn test_mod_11() {
    let test_data: [(u32, usize); 29] = [
        (810, 7),
        (820, 6),
        (830, 5),
        (840, 4),
        (850, 3),
        (860, 2),
        (870, 1),
        (880, 0),
        (890, 0),
        (900, 9),
        (910, 8),
        (920, 7),
        (930, 6),
        (940, 5),
        (950, 4),
        (960, 3),
        (970, 2),
        (980, 1),
        (990, 0),
        (1000, 0),
        (1010, 9),
        (1020, 8),
        (1030, 7),
        (1040, 6),
        (1050, 5),
        (1060, 4),
        (1070, 3),
        (1080, 2),
        (2510, 2),
    ];

    let test_results: Vec<bool> = test_data
        .iter()
        .map(|&(input, expected_output)| mod_11(input) == expected_output)
        .collect();
    assert!(test_results.iter().all(|&test_passed| test_passed));
}

#[test]
fn test_calc_digito() {
    
    fn _test(input: &str, multiplicadores: Vec<u32>) {
        let split_size: usize = multiplicadores.len();
        let mut digitos: Vec<u32> = somente_digitos(input, split_size + 1);
        let output: Vec<u32> = digitos.split_off(split_size);
        let result: u32 = calc_digito(digitos, multiplicadores, |x: u32| mod_11(10 * x) as u32);
        assert_eq!(result, output[0]);
    }

    let test_all: Vec<&str> = vec![
        "1556748620",
        "0155674862",
        "26.911.358-70",
        "326.911.358-7",
        "883.926.778-6",
        "83.926.778-62",
        "0155674862"
    ];

    fn gerar_multiplicadores(start: u32, end: u32) -> Vec<u32> {
        (start..=end).rev().collect()
    }

    let multiplicadores: Vec<u32> = gerar_multiplicadores(2, 10);

    for input in &test_all {
        _test(input, multiplicadores.clone());
    }
}
#[test]
fn test_completa_esquerda() {
    fn test_0123() {
        let mut value = vec![1, 2, 3];
        completa_esquerda(&mut value, 4);
        assert!(value.clone() == vec![0, 1, 2, 3]);
    }
    fn test_0007522413915() {
        let mut value = vec![7, 5, 2, 2, 4, 1, 3, 9, 1, 5];
        completa_esquerda(&mut value, 13);
        assert!(value.clone() == vec![0, 0, 0, 7, 5, 2, 2, 4, 1, 3, 9, 1, 5]);
    }
    fn test_789() {
        let mut value = vec![7, 8, 9];
        completa_esquerda(&mut value, 3);
        assert!(value.clone() == vec![7, 8, 9]);
    }
    fn test_7890000() {
        let mut value = vec![7, 8, 9, 0, 0, 0, 0];
        completa_esquerda(&mut value, 7);
        assert!(value.clone() == vec![7, 8, 9, 0, 0, 0, 0]);
    }
    fn test_0078900() {
        let mut value = vec![0, 0, 7, 8, 9, 0, 0];
        completa_esquerda(&mut value, 8);
        assert!(value.clone() == vec![0, 0, 0, 7, 8, 9, 0, 0]);
    }
    fn test_x13915() {
        let mut value = vec![0, 0, 0, 7, 5, 2, 2, 4, 1, 3, 9, 1, 5];
        completa_esquerda(&mut value, 5);
        assert!(value.clone() == vec![0, 0, 0, 7, 5, 2, 2, 4, 1, 3, 9, 1, 5]);
    }
    test_0123();
    test_0007522413915();
    test_789();
    test_7890000();
    test_0078900();
    test_x13915();
}

#[test]
fn test_somente_digitos() {
    let tests: Vec<(&str, Vec<u32>)> = vec![
        ("0", vec![0]),
        ("1", vec![1]),
        ("5", vec![5]),
        ("9", vec![9]),
        ("10", vec![1, 0]),
        ("123", vec![1, 2, 3]),
        ("1.23-0", vec![1, 2, 3, 0]),
        ("1.23/001-0", vec![1, 2, 3, 0, 0, 1, 0]),
    ];

    for (input, expected) in tests {
        let result = somente_digitos(input, input.len());
        assert_eq!(result, expected);
    }
}

#[test]
fn test_isvalid_cpf() {
    let cpfs: Vec<&str> = vec![
        "085.668.830-47",
        "712.926.512-45",
        "974.749.266-01",
        "201.859.154-18",
        "274.047.203-03",
        "864.148.641-02"
    ];

    for numero in &cpfs {
        assert!(Cpf::is_valid(numero));
    }}


#[test]
fn test_valid_cpf() {
    let cpfs: Vec<&str> = vec![
        "085.668.830-47",
        "712.926.512-45",
        "974.749.266-01",
        "201.859.154-18",
        "274.047.203-03",
        "864.148.641-02"
    ];

    for cpf in &cpfs {
        assert!(Cpf(cpf).validar());
    }
}

#[test]
fn test_valid_cnpj() {
    let cnpjs: Vec<&str> = vec![
        "14.572.457.0001-85",
        "76.553.412/0001-10",
        "45.184.338/0001-89",
        "37.645.328/0001-75",
        "55.131.839/0001-50",
        "44.122.045/0001-04",
        "41.768.737/0001-36",
        "89.267.553/0001-19",
        "40.357.317/0001-02",
        "46.922.782/0001-17",
        "86.107.547/0001-06"
    ];

    for cnpj in &cnpjs {
        assert!(Cnpj(cnpj).validar());
    }
}

#[test]
fn test_valid_cc() {
    let cards: Vec<&str> = vec![
        "5312 8338 4531 6765",
        "5440 4970 4224 4678",
        "6011 1122 7847 6822",
        "3479 467653 71543",
        "2014 4799698 0942",
        "6062 8271 2873 9719"
    ];

    for credit_card in &cards {
        assert!(CartaoCredito(credit_card).validar());
    }
}

#[test]
fn test_valid_titulo_eleitor() {
    let tittles: Vec<&str> = vec![
        "00435687 09-06",
        "781613810175",
        "8645582519 29",
        "272.20357 20-20"
    ];

    for tittle in &tittles {
        assert!(TituloEleitor(tittle).validar());
    }
}

#[test]
fn test_valid_cnh() {
    let cnhs: Vec<&str> = vec![
        "81814756744",
        "28851304391",
        "72887311600",
        "86033265137",
        "40386768760"
    ];

    for cnh in &cnhs {
        assert!(Cnh(cnh).validar());
    }
}

#[test]
fn test_valid_renavam() {
    assert!(Renavam(&"55879397036").validar());
    assert!(Renavam(&"31264299604").validar());
    assert!(Renavam(&"26739874261").validar());
    assert!(Renavam(&"18946231365").validar());
    assert!(Renavam(&"0269.417.0174").validar());
    assert!(Renavam(&"2694170174").validar());
    assert!(Renavam(&"61631697260").validar());
    assert!(Renavam(&"03340530540").validar());
    assert!(Renavam(&"3340530540").validar());
    assert!(Renavam(&"891353364").validar());
    assert!(Renavam(&"MG48.101 477-2").validar());
}

#[test]
fn test_valid_rg() {
    let rgs: Vec<&str> = vec![
        "14.176.381-4",
        "28.530.378-8",
        "28.346.393-4",
        "39.371.494-9",
        "24.657.777-0",
        "17.227.771-1"
    ];

    for rg in &rgs {
        assert!(Rg(rg).validar());
    }
}

#[test]
fn test_valid_cns() {
    assert!(Cns(&"184184462180018").validar());
    assert!(Cns(&"140387139970001").validar());
    assert!(Cns(&"178095857640000").validar());
    assert!(Cns(&"962183461450002").validar());
    assert!(Cns(&"189444452860006").validar());
    assert!(Cns(&"107254212750009").validar());
    assert!(Cns(&"233954785680009").validar());
    assert!(Cns(&"163273888290018").validar());
    assert!(Cns(&"207066918130005").validar());
    assert!(Cns(&"852892869900003").validar());
    assert!(Cns(&"720641595950005").validar());
    assert!(Cns(&"294490488830009").validar());
    assert!(Cns(&"864468656740001").validar());
    assert!(Cns(&"171034974690006").validar());
    assert!(Cns(&"111059690060009").validar());
    assert!(Cns(&"734512532720004").validar());
    assert!(Cns(&"274579054310008").validar());
    assert!(Cns(&"244688881450002").validar());
    assert!(Cns(&"701996683420003").validar());
    assert!(Cns(&"118934023320006").validar());
    assert!(Cns(&"220780617770000").validar());
    assert!(Cns(&"213826540730001").validar());
    assert!(Cns(&"281440564810008").validar());
    assert!(Cns(&"241293038630002").validar());
    assert!(Cns(&"186487812210006").validar());
    assert!(Cns(&"269472194400008").validar());
    assert!(Cns(&"816685676560004").validar());
    assert!(Cns(&"188643188220000").validar());
    assert!(Cns(&"703828335240002").validar());
    assert!(Cns(&"858681602500018").validar());
    assert!(Cns(&"181068087580004").validar());
    assert!(Cns(&"169391037690002").validar());
    assert!(Cns(&"216015350640002").validar());
    assert!(Cns(&"855561772500007").validar());
    assert!(Cns(&"171581061380018").validar());
    assert!(Cns(&"141674939420009").validar());
    assert!(Cns(&"764078680590003").validar());
    assert!(Cns(&"272101781280001").validar());
    assert!(Cns(&"792869204500009").validar());
    assert!(Cns(&"284830935640009").validar());
    assert!(Cns(&"149261133080003").validar());
    assert!(Cns(&"851068307450018").validar());
    assert!(Cns(&"158482206870007").validar());
    assert!(Cns(&"190191624780009").validar());
    assert!(Cns(&"871946533020007").validar());
    assert!(Cns(&"839286321970004").validar());
    assert!(Cns(&"268240267290004").validar());
    assert!(Cns(&"257633494510009").validar());
    assert!(Cns(&"191619409830006").validar());
    assert!(Cns(&"282467243710003").validar());
}

#[test]
fn test_valid_pis() {
    let pises: Vec<&str> = vec![
        "608.37951.54-6",
        "317.73180.80-0"
    ];

    for pis in &pises {
        assert!(Pis(pis).validar());
    }
}

#[test]
fn test_valid_codigobar() {
    //EAN 8
    assert!(CodigoBarrasGs1(&"7891234-2").validar());
    //EAN 11
    assert!(CodigoBarrasGs1(&"12345 6000-12").validar());
    //EAN 12
    assert!(CodigoBarrasGs1(&"614141007349").validar());
    assert!(CodigoBarrasGs1(&"811571013579").validar());
    //EAN 13
    assert!(CodigoBarrasGs1(&"4012345678901").validar());
    assert!(CodigoBarrasGs1(&"5901234123457").validar()); 
    assert!(CodigoBarrasGs1(&"1234567 890128").validar()); 
    assert!(CodigoBarrasGs1(&"7350053850033").validar()); 
    assert!(CodigoBarrasGs1(&"1312345123456 6").validar());
    assert!(CodigoBarrasGs1(&"789030058465-1").validar());
    assert!(CodigoBarrasGs1(&"789123456789-5").validar());
    assert!(CodigoBarrasGs1(&"789100031550-7").validar());
    //EAN 14
    assert!(CodigoBarrasGs1(&"1789835741001-2").validar());
    assert!(CodigoBarrasGs1(&"1 7891234 12345 6").validar());
    assert!(CodigoBarrasGs1(&"18496578214564").validar());
    //EAN 18
    assert!(CodigoBarrasGs1(&"376104250021234569").validar());
    assert!(CodigoBarrasGs1(&"376.10425002.123456-9").validar());
}

#[test]
#[allow(irrefutable_let_patterns)]
fn test_parse_cpf() {
    if let cpf = Cpf::try_from("085.668.830-47").unwrap() {
        assert!(cpf.validar());
    } else {
        unreachable!();
    };

    if let Ok(cpf) = Cpf::try_from("085.668.830-47") {
        assert!(cpf.validar());
    };

    let cpf: Result<Cpf, &'static str> = "085.668.830-47".try_into();
    match cpf {
        Ok(_cpf) => (),
        Err(_cpf_error) => unreachable!()
    };

    let cpf = Cpf::try_from("085.668.830-47");
    match cpf {
        Ok(_cpf) => (),
        Err(_cpf_error) => unreachable!()
    };
}
