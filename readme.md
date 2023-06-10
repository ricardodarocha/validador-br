# validador-br

_Brazilian document numbers validator written in rust_  
Validador para diversos documentos nacionais (BR) em rust.

[![Crates.io](https://img.shields.io/crates/v/validador-br)](https://crates.io/crates/validador-br) &bull; [![Crates.io](https://img.shields.io/crates/l/validador-br)](https://github.com/ricardodarocha/validador-br/blob/master/LICENSE) &bull; [![Build Status](https://travis-ci.com/ricardodarocha/validador-br.svg?branch=master)](https://travis-ci.com/ricardodarocha/validador) &bull; [![API](https://docs.rs/validador/badge.svg)](https://docs.rs/validador)

## Documentos implementados

```rust
struct Cpf(&str); ✔
struct Cnpj(&str); ✔
struct CartaoCredito(&str); ✔
struct TituloEleitor(&str); ✔
struct Cnh(&str); ✔
struct Rg(&str); ✔ ⚠ ///verificar a legislação em seu Estado
struct Pis(&str); ✔
struct Ie(Uf, &str); ❌
```

## Roadmap

- Implementar os documentos mais comuns ✅
- Ignora caracteres especiais, espaços e pontuações ✅
- Emitir um erro `panic!` caso o número de dígitos esteja incorreto ✅
- Valida antecipadamente a quantidade de dígitos numéricos. `000.000.00_.XX` ❌
- Analisar repetições de dígitos como `111.111.111-11` ou `000.000.000-00` ❌
- Validar inscrição estadual de cada estado ❌
- Implementar novos tipos de documentos ❌

## Instalação

Adicione a dependência ao seu `Cargo.toml`:

```toml
[dependencies]
validador-br = "0.1.2"
```

## Uso básico

```rust
use validador_br::validador::*;


Cpf("255.248.930-33").validar();
// ✅ true 

Cpf("25524893033").validar();
// ✅ true

Cnh("25524893033").validar();
// ❌ false

```

## Validando uma lista de cpfs

```rust
use validador_br::validador::*;

fn main() {
    let cpf_list = [&"133.976.410-55", &"922.261.830-00", &"922.261.830-01", &"218.571.960-23"];
    for num in cpf_list {
        if Cpf(num).validar() {
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
if Cpf(cpf_string.as_str()).validar() {..}
```

## Licença

**validador-br** é um software Open Source [licenciado pelo MIT](https://github.com/ricardodarocha/validador-br/blob/master/LICENSE)
