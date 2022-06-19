use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router,
    routing::{delete, get, post},
};
use bson::doc;
use chrono::Utc;
use futures::StreamExt;
use mongodb::options::FindOptions;
use mongodb::results::{DeleteResult, InsertOneResult};
use serde::Deserialize;

use tauri::{object_id, PAGE_SIZE};

use crate::{AppError, Order};
use crate::types::{ApiResult, SharedOrders};

pub fn router() -> Router {
    Router::new()
        .route("/api/orders/:page", get(get_orders))
        .route("/api/orders/count", get(count_orders))
        .route("/api/order", post(insert_order))
        .route("/api/order/:id", delete(delete_order))
}

#[derive(Deserialize)]
pub struct Params {
    pub merchant: String,
}

pub async fn get_orders(
    orders:SharedOrders,
    Path(page): Path<usize>,
    Query(queries): Query<Params>,
) -> ApiResult<Json<Vec<Order>>> {
    let filter = doc! {
        "merchant": queries.merchant
    };

    let mut options = FindOptions::default();
    options.skip = Some(11);

    let mut cursors = orders
        .find(filter, None).await?
        .skip((page - 1) * PAGE_SIZE)
        .take(PAGE_SIZE);
    let mut res: Vec<Order> = Vec::new();
    while let Some(order) = cursors.next().await {
        res.push(order?);
    }

    Ok(Json(res))
}

pub async fn count_orders(
    orders:SharedOrders,
    Query(queries): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    let filter = doc! {"merchant": queries.merchant};
    let count = orders.count_documents(filter, None).await?;

    Ok((StatusCode::OK, Json(count)))
}

pub async fn insert_order(
    orders:SharedOrders,
    Json(order): Json<Order>,
) -> ApiResult<Json<InsertOneResult>> {
    let order = Order {
        id: object_id(),
        create_at: Option::from(Utc::now()),
        ..order
    };

    let res = orders.insert_one(order, None).await?;

    Ok(Json(res))
}

pub async fn delete_order(
    orders:SharedOrders,
    Path(id): Path<String>,
) -> ApiResult<Json<DeleteResult>> {
    let filter = doc! {"_id": id};
    let res = orders.delete_one(filter, None).await?;

    Ok(Json(res))
}
