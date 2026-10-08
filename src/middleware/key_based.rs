use actix_web::{
    dev::{
        ServiceRequest,
        ServiceResponse
    },
    body::MessageBody,
    middleware::Next,
    HttpResponse,
    Error,
};
use crate::state::API_SETTINGS;



/// Authentication middleware
/// Checks for valid API key if provided
/// Short-circuits with 401 Unauthorized if checks fail
/// Otherwise calls the next service in the chain
pub async fn auth_check<B>(req: ServiceRequest, next: Next<B>) -> Result<ServiceResponse, Error>
    where B: MessageBody + 'static
{
    // Fetch the API Key from the Header
    let api_key: Option<String> = req
        .headers()
        .get("x-api-key")
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.to_string());

    // See if API key is valid
    if api_key.is_none() || api_key.unwrap() != *API_SETTINGS.self_api_key {
        tracing::warn!("API key check failed for request {} {}", req.method(), req.path());

        // Short-circuit and return 401 Unauthorized
        let resp = HttpResponse::Unauthorized()
            .append_header(("content-type", "text/plain; charset=utf-8"))
            .body("Unauthorized: Missing or Invalid API Key");

        // Convert into a ServiceResponse with a boxed body to satisfy types
        return Ok(req.into_response(resp).map_into_boxed_body());
    }

    // Authorized -> Call the next service
    let res = next.call(req).await?;
    Ok(res.map_into_boxed_body())
}
