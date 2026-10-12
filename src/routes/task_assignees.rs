use crate::features::task_assignees::{self, AddAssigneeDto};
use actix_web::{HttpResponse, delete, get, patch, post, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;



#[post("/create/{view_id}")]
pub async fn assign_user(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, body: web::Json<AddAssigneeDto>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let add_assignee_dto = body.into_inner();
    let view_id = path.into_inner();

    // Only the owner or one of the assignees can assign a user to a task and can not assign themselves
    if !add_assignee_dto.validate(&session_user.user_id) {
        return Ok(HttpResponse::Forbidden().body("You cannot assign yourself to this task"));
    }

    // Create a new task view using the provided information and the session user's ID
    task_assignees::assign_user(
        &state,
        &session_user.organization_id,
        &session_user.user_id,
        &add_assignee_dto,
        view_id
    ).await?;

    Ok(HttpResponse::Created().body("User assigned to task successfully"))
}
