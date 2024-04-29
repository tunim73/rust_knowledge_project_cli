use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use crate::models::{Categorias, CategoriasEnum, Conhecimentos};

const PATH_DIRECTORY: &str = "output";

pub fn output_lot_knowledge(conhecimentos: &Vec<Conhecimentos>, file_name: Option<&str>) {
    if conhecimentos.len() == 0 {
        println!("\nLista vazia !")
    }

    match file_name {
        None => {
            for conhecimento in conhecimentos.iter() {
                output_knowledge(conhecimento);
            }
        }
        Some(name) => {
            let mut file = create_file_in_directory(name).expect("Erro ao criar arquivo");
            for conhecimento in conhecimentos.iter() {
                writeln!(file, "{}", string_knowledge(conhecimento)).expect("Erro ao escrever no arquivo");
            }
        }
    };
}

pub fn output_knowledge(conhecimento: &Conhecimentos) {
    println!("{}", string_knowledge(&conhecimento));
}

pub fn output_categories(categories: &Vec<Categorias>, file_name: Option<&str>) {

    match file_name {
        None => {
            for category in categories.iter() {
                println!("Categoria ID: {}, nome: {}", category.id, category.categoria);
            }
        }
        Some(name) => {
            let mut file = create_file_in_directory(name).expect("Erro ao criar arquivo");
            for category in categories.iter() {
                writeln!(file, "Categoria ID: {}, nome: {}", category.id, category.categoria).expect("Erro ao escrever no arquivo");
            }
        }
    };


}

fn create_file_in_directory(file_name: &str) -> std::io::Result<File> {
    let base_path = Path::new(PATH_DIRECTORY);

    if !base_path.exists() {
        create_dir_all(base_path)?;
    }

    let file_path = base_path.join(file_name);
    let file = File::create(&file_path)?;

    Ok(file)
}

fn string_knowledge(conhecimento: &Conhecimentos) -> String
{
    let _category = CategoriasEnum::get_string_category(conhecimento.categoria_id);
    format!("ID: {}, Categoria: {}, Descrição: {:?}", conhecimento.id, _category, conhecimento.descricao)
}
