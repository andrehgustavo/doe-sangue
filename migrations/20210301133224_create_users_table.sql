-- Create users Table
CREATE TABLE users(
	   id uuid NOT NULL,
	   PRIMARY KEY (id),
	   email TEXT NOT NULL UNIQUE,
	   name TEXT NOT NULL,
	   role TEXT NOT NULL
);

INSERT INTO users
VALUES('b4fff169-b165-4ca3-bff4-1f1b437123a0', 'teste@email.com', 'Usuário Teste', 'Doador');
