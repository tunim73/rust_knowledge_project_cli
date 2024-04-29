use diesel::prelude::*;

use crate::db::establish_connection;
use crate::models::{Conhecimentos, NewConhecimento};
use crate::schema::conhecimentos;

pub fn create_knowledge(categories_id: i32, description: &str) -> Result<Conhecimentos, String> {
    let connection = &mut establish_connection();

    let new_conhecimento = NewConhecimento {
        categoria_id: &categories_id,
        descricao: description,
    };

    match diesel::insert_into(conhecimentos::table)
        .values(&new_conhecimento)
        .execute(connection) {
        Ok(_) => {}
        Err(_err) => { return Err(format!("Erro ao adicionar conhecimento: {} ", _err)); }
    };

    match conhecimentos::table
        .order(conhecimentos::id.desc())
        .first(connection) {
        Ok(element) => { Ok(element) }
        Err(_err) => { return Err(format!("Erro apresentar conhecimento adicionado: {} ", _err)); }
    }
}
