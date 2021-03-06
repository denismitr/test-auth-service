mod email_service;
mod vars;
mod model;

#[macro_use]
extern crate diesel;
extern crate serde_json;
extern crate lettre;
extern crate native_tls;

#[astix_rt::main]
async fn main() ->  std::io::Result<()> {
    use astix_cors::Cors;
    use astix_files::Files;
    use astix_sessions::CookieSession;
    use astix_web::{middleware, web, App, HttpServer, http::header};
    use diesel::{
        prelude::*,
        r2d2::{self, ConnectionManager}
    };

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger();

    // create a database connection pool
    let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
    let pool: model::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a database connection pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // Enable sessions
            .wrap(
                CookieSession::signed(&[0;32])
                    .domain(vars::domain_url().as_str())
                    .name("auth")
                    .secure(false))
            .wrap(
                Cors::new()
                    .allowed_origin("*")
                    .allowed_methods(vec!["POST","GET","DELETE"])
                    .max_age(3600)
                    .finish())
            .service(Files::new("/assets", "./templates/assets"))
    })
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}