use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::routing::{delete, get, post, put};
use bson::doc;
use futures::StreamExt;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::Deserialize;

use tauri::{object_id, PinyinForString, time_now};

use crate::{Customer};
use crate::types::{ApiResult, SharedCustomers};

pub fn router() -> Router {
    Router::new()
        .route("/api/customers", get(get_customers))
        .route("/api/customer", post(insert_customer))
        .route("/api/customer/:id", put(update_customer))
        .route("/api/customer/:id", delete(delete_customer))
}

#[derive(Deserialize)]
pub struct Params {
    pub merchant: String,
}

pub async fn get_customers(
    customers:SharedCustomers,
    Query(queries): Query<Params>,
) -> ApiResult<Json<Vec<Customer>>> {
    let filter = doc! {"merchant": queries.merchant};

    let mut cursors = customers.find(filter, None).await?;
    let mut res: Vec<Customer> = Vec::new();

    while let Some(customer) = cursors.next().await {
        res.push(customer?);
    }

    Ok(Json(res))
}

pub async fn insert_customer(
    customers:SharedCustomers,
    Json(customer): Json<Customer>,
) -> ApiResult<Json<InsertOneResult>> {
    let customer = Customer {
        id: object_id(),
        pinyin: customer.name.to_pinyin(),
        create_at: time_now(),
        ..customer
    };

    let res = customers.insert_one(customer, None).await?;

    Ok(Json(res))
}

pub async fn update_customer(
    customers:SharedCustomers,
    Path(id): Path<i32>,
    Json(customer): Json<Customer>,
) -> ApiResult<Json<UpdateResult>> {
    let filter = doc! {"_id":id};
    let res = customers.replace_one(filter, customer, None).await?;

    Ok(Json(res))
}

pub async fn delete_customer(
    customers:SharedCustomers,
    Path(id): Path<String>,
) -> ApiResult<Json<DeleteResult>> {
    let filter = doc! {"_id": id};
    let res = customers.delete_one(filter, None).await?;

    Ok(Json(res))
}
