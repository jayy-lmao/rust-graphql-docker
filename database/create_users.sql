CREATE TABLE persons
(
person_id SERIAL NOT NULL,
person_name VARCHAR(100),
cult INTEGER
);
 
-- ALTER TABLE PERSONS owner TO jayylmao;

INSERT INTO persons (person_name, cult) 
        VALUES ('Tim', 1);
INSERT INTO persons (person_name, cult) 
        VALUES ('Bill', 1);
INSERT INTO persons (person_name, cult) 
        VALUES ('Bob', 2);
INSERT INTO persons (person_name, cult) 
        VALUES ('Hamo', 2);
INSERT INTO persons (person_name) 
        VALUES ('Jerry');
