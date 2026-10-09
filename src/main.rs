use routes::{health, sample_db, auth, user};
use actix_web::web::scope as actix_scope;
use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer};
use std::env::var as env_var;
use actix_cors::Cors;


mod middleware;
mod messaging;
mod database;
mod features;
mod security;
mod errors;
mod routes;
mod tasks;
mod cache;
mod state;
mod utils;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _log_guard = utils::logging::init();

    let app_state = state::initialize().await;

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Cors::default()
                .allowed_origin_fn(state::cors_allowed_origin_fn)
                .allowed_methods(["GET", "POST", "PATCH", "DELETE"])
                .supports_credentials()
                .allow_any_header()
                .max_age(420)
            )
            .service(
                actix_scope("/health")
                .service(health::api_health_check)
            )
            .service(
                actix_scope("/internal")
                .wrap(from_fn(middleware::key_based::auth_check))
                .service(health::api_health_check)
            )
            .service(
                actix_scope("/auth")
                .service(auth::user_login)
                .service(auth::refresh_session)
                .service(
                    actix_scope("")
                    .wrap(from_fn(middleware::user_session::auth_check))
                    .service(user::update_fcm_token)
                    .service(auth::user_logout)
                    .service(auth::get_session)
                )
            )
            .service(
                actix_scope("/user")
                .wrap(from_fn(middleware::user_session::auth_check))
                // .service(user::email_dropdown_search)
                // .service(user::update_user_info)
                .service(user::get_user_info)
            )
            .service(
                actix_scope("/sample_db")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(sample_db::create_note_handler)
                .service(sample_db::list_notes_handler)
            )
    })
    .bind(("0.0.0.0", 8686))?
    .workers(env_var("API_WORKERS_COUNT").unwrap_or("4".to_string()).parse().unwrap())
    .run().await
}
