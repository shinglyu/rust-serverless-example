use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

fn main() {
    lambda!(handler)
}

fn handler(
    req: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    let response = match req.query_string_parameters().get("first_name") {
        Some(first_name) => json!({
            "message": format!("Hello, {}!", first_name),
        }).into_response(),
        None => json!({
            "message": "Hello there!"
        }).into_response()
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_handles() {
        let request = Request::default();
        let expected = json!({
            "message": "Hello there!"
        }).into_response();
        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
