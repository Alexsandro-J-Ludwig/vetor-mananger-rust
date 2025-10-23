# vetor-manipulation-rust

## Descrição do Projeto de Estudos
Este é um projeto feito com o objetivo de aprender e aplicar conceitos próprios do Rust, com foco em manipualçao de vetores(arrays dinâmicos) e com um sistema de interação com usuário. Este sistema é com objetivo de treinar e não possui validação de variáveis

## Conceitos Aplicadados
- 🏠 **Ownership**: O rust funciona como uma proprioedade privada onde cada variavel é um apartamento próprio e pertence a apenas uma pessoa, sendo necessário emprestar sua casa para que outras pessoas possam usar.
- 🎁 **Bowwowing**: Conceito semelhante mas é o ato de emprestar uma váriavel para outra, por exemplo, você emprestou seu carro para seu amigo, mas você quer tentar modificar seu carro, porém não pode fazer isso sem antes pegar de volta de seu amigo.
- 🖩 **Pattern Matching**: É o conceito de ```match``` sendo esse uma versão mais poderosa que switch/cases tradicionais de outras linguagens sendo o mais apropriado para multiplas opções de uma mesma variável semelhantes.
- ❌ **Result e Option**: No Rust, os conceitos de entrada de váriaveis e conversão devem ser feitos utilizando os ```.unwrap()```, ```.expect()``` ou até mesmo o ```match``` para capturar possíveis erros, sendo este uma versão do catch para vriaveis.
- 📏 **Vetores**: Vetores são de ultima análise, um tipo de array em Rust, sendo estes sua variante dinâmica sem um tamanho pré-definido, são declaradas como ```vec![]```, sua tipagem pode ser ou não definida, caso não seja definida na hora da declaração da váriavel, será inferida baseada no primeiro elemento.
- 🔁 **loop**: São uma forma while porém sem a necessidade de uma comparação, podendo ser infinito, seu conceito pode ser copiado em outras linguagens como ```while(true)```.

## Aprimoramento futuros no projeto de estudos
- [ ] - Escolher um determinado tipo de sort, sendo trabnalhado os conceitos de O(n) em Rust para treinamento de lógica em baixo nível.
- [ ] - Validação de entrada de dados por parte do usuário.
- [ ] - banchmark do tempo de organização de cada sort.

## Como aplicar este projeto?
Há duas formas de rodar este projeto.

### Via clone no seu IDE de preferência
  1. Tenha as ferramentas próprias do Rust para desenvolvimento instalados no seu desktop, caso não o possua, acesse o link do passoa a passo para instalar o Rust em seu desktop: https://rust-lang.org/pt-BR/tools/install/
  2. Abra o terminal e escreva git clone "https://github.com/Alexsandro-J-Ludwig/vetor-manipulation-rust".
  4. Use o comando ```cd vetor-manipulation-rust``` para ser direcionado a pasta do projeto.

### Via Rust Playground
  Usar esta aplicação no Rust Playground é possível e simples:
  1. Acesse https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=74dbea87650b20aeb74c02d1b9695edf e basta rodar.
