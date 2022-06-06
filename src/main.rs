extern crate log;
extern crate pretty_env_logger;



use actix_web::{App, HttpServer, middleware::Logger, web};

use crate::api::authentication;
use crate::authentication::{change_password_endpoint, dashboard_url_endpoint, is_etebase_endpoint, login_challenge_endpoint, login_endpoint, logout_endpoint, signup_endpoint};

mod api;
mod model;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let app = move || {
        let logger =  Logger::default();
        return App::new()
            .wrap(logger)
            .service(
            web::scope("/api/v1")
                .service(
                    web::scope("/authentication")
                        .service(is_etebase_endpoint)
                        .service(login_challenge_endpoint)
                        .service(login_endpoint)
                        .service(logout_endpoint)
                        .service(signup_endpoint)
                        .service(change_password_endpoint)
                        .service(dashboard_url_endpoint)

                )
                .service(
                web::scope("/collection"))
                .service(
                web::scope("/invitation")
                    .service(
                        web::scope("/incoming"))
                    .service(
                        web::scope("/outgoing")
                    )
                )
            );
    };

    let http_server =  HttpServer::new( app).bind("0.0.0.0:3735");
    return http_server?
        .run()
        .await;
}