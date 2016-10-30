CREATE TABLE translations (
  id INTEGER PRIMARY KEY NOT NULL,
  key TEXT NOT NULL,
  locale TEXT NOT NULL,
  content TEXT NOT NULL
);

INSERT INTO translations VALUES (1, "ui.add", "fr", "ajouter");
INSERT INTO translations VALUES (2, "ui.add", "de", "hinzufügen");
INSERT INTO translations VALUES (3, "ui.add", "en", "add");
INSERT INTO translations VALUES (4, "ui.add", "es", "añadir");
INSERT INTO translations VALUES (5, "ui.add", "it", "aggiungi");
INSERT INTO translations VALUES (6, "ui.add", "ru", "добавить");
