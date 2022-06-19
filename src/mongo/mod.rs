use std::sync::Arc;
use mongodb::{Client, Collection, Database};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::types::HandledResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct CartItem {
    name: String,
    color: String,
    #[serde(rename = "fullUnit")]
    full_unit: String,
    #[serde(rename = "fullCount", skip_serializing_if = "Option::is_none")]
    full_count: Option<f64>,
    unit: String,
    count: f64,
    price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    money: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub date: String,
    pub merchant: String,
    pub clerk: String,
    pub client: String,
    pub cart: Vec<CartItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overdraft: Option<f64>,
    #[serde(rename = "isReturn")]
    pub is_return: bool,
    #[serde(rename = "createAt", skip_serializing_if = "Option::is_none")]
    pub create_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cloth {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub merchant: String,
    pub name: String,
    pub pinyin: Option<String>,
    pub unit: String,
    pub price: f64,
    #[serde(rename = "createAt", skip_serializing_if = "Option::is_none")]
    pub create_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub phone: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinyin: Option<String>,
    pub merchant: String,
    #[serde(rename = "createAt", skip_serializing_if = "Option::is_none")]
    pub create_at: Option<DateTime<Utc>>,
}

pub async fn setup() -> HandledResult<(
    Arc<Collection<Order>>,
    Arc<Collection<Cloth>>,
    Arc<Collection<Customer>>
)> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;

    let db: Database = client.database("tauri");

    let orders = db.collection::<Order>("orders");
    let clothes = db.collection::<Cloth>("clothes");
    let customers = db.collection::<Customer>("customers");

    Ok((
        Arc::new(orders),
        Arc::new(clothes),
        Arc::new(customers)
    ))
}
