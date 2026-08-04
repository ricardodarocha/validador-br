<div align="center">
  <h1>💚💙💛 Validador-br</h1>

  <i>A Brazilian document numbers validator written in rust</i>
  <h3>Um validador para diversos documentos nacionais (BR) em rust </h3>
  
</div>

[![Crates.io](https://img.shields.io/crates/v/validador-br)](https://crates.io/crates/validador-br)
[![Crates.io](https://img.shields.io/crates/l/validador-br)](https://github.com/ricardodarocha/validador-br/blob/master/LICENSE)
[![API](https://docs.rs/validador-br/badge.svg)](https://docs.rs/validador-br)
[![Fórum de discussão](https://img.shields.io/github/issues/ricardodarocha/validador-br?label=tarefas)](https://github.com/ricardodarocha/validador-br/discussions)
<!-- [![Build Status](https://travis-ci.com/ricardodarocha/validador-br.svg?branch=master)](https://travis-ci.com/ricardodarocha/validador-br) &bull; -->

[![GitHub stars](https://img.shields.io/github/stars/ricardodarocha/validador-br?style=social&label=Star&maxAge=1)](https://github.com/ricardodarocha/validador-br/stargazers/)
[Contribua](CONTRIBUTING.md) comentando, compartilhando ou realizando um pull request!

## [0.1.6] - 2026-07-04

- [x] Implementado DV do código RENAVAM

## [0.1.5] - 2026-07-04

- [x] Remoção do namespace types::*. Acesse os documentos diretamente `use validador-br::Cnpj`

## [0.1.4] - 2026-07-03

- [x] Implementar suporte ao CNPJ alfanumérico

## [0.1.3] - 2023-06-12

- [x] Implementado trait `try_from()` `try_into()`
- [x] Os módulos foram separados em arquivos  

## [0.1.2] - 2023-06-11

- [x] Adicionados novos tipos de documentos

```rust ignore
pub struct Cns<'data>(pub &'data str);
pub struct Renavam<'data>(pub &'data str);
pub struct CodigoBarrasGs1<'data>(pub &'data str); // EAN8 EAN13 GTIN08..18
```

## [0.1.1] - 2023-06-10

```rust ignore
struct Cpf(&str); ✔
struct Cnpj(&str); ✔
struct CartaoCredito(&str); ✔
struct TituloEleitor(&str); ✔
struct Cnh(&str); ✔
struct Rg(&str); ✔ ⚠ ///verificar a legislação em seu Estado
struct Pis(&str); ✔
struct Ie(Uf, &str); ❌
```

Consulte o [change log](https://github.com/ricardodarocha/validador-br/blob/master/changelog.md) para mais informações.

## Roadmap

- Implementar os documentos mais comuns ✅
- Ignorar caracteres especiais, espaços e pontuações ✅
- Emitir um erro `panic!` caso o número de dígitos esteja incorreto ✅
- Validar antecipadamente a quantidade de dígitos numéricos. `000.000.00_.XX` ❌
- Analisar repetições de dígitos como `111.111.111-11` ou `000.000.000-00` ❌
- Validar inscrição estadual de cada estado ❌
- Implementar novos tipos de documentos ❌

## Instalação

Adicione a dependência ao seu `Cargo.toml`:

```toml
[dependencies]
validador-br = "0.1.4"
```

## Uso básico

```rust
use validador_br::{Cpf, Cnpj, Cnh};

let cpf = Cpf::try_from("085.668.830-47");
match cpf {
    Ok(cpf) => println!("{}✅", cpf.0),
    Err(invalido) => panic!("{}", invalido)
};

let cnpj = Cnpj::try_from("90.33N.F02/0001-88");
match cnpj {
    Ok(cnpj) => println!("{}✅", cnpj.0),
    Err(invalido) => panic!("{}", invalido)
};
```

Também é possível usar o método `validar()`.

```rust
use validador_br::Validador;
Cpf("255.248.930-33").validar(); // ✅ true 
Cpf("25524893033").validar();// ✅ true
Cnh("25524893033").validar();// ❌ false

```

## Validando uma lista de cpfs

```rust

fn main() {
    let cpf_list = [&"133.976.410-55", &"922.261.830-00", &"922.261.830-01", &"218.571.960-23"];
    for num in cpf_list {
        if Cpf::is_valid(num) {
            println!("{} ✅", num)
        } else {
            println!("{} ❌", num)
        }
    }
}
```

## Utilizando Strings dinâmicas

```rust
let cpf_string = String::from("133.976.410-55");
if Cpf(cpf_string.as_str()).validar() {}
```

## Licença

**validador-br** é um software Open Source [licenciado pelo MIT](https://github.com/ricardodarocha/validador-br/blob/master/LICENSE)
