/*
Não consigui rodar de forma separada ao projeto, então dá muitos erros.

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rust_knowledge_project::db::establish_connection;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

fn main() {
    let connection = &mut establish_connection();
    run_migrations(connection);
    println!("Migrações aplicadas com sucesso!");
}*/