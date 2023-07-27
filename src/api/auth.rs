use crate::data::auth::{LoginData, RegisterData};
use actix_web::web::Json;
use actix_web::{post, Responder};
use anyhow::Result;

#[post("/auth/login")]
async fn login(data: Json<LoginData>) -> impl Responder {
    "todo"
}

#[post("/auth/register")]
async fn register(data: Json<RegisterData>) -> impl Responder {
    "todo"
}
