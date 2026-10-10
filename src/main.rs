use routes::{health, auth, user, task_views};
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
                .service(task_views::create_shared_task_view)
                .service(task_views::delete_shared_task_view)
                .service(task_views::update_shared_task_view)
                // .service(task_views::list_shared_task_views)  // 2 Types: Shared with me & Shared by me on the given viewID (should be owned by the user)
            )
            // .service(
            //     actix_scope("/tasks")   // Should give ViewID every time
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(task_events::create_task)  // Any assignee can create a sub-task ; Any can_create can create a top-level task under that user ; Self can do anything under owned by them tasks
            //     .service(task_events::list_tasks)  // List tasks based on the view (Also Sub-Tasks)
            //     .service(task_events::get_task) // Including shared task fetch / Sub-Task fetch
            //     .service(task_events::update_task)  // Only self-owned
            //     .service(task_events::delete_task)  // Only self-owned
            // )
            // .service(
            //     actix_scope("/recurring")   // Should give ViewID every time
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(task_events::create_recurring_task)
            //     .service(task_events::list_recurring_tasks)  // List recurring tasks based on the view (There are no Sub-Tasks)
            //     .service(task_events::get_recurring_task) // Including shared task fetch (if the view permits)
            //     .service(task_events::update_recurring_task)  // Only self-owned
            //     .service(task_events::delete_recurring_task)  // Only self-owned
            // )
            // .service(
            //     actix_scope("/alerts")   // Should give AlertID every time
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     // This is only for self-owned alerts not shared at all
            //     .service(task_alerts::create_alert)
            //     .service(task_alerts::list_alerts)  // List alerts based on the view
            //     .service(task_alerts::list_all_alerts)  // List all alerts regardless of the view
            //     .service(task_alerts::get_alert) // Including shared alert fetch
            //     .service(task_alerts::update_alert)  // Only self-owned
            //     .service(task_alerts::delete_alert)  // Only self-owned
            // )
            // .service(
            //     actix_scope("/comments")   // Should give ViewID + TaskID every time (Should have show_comments = True)
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(task_comments::create_comment) // Only self-owned
            //     .service(task_comments::list_comments)  // List comments based on the view (Anyone)
            //     .service(task_comments::get_comment)    // Fetch just one comment (same as list struct use)
            //     .service(task_comments::update_comment)  // Only self-owned
            //     .service(task_comments::delete_comment)  // Only self-owned
            //     .service(task_comments::react_to_comment)  // Any user can react to any visible comment
            // )
            // .service(
            //     actix_scope("/assignees")   // Should give ViewID + TaskID every time
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(task_assignees::assign_user)  // Any-one can assign a user to a task (If they are the owner or one of the assignees)
            //     .service(task_assignees::remove_user)  // Only self-owned task can remove an assignee (Self can not exit)
            //     .service(task_assignees::list_assignees)  // List all assignees for a task
            // )
    })
    .bind(("0.0.0.0", 8686))?
    .workers(env_var("API_WORKERS_COUNT").unwrap_or("4".to_string()).parse().unwrap())
    .run().await
}
