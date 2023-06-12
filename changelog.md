# Changelog

Qualquer alteração notável deve ser documentada neste arquivo

## [0.1.3] - 2023-06-12

- [x] Implementado trait `try_from()` `try_into()`
- [x] Os módulos foram separados em arquivos  

## [0.1.2] - 2023-06-11

- Adicionados novos tipos de documentos

```diff
+ pub struct Cns<'data>(pub &'data str);
+ pub struct Renavam<'data>(pub &'data str);
+ pub struct CodigoBarrasGs1<'data>(pub &'data str); // EAN8 EAN13 GTIN08..18
```

## [0.1.1] - 2023-06-10

- Mudança de lifetimes para aceitar Strings dinâmicas

```diff
- pub struct Cpf(pub &'static str);
+ pub struct Cpf<'data>(pub &'data str);
```

O código abaixo passa a funcionar

```rust
let numero = String::from("133.976.410-55");
if Cpf(numero).validar() {..}
```

## [0.1.0] - 2023-06-10

- Primeira versão publicada
