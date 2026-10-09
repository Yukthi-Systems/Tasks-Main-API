use crate::features::task_views::{self, CreateTaskViewDto, UpdateTaskViewDto};
use actix_web::{HttpResponse, delete, get, patch, post, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;



#[post("/create")]
pub async fn create_task_view(session_user: web::ReqData<SessionUser>, data: web::Json<CreateTaskViewDto>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let new_info = data.into_inner();

    // Create a new task view using the provided information and the session user's ID
    task_views::create_task_view(&state.pg_pool, &session_user.user_id, &new_info).await?;
    
    Ok(HttpResponse::Created().body("Task view created successfully"))
}


#[get("/list")]
pub async fn list_task_views(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();

    // No Pagination implemented, since it might not be necessary for a small number of task views

    // List task views for the session user
    let task_views = task_views::list_task_views(&state.pg_pool, &session_user.user_id).await?;

    Ok(HttpResponse::Ok().json(task_views))
}


#[get("/get/{view_id}")]
pub async fn get_task_view(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let view_id = path.into_inner();

    let task_view = task_views::get_task_view_by_user(&state, &session_user.user_id, view_id).await?;

    Ok(HttpResponse::Ok().json(task_view))
}


#[patch("/update")]
pub async fn update_task_view(session_user: web::ReqData<SessionUser>, data: web::Json<UpdateTaskViewDto>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let new_info = data.into_inner();

    // Update an existing task view using the provided information and the session user's ID
    task_views::update_task_view(&state, &session_user.user_id, &new_info).await?;
    
    Ok(HttpResponse::Ok().body("Task view updated successfully"))
}


#[delete("/delete/{view_id}")]
pub async fn delete_task_view(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let view_id = path.into_inner();

    // Delete an existing task view using the provided view ID and the session user's ID
    task_views::delete_task_view(&state, &session_user.user_id, view_id).await?;
    
    Ok(HttpResponse::Ok().body("Task view deleted successfully"))
}
