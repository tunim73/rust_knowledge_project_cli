use diesel::prelude::*;

use crate::db::establish_connection;
use crate::models::Conhecimentos;
use crate::schema::conhecimentos;

pub fn find_by_id(id: i32) -> Option<Conhecimentos>
{
    let connection = &mut establish_connection();
    match conhecimentos::table
        .find(id)
        .first::<Conhecimentos>(connection) {
        Ok(conhecimento) => { Some(conhecimento) }
        Err(_) => { None }
    }
}


pub fn find_by_description_and_category(
    category_id: i32,
    text: &str,
) -> Result<Vec<Conhecimentos>, String>
{
    let connection = &mut establish_connection();
    let like_pattern = format!("%{}%", text);

    match conhecimentos::table
        .filter(conhecimentos::categoria_id.eq(category_id))
        .filter(conhecimentos::descricao.like(like_pattern))
        .load::<Conhecimentos>(connection) {
        Ok(conhecimentos) => { Ok(conhecimentos) }
        Err(_err) => { Err(format!("Erro ao localizar conhecimentos pela descricao: {} ", _err)) }
    }
}


pub fn find_by_category(category_id: i32) -> Result<Vec<Conhecimentos>, String>
{
    let connection = &mut establish_connection();

    match conhecimentos::table
        .filter(conhecimentos::categoria_id.eq(category_id))
        .load::<Conhecimentos>(connection) {
        Ok(conhecimentos) => { Ok(conhecimentos) }
        Err(_err) => { Err(format!("Erro listar conhecimentos pela categoria: {} ", _err)) }
    }
}
