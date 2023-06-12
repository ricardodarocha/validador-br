# Guidelines para novas implementações

## 🔱 Como ajudar

* Acesse as issues e verifique as demandas da comunidade  
* Participe dos fóruns de discussão e contribua com as análises e boas práticas para implementação  
* Crie uma WIKI com exemplos de utilização  
* Crie uma WIKI com casos de uso e casos de sucesso  

## 📄 Implementando novos documentos

_Checklist_
- [ ] Defina um novo tipo
- [ ] Implemente o trait `Validador`
- [ ] Escreva um Teste Unitário **TDD**
- [ ] Implemente o algoritmo do dígito verificador **DV**
- [ ] Implemente o trait `TryFrom` 

**1)** Defina um novo tipo na unit _types.rs_

```diff 
+ pub struct NovoDocumento<'data>(pub &'data str);
```

**2)** Implemente o trait `Validador` na unit _types.rs_  para o tipo ciado   
_É importante que a validação retorne `false` até implementar os TDDs na etapa **[3]**_

```diff
impl Validador for NovoDocumento<'_> {
    fn is_valid(numero: &str) -> bool {
+       false
    }

    fn validar(&self) -> bool {
+       Self::is_valid(self.0)
    }
}
```

**3)** Escreva um Teste Unitário  na unit _tests.rs_  para o tipo criado 

```diff
+ #[test] 
+ fn test_validar_novo_documento() {
    //CASE 1
    assert!(NovoDocumento::is_valid("1111111-11"));
    //CASE 2
    assert!(NovoDocumento::is_valid("2222222-22"));
```

> Agora você está pronto para rodar o seu primeiro teste unitário antes de prosseguir  
> O objetivo é ver o teste falhar; só depois iremos implementar o algoritmo do dígito verificador  
> ```shell
> cargo test test_validar_novo_documento
> ```

**4)** Refatore o método `is_valid()` na unit _types.rs_ para implementar o método de validação do dígito verificador  
⚠ _**Não será aceita Pull Request sem execução do Teste Unitário**_

_O exemplo a seguir foi retirado do documento **Pis**, que possui um algoritmo simples com um dígito verificador. Utiliza os pesos 3298765432 e função de validação `onze_menos_modulo11`_

```diff
impl Validador for NovoDocumento<'_> {
    fn is_valid(numero: &str) -> bool {
-       false
+       const TAMANHO: usize = 11;
+       const QTD_DV: usize = 1;
+       let multiplicadores = vec![3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
+       let mut digitos = somente_digitos(numero, TAMANHO);
+       completa_esquerda(digitos, TAMANHO);
+       let digitos_verificadores = digitos.split_off(TAMANHO-QTD_DV);

+       digitos_verificadores[0] == calc_digito(digitos, multiplicadores, onze_menos_mod11)
    }
}
```

**5)** Implemente o trait `TryFrom` na unit _parser.rs_ para o tipo criado para permitir realizar o parse de maneira segura  

```diff
+ impl<'data> TryFrom<&'data str> for NovoDocumento<'data>{
+ type Error = &'static str;

+    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
         if Self::is_valid(value) {
            Ok(Self(value))        
        } else {
+            Err("Não foi possível validar o documento XX")
        }
    }
}
```

**6)** 

**Parabéns!** Você está pronto para realização do Pull Request!

## 🚀 Criação de novos Testes Unitários e Casos de Uso

Nós encorajamos ampliar a cobertura de Testes. Casos específicos ou casos raros devem ser implementados, como: 
 - documentos com dígito verificador 0
 - número de documentos com caracteres especiais
 - documentos com tamanho variável, etc
 
 Para implementar novos testes basta inserir uma função na unit _testes.rs_ adicionando a macro `#[test]`
 Implemente tantas funções quanto necessário para cada caso de uso encontrado.

## 💡 Criação de novas funcionalidades

Novas funcionalidades que estejam muito além da capacidade de validar ou realizar parse seguro devem ser discutidas no 
[Fórum de discussão](https://github.com/ricardodarocha/validador-br/discussions)

## 💛 Solicitar um novo tipo de documento

Crie uma nova issue com a solicitação. A comunidade irá avaliar a possibilidade de implementar. Importante dar o máximo de informações, 
adicionar links com a documentação técnica, exemplos e pelo menos uma dúzia de números de documento válidos para 
ser adicionado ao Teste

## ✅ Revisão de Código e Refatoração

Toda revisão de código ou refactoring é bem vinda, contando que os testes unitários não sofram alteração antes e após a refatoração
