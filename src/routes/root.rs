use actix_web::{get, web::Json, Error, Responder};
use serde_json::json;

#[get("/")]
async fn main() -> Result<impl Responder, Error> {
    Ok(Json(json!({
        "status": 200,
        "message": "Hello, world"
    })))
}
