use diesel::prelude::*;

use crate::db::establish_connection;
use crate::models::Conhecimentos;
use crate::options::read::find_by_id;
use crate::schema::conhecimentos::dsl::*;

pub fn update_knowledge(
    _id: i32,
    category_id: i32,
    description: &str,
) -> Result<Conhecimentos, String>
{
    let connection = &mut establish_connection();

    let conhecimento = Conhecimentos {
        id: _id,
        categoria_id: category_id,
        descricao: description.to_string(),
    };

    match diesel::update(conhecimentos.find(conhecimento.id))
        .set(&conhecimento)
        .execute(connection) {
        Ok(_) => {}
        Err(_err) => { return Err(format!("Erro ao atualizar item: {:?}", _err)); }
    };

    match find_by_id(conhecimento.id) {
        None => { return Err(format!("conhecimento com id {:?} não localizado", id)); }
        Some(e) => { Ok(e) }
    }
}


