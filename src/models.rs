use std::str::FromStr;
use crate::schema::categorias;
use crate::schema::conhecimentos;
use diesel::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = categorias)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Categorias {
    pub id: i32,
    pub categoria: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = categorias)]
pub struct NewCategoria<'a> {
    pub categoria: &'a str,
}


#[derive(Queryable, Selectable, AsChangeset, Debug)]
#[diesel(table_name = conhecimentos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Conhecimentos {
    pub id: i32,
    pub categoria_id: i32,
    pub descricao: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = conhecimentos)]
pub struct NewConhecimento<'a> {
    pub categoria_id: &'a i32,
    pub descricao: &'a str,
}

#[derive(Debug, PartialEq, EnumString, EnumIter, Display)]
pub enum CategoriasEnum {
    #[strum(serialize = "php")]
    Php = 1,
    #[strum(serialize = "node")]
    Node = 2,
    #[strum(serialize = "git")]
    Git = 3,
    #[strum(serialize = "docker")]
    Docker = 4,
}

impl CategoriasEnum {

    pub fn get_number(input: &str) -> Option<i32> {
        match CategoriasEnum::from_str(input) {
            Ok(_categoria) => Some(_categoria.to_number()),
            Err(_) => None,
        }
    }

    pub fn get_string_categories() -> Vec<String> {
        return CategoriasEnum::iter()
            .map(|categoria| categoria.to_string())
            .collect()
    }
    pub fn get_string_category(category_id: i32) -> String {
        let _x: Vec<CategoriasEnum> = CategoriasEnum::iter()
            .filter(|category| category.to_number() == category_id)
            .collect();
        return _x[0].to_string();
    }


    fn to_number(&self) -> i32 {
        match self {
            CategoriasEnum::Php => 1,
            CategoriasEnum::Node => 2,
            CategoriasEnum::Git => 3,
            CategoriasEnum::Docker => 4,
        }
    }
}


pub enum TypeErro {
    InvalidCommand,
    InvalidArguments,
    UninitializedSeed,
    Others
}

pub struct ErrorApp {
    pub type_error: TypeErro,
    pub msg: String
}




