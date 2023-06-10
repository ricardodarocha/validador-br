# Changelog
Qualquer alteração not[avelAll notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [0.1.1] - 2023-06-10

- Mudança de lifetimes para aceitar Strings dinâmicas 

```diff
+ pub struct Cpf(pub &'static str);
- pub struct Cpf<'data>(pub &'data str);
```

O código abaixo passa a funcionar

```rust
let numero = String::from("133.976.410-55");
if Cpf(numero).validar() {..}
```

## [0.1.0] - 2023-06-10

- Primeira versão experimental 
