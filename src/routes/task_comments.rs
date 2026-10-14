use actix_web::{HttpResponse, delete, get, patch, post, web};
use crate::features::users::SessionUser;
use crate::features::task_comments;
use crate::errors::ApiResponse;
use crate::state::AppState;



#[post("/create/{view_id}/{task_id}")]
pub async fn create_comment(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, i64)>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
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


#[patch("/react/{view_id}/{task_id}/{comment_id}")]
pub async fn react_to_comment(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, i64, i64)>, body: web::Json<serde_json::Value>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let (view_id, task_id, comment_id) = path.into_inner();
    let reaction_data = body.into_inner();

    // Any user associated with the task can add a reaction to a comment
    task_comments::add_reaction(
        &state,
        &session_user.user_id,
        view_id,
        task_id,
        comment_id,
        &reaction_data,
    ).await?;

    Ok(HttpResponse::Ok().body("Reaction added successfully"))
}


#[delete("/delete/{view_id}/{task_id}/{comment_id}")]
pub async fn delete_comment(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, i64, i64)>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let (_view_id, task_id, comment_id) = path.into_inner();

    // Only possible if its a comment Author, since he is the one who created it
    // No checks are performed to verify this, only simple author_id match to the session_user.user_id is used

    // Any user associated with the task can delete a comment
    let result = task_comments::delete_comment(
        &state.pg_pool,
        &session_user.user_id,
        &task_id,
        &comment_id,
    ).await?;
    if result == 0 {
        return Ok(HttpResponse::NotFound().body("Comment is not found or it does not belong to the current user"));
    }

    Ok(HttpResponse::Ok().body("Comment deleted successfully"))
}
