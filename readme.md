Original Rest API & DB connection is inspired by:
https://turreta.com/2019/09/21/rest-api-with-rust-actix-web-and-postgresql-part-3/

Graphql setup is inspired by:
https://www.freecodecamp.org/news/building-powerful-graphql-servers-with-rust/

The dockerfile is all me baby :)

## Running locally

simple as `docker-compose up`, crazy!

## TODO

- [x] DB connection
- [x] Expose graphql
- [ ] Mutations!
  - [x] Create
  - [ ] Update
  - [ ] Delete
- [ ] Context to later use in Dataloaders and Auth
- [ ] Dataloaders
- [x] Auth?
- [ ] DB pool
