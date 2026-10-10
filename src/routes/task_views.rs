use crate::features::task_views::{self, CreateSharedTaskViewDto, CreateTaskViewDto, UpdateTaskViewDto};
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


#[get("/list/{is_shared}")]
pub async fn list_task_views(session_user: web::ReqData<SessionUser>, path: web::Path<bool>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let is_shared = path.into_inner();

    // No Pagination implemented, since it might not be necessary for a small number of task views

    // List task views for the session user
    let task_views = if is_shared {
        task_views::list_my_shared_views(&state.pg_pool, &session_user.user_id).await?
    } else {
        task_views::list_task_views(&state.pg_pool, &session_user.user_id).await?
    };

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


#[post("/create")]
pub async fn create_shared_task_view(session_user: web::ReqData<SessionUser>, data: web::Json<CreateSharedTaskViewDto>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let new_info = data.into_inner();

    // Create a new shared task view using the provided information and the session user's ID
    let result = task_views::create_shared_task_view(&state.pg_pool, &session_user.organization_id, &session_user.user_id, &new_info).await?;
    if result == 0 {
        return Ok(HttpResponse::BadRequest().body("Failed to create shared task view"));
    }

    Ok(HttpResponse::Created().body("Shared task view created successfully"))
}


#[delete("/delete/{shared_view_id}/{user_id}")]
pub async fn delete_shared_task_view(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, uuid::Uuid)>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let (shared_view_id, user_id) = path.into_inner();

    // Delete an existing shared task view using the provided shared view ID and the session user's ID
    task_views::remove_user_from_shared_task_view(&state, &session_user.user_id, shared_view_id, &user_id).await?;

    Ok(HttpResponse::Ok().body("Shared task view deleted successfully"))
}


#[patch("/update")]
pub async fn update_shared_task_view(session_user: web::ReqData<SessionUser>, data: web::Json<CreateSharedTaskViewDto>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let new_info = data.into_inner();

    // Update an existing shared task view using the provided information and the session user's ID
    let result = task_views::update_shared_task_view(&state, &session_user.user_id, &new_info).await?;
    if result == 0 {
        return Ok(HttpResponse::BadRequest().body("Failed to update shared task view"));
    }

    Ok(HttpResponse::Ok().body("Shared task view updated successfully"))
}


#[get("/list/{view_id}")]
pub async fn list_all_shared_users(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let view_id = path.into_inner();

    // Only possible if its the owner of the shared task views, else no
    // List all users who have access to the specified shared task view
    let result = task_views::list_all_shared_users(&state, &session_user.user_id, view_id).await?;

    Ok(HttpResponse::Ok().json(result))
}


#[get("/info/{view_id}/{user_id}")]
pub async fn get_shared_task_view(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, uuid::Uuid)>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let (view_id, user_id) = path.into_inner();

    // Get the shared task view for the specified view ID and user ID
    let shared_task_view = task_views::get_shared_task_view_user_info(&state, &session_user.user_id, view_id, &user_id).await?;

    Ok(HttpResponse::Ok().json(shared_task_view))
}
