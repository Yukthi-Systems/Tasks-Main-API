use routes::{health, auth, user, task_views, task_events, task_assignees, task_comments, task_alerts, event_invitees};
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
                actix_scope("/public")
                .service(event_invitees::get_event_info)
                // .service(event_invitees::update_response)
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
                .service(user::email_dropdown_search)
                .service(user::update_user_info)
                .service(user::get_user_info)
            )
            .service(
                actix_scope("/views")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(task_views::create_task_view)
                .service(task_views::update_task_view)
                .service(task_views::delete_task_view)
                .service(task_views::list_task_views)
                .service(task_views::get_task_view)
            )
            .service(
                actix_scope("/shared")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(task_views::update_shared_task_view_info)
                .service(task_views::create_shared_task_view)
                .service(task_views::delete_shared_task_view)
                .service(task_views::update_shared_task_view)
                .service(task_views::list_all_shared_users)
                .service(task_views::get_shared_task_view)
            )
            .service(
                actix_scope("/tasks")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(task_events::create_task)
                .service(task_events::update_task)
                .service(task_events::delete_task)
                .service(task_events::list_tasks)
                .service(task_events::get_task)
            )
            .service(
                actix_scope("/recurring")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(task_events::create_recurring_task)
                .service(task_events::update_recurring_task)
                .service(task_events::delete_recurring_task)
                .service(task_events::list_recurring_tasks)
                .service(task_events::get_recurring_task)
            )
            .service(
                actix_scope("/alerts")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(task_alerts::list_all_alerts)
                .service(task_alerts::create_alert)
                .service(task_alerts::delete_alert)
                .service(task_alerts::update_alert)
                .service(task_alerts::list_alerts)
            )
            .service(
                actix_scope("/invites")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(event_invitees::create_invitee)
                .service(event_invitees::resend_invite)
                .service(event_invitees::delete_invite)
                .service(event_invitees::list_invites)
            )
            .service(
                actix_scope("/comments")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(task_comments::react_to_comment)
                .service(task_comments::create_comment)
                .service(task_comments::update_comment)
                .service(task_comments::delete_comment)
                .service(task_comments::list_comments)
                .service(task_comments::get_comment)
            )
            .service(
                actix_scope("/assignees")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(task_assignees::assign_user)
                .service(task_assignees::remove_user)
            )
    })
    .bind(("0.0.0.0", 8686))?
    .workers(env_var("API_WORKERS_COUNT").unwrap_or("4".to_string()).parse().unwrap())
    .run().await
}
