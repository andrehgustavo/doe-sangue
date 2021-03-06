-- Create users Table
CREATE TABLE users(
	   id uuid NOT NULL,
	   PRIMARY KEY (id),
	   email TEXT NOT NULL UNIQUE,
	   name TEXT NOT NULL,
	   role TEXT NOT NULL
);

