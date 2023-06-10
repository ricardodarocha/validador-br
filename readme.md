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

- Implementar os documentos mais comuns. ✅
- Ignora caracteres especiais, espaços e pontuações. ✅
- Valida antecipadamente a quantidade de dígitos numéricos. ❌
- Analisas repetições de dígitos como `111.111.111-11` ou `000.000.000-00`. ❌
- Validar inscrição estadual de cada estado. ❌
- Implementar novos tipos de documentos. ❌

## Instalação

Adicione essa dependência no seu `Cargo.toml`:

```toml
[dependencies]
validador-br = "0.1"
```

## Uso básico

```rust
use validador_br::{Cpf, Cnh};


Cpf("255.248.930-33").validate();
// ✅ true 

Cpf("25524893033").validate();
// ✅ true

Cnh("25524893033").validate();
// ❌ false

```

## Licença

**validador-br** é um software Open Source [licenciado pelo MIT](https://github.com/ricardodarocha/validador-br/blob/master/LICENSE)
