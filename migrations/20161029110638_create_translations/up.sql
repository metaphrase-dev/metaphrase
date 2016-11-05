CREATE TABLE translations (
  id INTEGER PRIMARY KEY NOT NULL,
  key TEXT NOT NULL,
  locale TEXT NOT NULL,
  content TEXT,
  timestamp TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO translations (key, locale, content) VALUES ("ui.add", "fr", "ajouter");
INSERT INTO translations (key, locale, content) VALUES ("ui.add", "de", "hinzufügen");
INSERT INTO translations (key, locale, content) VALUES ("ui.add", "en", "add");
INSERT INTO translations (key, locale, content) VALUES ("ui.add", "es", "añadir");
INSERT INTO translations (key, locale, content) VALUES ("ui.add", "it", "aggiungi");
INSERT INTO translations (key, locale, content) VALUES ("ui.add", "ru", "добавить");
