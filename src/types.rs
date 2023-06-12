use crate::validador_ie::IeUf;
/// Cadastro de Pessoa Física
pub struct Cpf<'data>(pub &'data str);
/// Cadastro de Pessoa Jurídica
pub struct Cnpj<'data>(pub &'data str);
/// Inscrição Estadual
#[allow(dead_code)]
pub struct Ie<'data>(pub IeUf<'data>);
/// Cartão de Crédito
pub struct CartaoCredito<'data>(pub &'data str);
/// Título de Eleitor
pub struct TituloEleitor<'data>(pub &'data str);
/// Carteira Nacional de Habilitação (Motorista)
pub struct Cnh<'data>(pub &'data str);
/// Registro Nacional de Veículos Automotores
pub struct Renavam<'data>(pub &'data str);
/// Registro Geral, Cédula de Identidade
pub struct Rg<'data>(pub &'data str);
/// Cartão Nacional de Saúde
pub struct Cns<'data>(pub &'data str);
pub enum TipoCns<'data> {
    Cns(Cns<'data>),
    Provisorio(Pis<'data>),
}
/// Código de Barras Gs1 7890300584651
pub struct CodigoBarrasGs1<'data>(pub &'data str);
/// Programa de Integração Social, PIS, PASEP, NIT
pub struct Pis<'data>(pub &'data str);