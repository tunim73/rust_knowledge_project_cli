// @generated automatically by Diesel CLI.

diesel::table! {
    categorias (id) {
        id -> Integer,
        categoria -> Text,
    }
}

diesel::table! {
    conhecimentos (id) {
        id -> Integer,
        categoria_id -> Integer,
        descricao -> Text,
    }
}

diesel::joinable!(conhecimentos -> categorias (categoria_id));

diesel::allow_tables_to_appear_in_same_query!(
    categorias,
    conhecimentos,
);
