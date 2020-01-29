use actix_web::web;

mod handler;
pub mod resolvers;

pub(super) fn route(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/graphql")
      .route(web::post().to(handler::graphql))
      .route(web::get().to(handler::playground)),
  );
}
