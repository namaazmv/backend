use actix_web::{
    get,
    web::{self, Json},
    Responder,
};
use serde_json::json;

use crate::lib::{
    is_date::today_date,
    prayer::Prayer,
    structs::{DataQuery, SalatError},
};

#[get("/today")]
async fn main(
    data: web::Data<Prayer>,
    query: web::Query<DataQuery>,
) -> Result<impl Responder, SalatError> {
    let island = &data.get_island(query.island).ok_or_else(|| SalatError {
        message: "Island not found".to_owned(),
    })?;
    let prayer_today = &data.get_today(island.to_owned());

    Ok(Json(json!({
        "today": prayer_today.clone().ok_or_else(|| SalatError {
            message: "Prayer for today not found.".to_owned(),
        })?,
              "island": island,
        "hijri_date": today_date()
    })))
}
