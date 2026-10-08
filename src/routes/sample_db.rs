use crate::database::notes::{add_new_notes, fetch_all_notes};
use actix_web::{get, post, web, HttpResponse};
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::{
    users::SessionUser,
    notes::Notes,
};


#[post("/create-note")]
pub async fn create_note_handler(session_user: web::ReqData<SessionUser>, body: web::Json<Notes>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let notes_body = body.into_inner(); 

    tracing::debug!("{} is creating a new note.", session_user);

    add_new_notes(&state.pg_pool, vec![notes_body]).await?;

    Ok(HttpResponse::Ok().json("Note created successfully!"))
}


#[get("/notes")]
pub async fn list_notes_handler(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();

    tracing::debug!("User '{}' is listing notes.", session_user.user_name);

    let notes = fetch_all_notes(&state.pg_pool).await?;

    Ok(HttpResponse::Ok().json(notes))
}
