use crate::validador::{Validador};
use crate::types::{CartaoCredito, Cnh, Cnpj, Cpf, Cns, Pis, Renavam, Rg, 
    TituloEleitor, CodigoBarrasGs1,
};

impl<'data> TryFrom<&'data str> for CartaoCredito<'data>{
type Error = &'static str;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
            Err("Cartão de crédito inválido")
        }
    }
}

impl<'data> TryFrom<&'data str> for Cpf<'data>{
type Error = &'static str;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
            Err("Cpf inválido")
        }
    }
}

// impl<'data> TryFrom<&'data String> for Cpf<'data>{
// type Error = &'static str;

// fn try_from(value: &'data String) -> Result<Self, Self::Error> {
//     if Cpf::is_valid(&value) {
//         Ok(Self(value))        
//     } else {
//         Err("Cpf inválido")
//     }
// }
// }

impl<'data> TryFrom<&'data str> for Cnh<'data>{
type Error = &'static str;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
            Err("Carteira de motorista inválida")
        }
    }
}

impl<'data> TryFrom<&'data str> for Cnpj<'data>{
type Error = &'static str;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
            Err("cnpj inválido")
        }
    }
}

impl<'data> TryFrom<&'data str> for Cns<'data>{
type Error = &'static str;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
            Err("Cartão de saúde inválido")
        }
    }
}

impl<'data> TryFrom<&'data str> for Pis<'data>{
type Error = &'static str;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
            Err("Número do Pis inválido")
        }
    }
}

impl<'data> TryFrom<&'data str> for Renavam<'data>{
type Error = &'static str;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
            Err("Número do Renavam inválido")
        }
    }
}

impl<'data> TryFrom<&'data str> for Rg<'data>{
type Error = &'static str;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
            Err("Não foi possível validar o número de identidade. Verifique a legislação do seu Estado")
        }
    }
}

impl<'data> TryFrom<&'data str> for TituloEleitor<'data>{
type Error = &'static str;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
            Err("Número do Título de Eleitor inválido")
        }
    }
}

impl<'data> TryFrom<&'data str> for CodigoBarrasGs1<'data>{
type Error = &'static str;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
            Err("Não foi possível validar o dígito verificador do código de barras")
        }
    }
}