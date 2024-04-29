CREATE TABLE conhecimentos (
                           id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                           categoria_id INTEGER NOT NULL,
                           descricao TEXT NOT NULL,
                           FOREIGN KEY (categoria_id) REFERENCES categorias(id)
);
