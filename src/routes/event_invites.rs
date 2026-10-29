use actix_web::{HttpResponse, delete, get, patch, post, web};
use crate::features::task_comments::{self, ListCommentsDto};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;



#[post("/create/{view_id}/{task_id}")]
pub async fn create_invite(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, i64)>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let (view_id, task_id) = path.into_inner();
    let comment_content = body.into_inner();

    // Any user associated with the task can add a comment
    task_comments::create_comment(
        &state,
        &session_user.organization_id,
        &session_user.user_id,
        view_id,
        task_id,
        &comment_content,
    ).await?;

    Ok(HttpResponse::Created().body("Comment created successfully"))
}
