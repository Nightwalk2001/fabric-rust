use std::sync::Arc;
use axum::Extension;
use mongodb::Collection;
use crate::AppError;
use crate::mongo::{Cloth, Customer, Order};

pub type HandledResult<T, E = AppError> = Result<T, E>;
pub type ApiResult<T, E = AppError> = Result<T, E>;

pub type SharedCollection<T> = Extension<Arc<Collection<T>>>;

pub type SharedOrders = SharedCollection<Order>;
pub type SharedClothes = SharedCollection<Cloth>;
pub type SharedCustomers = SharedCollection<Customer>;
