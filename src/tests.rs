#[cfg(test)]
mod tests {
    use crate::options::create::create_knowledge;
    use crate::options::delete::delete_knowledge;
    use crate::options::read::{find_by_category, find_by_description_and_category, find_by_id};
    use crate::options::update::update_knowledge;

    const DESCRIPTION: &str = "abcdefghi1";
    const CATEGORY_ID: i32 = 1;


    #[test]
    fn test_crud(){

        let category_id = CATEGORY_ID;
        let mut description = DESCRIPTION.to_string();

        //create and read by id
        let _conhecimento = create_knowledge(category_id, &description).unwrap();
        let result_read_by_id = find_by_id(_conhecimento.id).unwrap();
        assert_eq!(_conhecimento.id, result_read_by_id.id);
        assert_eq!(_conhecimento.categoria_id, result_read_by_id.categoria_id);
        assert_eq!(_conhecimento.descricao, result_read_by_id.descricao);

        //updated
        description = "updated".to_string();
        let result_updated = update_knowledge(_conhecimento.id, _conhecimento.categoria_id, &description)
            .unwrap();
        assert_eq!(result_updated.descricao, description);

        //delete
        let result_delete = delete_knowledge(_conhecimento.id).unwrap();
        assert!(result_delete, "Erro ao deletar")

    }

    #[test]
    fn test_find_by_description_and_category()
    {
        create_knowledge(CATEGORY_ID, &DESCRIPTION).unwrap();
        let result = find_by_description_and_category(CATEGORY_ID, DESCRIPTION).unwrap();
        assert_eq!(result[0].descricao, DESCRIPTION);
    }

    #[test]
    fn test_find_by_category()
    {
        create_knowledge(CATEGORY_ID, &DESCRIPTION).unwrap();
        let result = find_by_category(CATEGORY_ID).unwrap();
        assert_eq!(result[0].categoria_id, CATEGORY_ID);
    }

    
}





