use diesel::prelude::*;

use crate::db::establish_connection;
use crate::models::{Categorias, Conhecimentos};
use crate::schema::{categorias, conhecimentos};

pub fn all_categories() -> Result<Vec<Categorias>, String> {
    let connection = &mut establish_connection();

    match categorias::table
        .order(categorias::id.asc())
        .load::<Categorias>(connection) {
        Ok(categorias) => { Ok(categorias) }
        Err(_err) => { Err(format!("Erro ao listar categorias: {} ", _err)) }
    }
}


pub fn all_knowledge() -> Result<Vec<Conhecimentos>, String> {
    let connection = &mut establish_connection();

    match conhecimentos::table
        .order(conhecimentos::id.asc())
        .load::<Conhecimentos>(connection) {
        Ok(conhecimentos) => { Ok(conhecimentos) }
        Err(_err) => { Err(format!("Erro ao listar conhecimentos: {} ", _err)) }
    }
}