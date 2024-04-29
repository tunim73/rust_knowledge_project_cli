use crate::models::{CategoriasEnum, Conhecimentos, ErrorApp};
use crate::models::TypeErro::{InvalidArguments, Others};
use crate::options::create::create_knowledge;
use crate::options::delete::delete_knowledge;
use crate::options::list::{all_categories, all_knowledge};
use crate::options::read::{find_by_category, find_by_description_and_category, find_by_id};
use crate::options::update::update_knowledge;
use crate::output::{output_categories, output_knowledge, output_lot_knowledge};


pub fn read_by_id(args: &Vec<String>) -> Result<bool, ErrorApp>
{
    let id = &args[2];
    let id = id.trim().to_lowercase();
    let id = id.as_str();

    let id_i32 = match id.parse::<i32>() {
        Ok(res) => res,
        Err(_) => {
            let _error = ErrorApp {
                type_error: InvalidArguments,
                msg: "\n
  id precisa ser um numero".to_string()
            };

            return Err(_error);
        }
    };

    let result = match find_by_id(id_i32) {
        None => {
            let _error = ErrorApp {
                type_error: Others,
                msg: format!("Conhecimento com id {} não localizado", id_i32)
            };
            return Err(_error);
        }
        Some(r) => { r }
    };

    output_knowledge(&result);

    Ok(true)
}

pub fn read_category(args: &Vec<String>) -> Result<bool, ErrorApp>
{
    let category_id_string = &args[2];
    let category_id_string = category_id_string.trim().to_lowercase();
    let category_id_string = category_id_string.as_str();


    let category_i32 = match CategoriasEnum::get_number(category_id_string) {
        None => {
            let _error = ErrorApp {
            type_error: InvalidArguments,
            msg: "categoria nao localizada".to_string()
        };
            return Err(_error);
        }
        Some(item) => { item }
    };

    let result: Vec<Conhecimentos> = match find_by_category(category_i32) {
        Ok(item) => { item }
        Err(_err) => {
            let _error = ErrorApp {
                type_error: Others,
                msg: _err
            };
            return Err(_error);
        }
    };

    if args.len() == 5 {
        let file_name = &args[4];
        if !file_name.ends_with(".txt") {
            let _error = ErrorApp {
                type_error: Others,
                msg: "arquivo de saída precisa ser um txt".to_string()
            };
            return Err(_error);
        }
        output_lot_knowledge(&result, Some(file_name));
    } else {
        output_lot_knowledge(&result, None);
    }



    Ok(true)
}

pub fn read_by_description_and_category(args: &Vec<String>) -> Result<bool, ErrorApp>
{
    let category_string: &String = &args[2];
    let category_string = category_string.trim().to_lowercase();
    let category_string = category_string.as_str();

    let category_number = match CategoriasEnum::get_number(category_string) {
        None => {
            let _error = ErrorApp {
                type_error: InvalidArguments,
                msg: "categoria nao localizada".to_string()
            };
            return Err(_error);
        }
        Some(item) => { item }
    };
    let description: &String = &args[3];
    let description = description.trim().to_lowercase();
    let description = description.as_str();

    let result = match find_by_description_and_category(category_number, description) {
    Ok(item) => { item }
    Err(_err) => {
        let _error = ErrorApp {
            type_error: Others,
            msg: _err
        };
        return Err(_error);
    }
};

    if args.len() == 6 {
        let file_name = &args[5];
        if !file_name.ends_with(".txt") {
            let _error = ErrorApp {
                type_error: Others,
                msg: "arquivo de saída precisa ser um txt".to_string()
            };
            return Err(_error);
        }
        output_lot_knowledge(&result, Some(file_name));
    } else {
        output_lot_knowledge(&result, None);
    }

    Ok(true)
}

