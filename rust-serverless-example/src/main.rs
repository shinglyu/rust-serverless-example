//extern crate rusoto_core;
//extern crate rusoto_dynamodb;

use std::error::Error;
use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_runtime::{error::HandlerError, Context};
use log::{self, info};
use simple_logger;
use serde_json::json;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

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
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).sync() {
    Ok(output) => {
      match output.table_names {
        Some(table_name_list) => {
          println!("Tables in database:");

          for table_name in table_name_list {
            println!("{}", table_name);
            return Ok(json!({
                "message": format!("Hello, {}", table_name),
            }).into_response());
          }
        }
        None => println!("No tables in database!"),
      }
    }
    Err(error) => {
      println!("Error: {:?}", error);
    }
  }
    let response = match req.query_string_parameters().get("first_name") {
        Some(first_name) => json!({
            "message": format!("Hello, {}!", first_name),
        }).into_response(),
        None => {
            info!("No first_name is provided.");
            json!({
                "message": "Hello there!"
            }).into_response()
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
