use std::{env, process};
use std::str::FromStr;

use crate::routes::*;
use crate::models::{CategoriasEnum, ErrorApp};
use crate::models::TypeErro::{InvalidArguments, InvalidCommand, Others, UninitializedSeed};
use crate::options::list::all_categories;

pub mod options;
pub mod db;
pub mod schema;
pub mod models;
pub mod routes;
pub mod output;
pub mod tests;

pub fn start() {

    println!("\n");
    let args:Vec<String> = env::args().collect();

    run(args).unwrap_or_else(|err| {
        eprintln!("\n");
        match err.type_error {
            InvalidCommand => {
                eprintln!("Comando inválido");
                eprintln!("{}",err.msg);
            }
            InvalidArguments => {
                eprintln!("Argumentos inválidos, você quis dizer:");
                eprintln!("{}",err.msg);
            }
            UninitializedSeed => {
                eprintln!("{}",err.msg);
            }
            Others => {
                eprintln!("{}",err.msg);
            }
        }

        eprintln!("\n");

        process::exit(1);
    });
    println!("\n");
    process::exit(0);
}

fn run(args: Vec<String>) -> Result<bool, ErrorApp> {

    match all_categories() {
        Ok(item) => {
            if item.len() == 0 {
                let _msg = "Nenhuma categoria adicionada ainda. \n\
        Execute o comando abaixo, para executar o seed: \n\n\
        \t cargo run --bin seed \n\n";

                let _error = ErrorApp {
                    type_error: UninitializedSeed,
                    msg: _msg.to_string()
                };
                return Err(_error);
            }
        }
        Err(_err) => { eprintln!("Erro ao validar execução: {:?}", _err) }
    }

    if args.len() <2 {
        let _error = ErrorApp {
            type_error: InvalidCommand,
            msg: help_message()
        };

        return Err(_error);
    }

    let start_command = &args[1];

    match start_command.as_str() {
        "-c" => {
            if args.len() != 4 {
                let categories = available_categories();
                let msg = format!("
  -c <categoria> <descrição>\n\n
  {}

  Sugestões para descrição do novo conhecimento: \n
   - Qual era o problema ?\n
   - Como conseguiu resolver ?\n
   - Tempo que levou ?\n
   - Qual foi a dificuldade ? \n", categories).to_string();

                let _error = ErrorApp {
                    type_error: InvalidArguments,
                    msg
                };

                return Err(_error);
            }
            create(&args)?;
        }
        "-u" => {
            if args.len() != 5 {
                let categories = available_categories();
                let msg = format!("
  -u <id> <categoria> <descrição>\n\n
  {}
  Sugestões para descrição do novo conhecimento: \n
   - Qual era o problema ?\n
   - Como conseguiu resolver ?\n
   - Tempo que levou ?\n
   - Qual foi a dificuldade ? \n", categories).to_string();

                let _error = ErrorApp {
                    type_error: InvalidArguments,
                    msg
                };

                return Err(_error);
            }
            update(&args)?;
        }
        "-d" => {
            if args.len() != 3 {
                let msg = "
  -d <id>\n".to_string();

                let _error = ErrorApp {
                    type_error: InvalidArguments,
                    msg
                };

                return Err(_error);
            }
            delete(&args)?;
        }
        "-r" => {
            if args.len() == 3 || args.len() == 5 {
                let value = &args[2];

                if CategoriasEnum::from_str(value).is_ok() {
                    read_category(&args)?;
                } else {
                    read_by_id(&args)?;
                }
            }
            else if args.len() == 4 || args.len() == 6 {
                read_by_description_and_category(&args)?;
            }
            else {
                let categories = available_categories();
                let msg = format!("
  -r <id>                                \n
  -r <categoria> <?opcional?>                 \n
  -r <categoria> <descrição> <?opcional?>             \n

  opcional
    -t <file name>
  {}", categories).to_string();

                let _error = ErrorApp {
                    type_error: InvalidArguments,
                    msg
                };

                return Err(_error);
            }
        }
        "-l" => {
            if args.len() != 3 && args.len() != 5 {

                let msg = "
  -l categorias <?opcional?>\n
  -l conhecimentos <?opcional?>\n

   opcionais:
    -t <file name>
   \n".to_string();

                let _error = ErrorApp {
                    type_error: InvalidArguments,
                    msg
                };

                return Err(_error);
            }
            let _type = &args[2];

            match _type.as_str() {
                "categorias" => { list_categories(&args)?; }
                "conhecimentos" => { list_knowledge(&args)?; }
                _ => { let categories = available_categories();
                let msg = format!("
  -c <categoria> <descrição>\n\n
  {}

  Sugestões para descrição do novo conhecimento: \n
   - Qual era o problema ?\n
   - Como conseguiu resolver ?\n
   - Tempo que levou ?\n
   - Qual foi a dificuldade ? \n", categories).to_string();

                    let _error = ErrorApp {
                        type_error: InvalidArguments,
                        msg
                    };

                    return Err(_error); }
            };
        }
        "-h" => {
            println!("{}", help_message());
        }
        _ => {

            let _error = ErrorApp {
                type_error: InvalidCommand,
                msg: help_message()
            };

            return Err(_error);
        }
    }

    return Ok(true);
}

fn help_message() -> String
{
    let categories = available_categories();


    return format!("\n\nPara usar:\n\n
  -c <categoria> <descrição>                   adiciona um novo conhecimento, verifique as \
  categorias disponíveis\n
  -r <id>                                      pega um conhecimento pelo <id>\n
  -r <categoria> <?opcional?>                  pega todos os conhecimentos de acordo com a \
  <categoria>\n
  -r <categoria> <descrição> <?opcional?>      pega todos os conhecimentos de acordo com a \
  <categoria>\
   e \
  alguma breve <descrição>\n
  -u <id> <categoria> <descrição>              atualiza a <categoria> e a <descrição>, de acordo \
  com \
  <id>\n
  -d <id>                                      deleta um conhecimento, de acordo com <id>\n
  -l conhecimentos <?opcional?>                lista todos os conhecimentos independentes da \
  categoria\n
  -l categorias <?opcional?>                   lista todas as categorias disponíveis no sistema\n
  -h                                           lista os comandos\n\n

opcionais:

  -t <file name>


{}", categories);


}

fn available_categories() -> String
{
    let categories = CategoriasEnum::get_string_categories();
    
    let mut res:String ="Categorias disponíveis:\n\n".to_string();

    for (index, category) in categories.iter().enumerate(){
        let msg = &format!("  {}- {}\n", index+1, category);
        res.push_str(msg);
    }
    res
}








