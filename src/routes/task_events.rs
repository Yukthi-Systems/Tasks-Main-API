use crate::features::task_events::{self, CreateRecurringTaskDto};
use actix_web::{HttpResponse, delete, get, patch, post, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;



#[post("/create/{view_id}")]
pub async fn create_recurring_task(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, data: web::Json<CreateRecurringTaskDto>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let task_info = data.into_inner();
    let view_id = path.into_inner();

    // Can create recurring tasks if show_recurring, can_create is true

    // Create a new task view using the provided information and the session user's ID
    task_events::create_recurring_task(&state, &session_user.user_id, &task_info, view_id).await?;

    Ok(HttpResponse::Created().body("Recurring task created successfully"))
}


#[get("/list/{view_id}")]
pub async fn list_recurring_tasks(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let view_id = path.into_inner();

    // List recurring tasks based on the view (There are no Sub-Tasks)
    // Has the show_recurring permission enabled for this view

    let tasks = task_events::list_recurring_tasks(&state, &session_user.user_id, view_id).await?;

    Ok(HttpResponse::Ok().json(tasks))
}
