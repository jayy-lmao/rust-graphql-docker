CREATE TABLE persons
(
	person_id serial NOT NULL,
	person_name VARCHAR(100)
);
 
-- ALTER TABLE PERSONS owner TO jayylmao;

INSERT INTO persons (person_name) 
        VALUES ('Tim');
