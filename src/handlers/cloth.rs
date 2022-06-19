use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::routing::{delete, get, post};
use bson::doc;
use futures::StreamExt;
use mongodb::results::{DeleteResult, InsertOneResult};
use serde::Deserialize;

use tauri::{object_id,  PinyinForString, time_now};

use crate::Cloth;
use crate::types::{ApiResult, SharedClothes};

pub fn router() -> Router {
    Router::new()
        .route("/api/clothes", get(get_clothes))
        // .route("/api/clothes/count", get(count_clothes))
        .route("/api/cloth", post(insert_cloth))
        .route("/api/cloth/:id", delete(delete_cloth))
}

#[derive(Deserialize)]
pub struct Params {
    pub merchant: String,
}

pub async fn get_clothes(
    clothes: SharedClothes,
    Query(queries): Query<Params>,
) -> ApiResult<Json<Vec<Cloth>>> {
    let filter = doc! {"merchant": queries.merchant};

    let mut cursors = clothes.find(filter, None).await?;
    let mut res: Vec<Cloth> = Vec::new();

    while let Some(cloth) = cursors.next().await {
        res.push(cloth?);
    }

    Ok(Json(res))
}

pub async fn insert_cloth(
    clothes: SharedClothes,
    Json(cloth): Json<Cloth>,
) -> ApiResult<Json<InsertOneResult>> {
    let cloth = Cloth {
        id: object_id(),
        pinyin: cloth.name.to_pinyin(),
        create_at: time_now(),
        ..cloth
    };

    let res = clothes.insert_one(cloth, None).await?;

    Ok(Json(res))
}

// pub async fn update_cloth() {}

pub async fn delete_cloth(
    clothes: SharedClothes,
    Path(id): Path<String>,
) -> ApiResult<Json<DeleteResult>> {
    let filter = doc! {"_id": id};
    let res = clothes.delete_one(filter, None).await?;

    Ok(Json(res))
}
