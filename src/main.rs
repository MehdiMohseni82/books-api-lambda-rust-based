use axum::http::StatusCode;
use axum::{
    extract::Path,
    response::Json,
    routing::get,
    Router,
};
use axum::response::IntoResponse;
use lambda_http::{run, tracing, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_dynamo::to_attribute_value;
use std::env::set_var;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Clone, Deserialize, Serialize, Debug)]
struct BookDto {
    id: Option<String>,
    category: Option<String>,
    name: Option<String>
}

async fn get_book_by_id(Path(id): Path<String>) -> impl IntoResponse {

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    tracing::info!("Received GET /book with id: {:?}", id);

    let table_name = "booksTable";
    let pk = "PK";
    let sk = "SK";

    match client
        .query()
        .table_name(table_name)
        .key_condition_expression("#pk = :pk AND begins_with(#sk, :sk_prefix)")
        .expression_attribute_names("#pk", pk)
        .expression_attribute_names("#sk", sk)
        .expression_attribute_values(":pk", AttributeValue::S("book".to_string()))
        .expression_attribute_values(":sk_prefix", AttributeValue::S(format!("{}#", id)))
        .send()
        .await
    {
        Ok(result) => {
            if let Some(items) = result.items {
                if !items.is_empty() {
                    let item_json: Value = serde_dynamo::from_item(items[0].clone()).unwrap_or_else(|_| json!({ "error": "Failed to deserialize item" }));
                    Json(item_json).into_response()
                } else {
                    Json(json!({ "error": "Item not found" })).into_response()
                }
            } else {
                Json(json!({ "error": "Item not found" })).into_response()
            }
        },
        Err(e) => {
            tracing::error!("Failed to query items: {:?}", e);
            Json(json!({ "error": "Failed to query items" })).into_response()
        }
    }
    
}

async fn get_all_books() -> impl IntoResponse {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    tracing::info!("Received GET request to fetch all books");

    let table_name = "booksTable";
    let pk = "PK";

    match client
        .query()
        .table_name(table_name)
        .key_condition_expression("#pk = :pk")
        .expression_attribute_names("#pk", pk)
        .expression_attribute_values(":pk", AttributeValue::S("book".to_string()))
        .send()
        .await
    {
        Ok(result) => {
            if let Some(items) = result.items {
                // Convert the retrieved items to JSON
                let items_json: Vec<Value> = items.into_iter()
                    .map(|item| serde_dynamo::from_item(item).unwrap_or_else(|_| json!({ "error": "Failed to deserialize item" })))
                    .collect();
                Json(items_json).into_response()
            } else {
                Json(json!({ "error": "No items found" })).into_response()
            }
        },
        Err(e) => {
            tracing::error!("Failed to query items: {:?}", e);
            Json(json!({ "error": "Failed to query items" })).into_response()
        }
    }
}

async fn get_books_by_category(Path(category): Path<String>) -> impl IntoResponse {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    tracing::info!("Received GET /books with category: {:?}", category);

    let table_name = "booksTable";
    let pk = "PK";
    let sk = "SK";

    match client
        .query()
        .table_name(table_name)
        .key_condition_expression("#pk = :pk")
        .filter_expression("contains(#sk, :category)")
        .expression_attribute_names("#pk", pk)
        .expression_attribute_names("#sk", sk)
        .expression_attribute_values(":pk", AttributeValue::S("book".to_string()))
        .expression_attribute_values(":category", AttributeValue::S(format!("#{}", category)))
        .send()
        .await
    {
        Ok(result) => {
            if let Some(items) = result.items {
                // Convert the retrieved items to JSON
                let items_json: Vec<Value> = items.into_iter()
                    .map(|item| serde_dynamo::from_item(item).unwrap_or_else(|_| json!({ "error": "Failed to deserialize item" })))
                    .collect();
                Json(items_json).into_response()
            } else {
                Json(json!({ "error": "No items found" })).into_response()
            }
        },
        Err(e) => {
            tracing::error!("Failed to query items in DynamoDB: {}", e);
            Json(json!({ "error": "Failed to query items" })).into_response()
        }
    }
}

async fn add_new_book(Json(payload): Json<BookDto>) -> Result<Json<Value>, (StatusCode, String)> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    tracing::info!("Received POST /books with payload: {:?}", payload);

    match add_item(&client, payload.clone(), "booksTable").await {
        Ok(_) => Ok(Json(json!({ "msg": "Item added to DynamoDB" }))),
        Err(e) => {
            tracing::error!("Failed to add item to DynamoDB: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to add item to DynamoDB".to_string()))
        }
    }
}

async fn health_check() -> (StatusCode, String) {
    let health = true;
    match health {
        true => (StatusCode::OK, "Healthy!".to_string()),
        false => (StatusCode::INTERNAL_SERVER_ERROR, "Not healthy!".to_string()),
    }
}

async fn add_item(client: &Client, item: BookDto, table: &str) -> Result<(), Error> {
    let pk_av = to_attribute_value("book".to_string())?;
    let sk_av = to_attribute_value(format!("{}#{}", item.id.unwrap(), item.category.unwrap()))?;
    let name_av = to_attribute_value(item.name)?;

    let request = client
        .put_item()
        .table_name(table)
        .item("PK", pk_av)
        .item("SK", sk_av)
        .item("name", name_av);

    tracing::info!("Adding item to DynamoDB");

    let resp = request.send().await?;

    tracing::info!("DynamoDB response: {:?}", resp);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/books/:id", get(get_book_by_id))
        .route("/books/:category/category", get(get_books_by_category))
        .route("/books", get(get_all_books).post(add_new_book))
        .route("/health/", get(health_check));

    run(app).await
}