use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use super::service::ServiceError;
use serde::Serialize;


pub type ApiResponse = Result<HttpResponse, ServiceError>;


#[derive(Debug, Serialize)]
struct ErrorResp {
    error: String,
    code: u16
}


// ------- Implementations ------- //


impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::Redis(_) => StatusCode::BAD_GATEWAY,
            ServiceError::SerDe(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ServiceError::Reqwest(_) => StatusCode::BAD_GATEWAY,
            ServiceError::Pgsql(_) => StatusCode::EXPECTATION_FAILED,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ServiceError::Unprocessable(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ServiceError::PreconditionFailed(_) => StatusCode::PRECONDITION_FAILED,
            ServiceError::Gone(_) => StatusCode::GONE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        // Full internal error goes to logs.
        tracing::error!(
            error = ?self,
            status = %status,
            "Application error"
        );

        // Only safe information goes to the client.
        let message = match self {
            ServiceError::BadRequest(message)
            | ServiceError::NotFound(message)
            | ServiceError::Unauthorized(message)
            | ServiceError::Unprocessable(message)
            | ServiceError::PreconditionFailed(message)
            | ServiceError::Gone(message) => message.as_str(),

            // Hidden internal errors
            ServiceError::SerDe(_) => "Error parsing data",
            ServiceError::Reqwest(_) => "Upstream API service error",
            ServiceError::Redis(_) => "Cache Database error",
            ServiceError::Pgsql(_) => "SQL Database error",
        };

        HttpResponse::build(status).json(ErrorResp {
            error: message.to_owned(),
            code: status.as_u16(),
        })
    }
}