pub fn create(args: &Vec<String>) -> Result<bool, ErrorApp>
{
    let category_string = &args[2];
    let category_string = category_string.trim().to_lowercase();
    let category_string = category_string.as_str();

    let category_i32 = match CategoriasEnum::get_number(category_string) {
        None => {
            let _error = ErrorApp {
                type_error: InvalidArguments,
                msg: "categoria nao localizada".to_string()
            };
            return Err(_error);
        }
        Some(item) => { item }
    };

    let description = &args[3];
    let description = description.trim().to_lowercase();
    let description = description.as_str();

    let result = match create_knowledge(category_i32, description) {
    Ok(item) => { item }
    Err(_err) => {
        let _error = ErrorApp {
            type_error: Others,
            msg: _err
        };
        return Err(_error);
    }
};

    println!("Conhecimento adicionado com sucesso !\n");
    output_knowledge(&result);

    Ok(true)
}

pub fn update(args: &Vec<String>) -> Result<bool, ErrorApp>
{
    let id = &args[2];
    let id = id.trim().to_lowercase();
    let id = id.as_str();

    let id_i32 = match id.parse::<i32>() {
        Ok(res) => res,
        Err(_) => {
            let _error = ErrorApp {
                type_error: InvalidArguments,
                msg: "id precisa ser um numero".to_string()
            };
            return Err(_error);
        }
    };

    let category_string = &args[3];
    let category_string = category_string.trim().to_lowercase();
    let category_string = category_string.as_str();

    let category_i32 = match CategoriasEnum::get_number(category_string) {
        None => {
            let _error = ErrorApp {
                type_error: InvalidArguments,
                msg: "categoria nao localizada".to_string()
            };
            return Err(_error);
        }
        Some(item) => { item }
    };

    let description = &args[4];
    let description = description.trim().to_lowercase();
    let description = description.as_str();
    let result = match  update_knowledge(id_i32, category_i32, description) {
    Ok(item) => { item }
    Err(_err) => {
        let _error = ErrorApp {
            type_error: Others,
            msg: _err
        };
        return Err(_error);
    }
};
    output_knowledge(&result);

    Ok(true)
}

pub fn delete(args: &Vec<String>) -> Result<bool, ErrorApp>
{
    let id = &args[2];
    let id = id.trim().to_lowercase();
    let id = id.as_str();

    let id_i32 = match id.parse::<i32>() {
        Ok(res) => res,
        Err(_) => {
            let _error = ErrorApp {
                type_error: InvalidArguments,
                msg: "id precisa ser um numero".to_string()
            };
            return Err(_error);
        }
    };

    match delete_knowledge(id_i32) {
        Ok(_) => { Ok(true) }
        Err(_err) => {
            let _error = ErrorApp {
                type_error: Others,
                msg: _err
            };
            return Err(_error);
        }
    }

}

pub fn list_knowledge(args: &Vec<String>) -> Result<bool, ErrorApp>
{
    let result = match all_knowledge() {
    Ok(item) => { item }
    Err(_err) => {
        let _error = ErrorApp {
            type_error: Others,
            msg: _err
        };
        return Err(_error);
    }
};

    if args.len() == 5 {
        let file_name = &args[4];
        if !file_name.ends_with(".txt") {
            let _error = ErrorApp {
                type_error: Others,
                msg: "arquivo de saída precisa ser um txt".to_string()
            };
            return Err(_error);
        }
        output_lot_knowledge(&result, Some(file_name));
    } else {
        output_lot_knowledge(&result, None);
    }
    Ok(true)
}

pub fn list_categories(args: &Vec<String>) -> Result<bool, ErrorApp>
{
    let result = match all_categories() {
    Ok(item) => { item }
    Err(_err) => {
        let _error = ErrorApp {
            type_error: Others,
            msg: _err
        };
        return Err(_error);
    }
};

    if args.len() == 5 {
        let file_name = &args[4];
        if !file_name.ends_with(".txt") {
            let _error = ErrorApp {
                type_error: Others,
                msg: "arquivo de saída precisa ser um txt".to_string()
            };
            return Err(_error);
        }
        output_categories(&result, Some(file_name));
    } else {
        output_categories(&result, None);
    }

    Ok(true)
}






