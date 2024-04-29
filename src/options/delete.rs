use diesel::prelude::*;
use crate::db::establish_connection;

use crate::schema::conhecimentos;

pub fn delete_knowledge(
    _id: i32,
) -> Result<bool, String> {
    let connection = &mut establish_connection();

    match diesel::delete(conhecimentos::table.find(_id))
        .execute(connection) {
        Ok(_) => {Ok(true)}
        Err(_err) => {  Err(format!("Erro ao deletar: {} ", _err)) }
    }
}
