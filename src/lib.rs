#![doc=include_str!("../readme.md")]
pub mod types;
pub mod validador_ie;
pub mod funcoes;    
pub mod validador;
pub mod parser;

pub use types::*;
pub use validador::Validador;