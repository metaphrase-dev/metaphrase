INSERT INTO translations (key, locale, content) VALUES ("ui.add", "fr", "ajouter");
INSERT INTO translations (key, locale, content) VALUES ("ui.add", "de", "hinzufügen");
INSERT INTO translations (key, locale, content) VALUES ("ui.add", "en", "add");
INSERT INTO translations (key, locale, content) VALUES ("ui.add", "es", "añadir");
INSERT INTO translations (key, locale, content) VALUES ("ui.add", "it", "aggiungi");
INSERT INTO translations (key, locale, content) VALUES ("ui.add", "ru", "добавить");

-- password = "testpassword"
INSERT INTO users (email, hashed_password) VALUES ("raphael@lustin.fr", "$2b$08$AA4FQEHw4mmqL2i9SGfFge8PMImg.3QyPpgCmF9L4sMfYiDOuSnPO");

INSERT INTO sessions (token, user_id, expired_at) VALUES ("tokentodelete", 1, "2050-12-31 23:59:59");
INSERT INTO sessions (token, user_id, expired_at) VALUES ("goodtokenfortests", 1, "2050-12-31 23:59:59");
INSERT INTO sessions (token, user_id, expired_at) VALUES ("expiredtokenfortests", 1, "2016-12-01 00:00:00");
