use crate::features::task_alerts::{self, CreateTaskAlertDto, EditTaskAlertDto, DeleteTaskAlertDto};
use actix_web::{HttpResponse, delete, get, patch, post, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;


// All alerts are associated only with self (the session user)
// So no need to do any validation for ownership or assignee status


#[post("/create/{task_id}")]
pub async fn create_alert(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, body: web::Json<CreateTaskAlertDto>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let task_id = path.into_inner();
    let create_task_alert_dto = body.into_inner();

    // Validate the task alert DTO
    if !create_task_alert_dto.validate() {
        return Ok(HttpResponse::BadRequest().body("Invalid task alert data, maybe check the alert time"));
    }

    // Create a new task alert for the session user
    task_alerts::create_alert(
        &state,
        &session_user.user_id,
        task_id,
        create_task_alert_dto
    ).await?;

    Ok(HttpResponse::Created().body("Task alert created successfully"))
}


#[delete("/delete/{task_id}")]
pub async fn delete_alert(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, body: web::Json<DeleteTaskAlertDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let task_id = path.into_inner();
    let delete_task_alert_dto = body.into_inner();

    task_alerts::delete_alert(&state, &session_user.user_id, task_id, delete_task_alert_dto).await?;

    Ok(HttpResponse::Ok().body("Task alert deleted successfully"))
}


#[patch("/edit/{task_id}")]
pub async fn update_alert(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, body: web::Json<EditTaskAlertDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let task_id = path.into_inner();
    let edit_task_alert_dto = body.into_inner();

    // Validate the edit task alert DTO
    if !edit_task_alert_dto.validate() {
        return Ok(HttpResponse::BadRequest().body("Invalid task alert data, maybe check the alert time"));
    }

    task_alerts::edit_alert(&state, &session_user.user_id, task_id, edit_task_alert_dto).await?;

    Ok(HttpResponse::Ok().body("Task alert updated successfully"))
}
