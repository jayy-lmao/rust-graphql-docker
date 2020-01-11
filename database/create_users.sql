CREATE TABLE persons
(
	person_id serial NOT NULL,
	person_name VARCHAR(100)
);
 
-- ALTER TABLE PERSONS owner TO jayylmao;

INSERT INTO persons (person_name) 
        VALUES ('Tim');
INSERT INTO persons (person_name) 
        VALUES ('Bill');
INSERT INTO persons (person_name) 
        VALUES ('Bob');
INSERT INTO persons (person_name) 
        VALUES ('Hamo');
INSERT INTO persons (person_name) 
        VALUES ('Jerry');
