# Knowledge project


<p>
Construção de uma CLI em rust, a fim de aprender o básico da linguagem e de suas convenções.</b></b>
O projeto conhecimento visa salvar, atualizar, buscar e deletar os conhecimentos de uma pessoa sobre git, docker, php e node, por enquanto. </b> 
Os conhecimentos podem ser o que preferir, comandos, problemas e suas resoluções, as dificuldades que passou, o tempo que levou, etc.

</p>

### Tecnologias

RUST | DIESEL


### Executar

1. Após o git clone, entre na pasta:

```bash
cd rust_knowledge_project_cli
```

2. Para o build e já executar o sistema:

```bash
cargo run --bin rk -- -h
```

3. Para executar os tests:

```bash
cargo test
```

4. Exemplos de comandos:

- lista as categorias disponíveis 
```bash
cargo run --bin rk -- -l categorias
```

- lista os conhecimentos adicionados ao sistema:
```bash
cargo run --bin rk -- -l conhecimentos -t conhecimentos_output.txt
```
- adiciona um novo conhecimento, "hello world em node", no sistema na categoria node:
```bash
cargo run --bin rk -- -c node "hello world em node" 
```
- atualiza o conhecimento de id 1, para a categoria php com a descrição "hello world em php, echo \"hello world! \" ";
```bash
cargo run --bin rk -- -u 1 php "hello world em php, echo \"hello world! \" "
```
- procurar dentro da categoria node, os conhecimentos que possuem essa descrição:
```bash
cargo run --bin rk -- -r node "hello world" 
```
- list todos os conhecimentos da categoria git
```bash
cargo run --bin rk -- -r git
```
- procuta o conhecimento com determinado <id>
```bash
cargo run --bin rk -- -r <id>
```
- deletar o conhecimento com <id>
```bash
cargo run --bin rk -- -d <id>
```
