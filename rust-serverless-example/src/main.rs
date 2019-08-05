//extern crate rusoto_core;
//extern crate rusoto_dynamodb;

use std::collections::HashMap;
use std::error::Error;
use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_http::http::{StatusCode, Response};
use lambda_runtime::{error::HandlerError, Context};
use log::{self, info};
use simple_logger;
use serde_json::json;

use rusoto_core::{Region, RusotoError};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient,
    GetItemInput, GetItemOutput, GetItemError, ListTablesInput};

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

    let mut key: HashMap<String, AttributeValue> = HashMap::new();
    key.insert("giftcode".to_string(), AttributeValue { s: Some("ABC124".to_string()), ..Default::default() });

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
                    return Ok(json!({
                        "giftcode": item.get("giftcode").unwrap().s,
                        "status": item.get("status").unwrap().s,
                    }).into_response())
                }
                None => {
                    return Ok(
                        Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body("Gift code not found".into())
                            .expect("Failed to render response")
                    )
                }
            }
        }
        Err(error) => {
            return Ok(
                Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("{:?}", error).into())
                .expect("Failed to render response")
            )
        }
    }

    return Ok(
        Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Unhandled error".into())
        .expect("Failed to render response")
    )
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
