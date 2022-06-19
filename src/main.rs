use std::{net::SocketAddr};

use axum::{
    extract::Extension,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
};
use job_scheduler::{Job, JobScheduler};
use tower_http::cors::{AllowOrigin, CorsLayer};

use handlers::{cloth, customer, order};
use errors::AppError;
use mongo::{Order, Cloth, Customer};
use crate::types::HandledResult;

mod handlers;
mod mongo;
mod errors;
mod types;

#[tokio::main]
async fn main() -> HandledResult<()> {
    let mut scheduler = JobScheduler::new();

    scheduler.add(
        Job::new("0 59 23 * * *".parse().unwrap(),
                 || { println!("{}", 123); })
    );

    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let (
        orders,
        clothes,
        customers
    ) = mongo::setup().await?;

    let app = order::router()
        .merge(cloth::router())
        .merge(customer::router())
        .layer(cors_layer)
        .layer(Extension(orders))
        .layer(Extension(clothes))
        .layer(Extension(customers));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();

    Ok(())
}
