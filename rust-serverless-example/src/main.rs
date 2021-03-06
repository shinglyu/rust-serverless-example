//extern crate rusoto_core;
//extern crate rusoto_dynamodb;

use std::collections::HashMap;
use std::error::Error;
use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_http::http::{StatusCode, Response};
use lambda_runtime::{error::HandlerError, Context};
use log::{self};
use simple_logger;
use serde_json::json;

use rusoto_core::{Region};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(handler);

    Ok(())
}

fn handler(
    req: Request,
    _: Context,
    ) -> Result<impl IntoResponse, HandlerError> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response

    let client = DynamoDbClient::new(Region::UsEast1);

    let response = match req.query_string_parameters().get("giftcode") {
        Some(giftcode) => {
            let mut key: HashMap<String, AttributeValue> = HashMap::new();
            key.insert("giftcode".to_string(), AttributeValue { s: Some(giftcode.to_string()), ..Default::default() });

            let get_item_input: GetItemInput = GetItemInput {
                table_name: "gift-codes".to_string(),
                key: key,
                ..Default::default()
                    /*
                       attributes_to_get: None, // deprecated
                       consistent_read: None,
                       expression_attribute_names: None,
                       projection_expression: None, // return everything
                       return_consumed_capacity: None,
                       */
            };

            match client.get_item(get_item_input).sync() {
                Ok(output) => {
                    println!("{:?}", output);
                    match output.item {
                        Some(item) => {
                            json!({
                                "giftcode": item.get("giftcode").unwrap().s,
                                "status": item.get("status").unwrap().s,
                            }).into_response()
                        }
                        None => {
                            Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body("Gift code not found".into())
                                .expect("Failed to render response")
                        }
                    }
                }
                Err(error) => {
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("{:?}", error).into())
                        .expect("Failed to render response")
                }
            }
        }
        None => {
            Response::builder()
                .status(StatusCode::BAD_REQUEST) // 400
                .body("Missing parameter: giftcode".into())
                .expect("Failed to render response")
        }
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
