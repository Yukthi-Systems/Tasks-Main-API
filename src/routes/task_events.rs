use crate::features::task_events::{self, CreateRecurringTaskDto, UpdateRecurringTaskDto, CreateTaskDto};
use actix_web::{HttpResponse, delete, get, patch, post, web};
use crate::features::task_views::TaskTag;
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
pub async fn list_recurring_tasks(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, query: web::Query<Vec<TaskTag>>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let view_id = path.into_inner();
    let task_tags = query.into_inner();

    // List recurring tasks based on the view (There are no Sub-Tasks)
    // Has the show_recurring permission enabled for this view
    let tasks = task_events::list_recurring_tasks(&state, &session_user.user_id, view_id, task_tags).await?;

    Ok(HttpResponse::Ok().json(tasks))
}


#[get("/info/{view_id}/{recurring_task_id}/{task_tag}")]
pub async fn get_recurring_task(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, i64, TaskTag)>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let (view_id, recurring_task_id, task_tag) = path.into_inner();

    // Has the show_recurring permission enabled for this view
    let task = task_events::get_one_recurring_task(&state, &session_user.user_id, view_id, recurring_task_id, task_tag).await?;

    Ok(HttpResponse::Ok().json(task))
}


#[patch("/update/{view_id}")]
pub async fn update_recurring_task(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, data: web::Json<UpdateRecurringTaskDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let task_info = data.into_inner();
    let view_id = path.into_inner();

    // Can update recurring tasks if show_recurring, can_update is true
    task_events::update_recurring_task(&state, &session_user.user_id, view_id, &task_info).await?;

    Ok(HttpResponse::Ok().body("Recurring task updated successfully"))
}


#[delete("/delete/{view_id}/{recurring_task_id}/{task_tag}")]
pub async fn delete_recurring_task(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, i64, TaskTag)>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let (view_id, recurring_task_id, task_tag) = path.into_inner();

    // Can delete recurring tasks if show_recurring, can_delete is true
    task_events::delete_recurring_task(&state, &session_user.user_id, view_id, recurring_task_id, task_tag).await?;

    Ok(HttpResponse::Ok().body("Recurring task deleted successfully"))
}


#[post("/create/{view_id}")]
pub async fn create_task(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, data: web::Json<CreateTaskDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let view_id = path.into_inner();
    let task_info = data.into_inner();

    // Any assignee can create a sub-task
    // Any can_create can create a top-level task under that user
    // Self can do anything under owned by them tasks
    task_events::create_task(&state, &session_user.user_id, view_id, &task_info).await?;

    Ok(HttpResponse::Created().body("Task created successfully"))
}
