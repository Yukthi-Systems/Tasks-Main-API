use crate::features::task_alerts::{self, TaskAlertDto};
use actix_web::{HttpResponse, delete, get, post, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;


// All alerts are associated only with self (the session user)
// So no need to do any validation for ownership or assignee status


#[post("/create/{task_id}")]
pub async fn create_alert(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, body: web::Json<TaskAlertDto>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let task_id = path.into_inner();
    let task_alert_dto = body.into_inner();

    // Validate the task alert DTO
    if !task_alert_dto.validate() {
        return Ok(HttpResponse::BadRequest().body("Invalid task alert data, maybe check the alert time"));
    }

    // Create a new task alert for the session user
    task_alerts::create_alert(
        &state,
        &session_user.user_id,
        task_id,
        task_alert_dto
    ).await?;

    Ok(HttpResponse::Created().body("Task alert created successfully"))
}
