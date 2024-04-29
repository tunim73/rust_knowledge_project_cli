use std::process;

use diesel::RunQueryDsl;

use rust_knowledge_project::db::establish_connection;
use rust_knowledge_project::models::{NewCategoria, NewConhecimento};
use rust_knowledge_project::schema::categorias;
use rust_knowledge_project::schema::conhecimentos;


fn main() {
    println!("Gerando seed de categorias !");
    seed().unwrap_or_else(|e| {
        eprintln!("ERRO: {} \n\n Contate o desenvolvedor.", e);
        process::exit(1);
    });

    println!("Seed gerado com sucesso !");
}

fn seed() -> Result<bool, String> {
    let connection = &mut establish_connection();

    let php = NewCategoria {
        categoria: "php",
    };
    let node = NewCategoria {
        categoria: "node",
    };
    let git = NewCategoria {
        categoria: "git",
    };
    let docker = NewCategoria {
        categoria: "docker",
    };

    let categorias = vec![php, node, git, docker];

    match diesel::insert_into(categorias::table)
        .values(&categorias)
        .execute(connection) {
        Ok(_) => { }
        Err(err) => { return Err(format!("Erro ao adicionar categorias: {:?} ", err)); }
    }

    let test1 = NewConhecimento {
        categoria_id: &1,
        descricao: "test1",
    };
    let test2 = NewConhecimento {
        categoria_id: &2,
        descricao: "test2",
    };
    let test3 = NewConhecimento {
        categoria_id: &3,
        descricao: "test3",
    };

    let new_conhecimentos = vec![test1, test2, test3];

    match diesel::insert_into(conhecimentos::table)
        .values(&new_conhecimentos)
        .execute(connection) {
        Ok(_) => { }
        Err(err) => { return Err(format!("Erro ao adicionar conhecimentos: {:?} ", err)); }
    }


    Ok(true)
}
