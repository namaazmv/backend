use actix_web::{
    get,
    web::{self, Json},
    Responder,
};
use chrono::Local;
use serde_json::json;

use crate::lib::{
    is_date::today_date,
    prayer::Prayer,
    structs::{DataQuery, SalatError},
    utils,
};

use crate::lib::utils::days_into_year;

#[get("/next")]
async fn main(
    data: web::Data<Prayer>,
    query: web::Query<DataQuery>,
) -> Result<impl Responder, SalatError> {
    let island = &data.get_island(query.island).ok_or_else(|| SalatError {
        message: "Island not found".to_owned(),
    })?;

    let prayer_error = SalatError {
        message: "Prayer for next not found".to_owned(),
    };

    let prayer_today = &data
        .get_today(island.clone())
        .ok_or_else(|| prayer_error.clone())?;

    let now = Local::now();

    let call = &data
        .timings
        .iter()
        .find(|p| {
            utils::convert_timestamp_to_date(prayer_today.get_value(p.to_owned().to_owned()).into())
                .expect("")
                >= now
        })
        .cloned();

    let new_call = call.as_ref().map_or("fajr".to_owned(), String::clone);
    let new_prayer = if call.is_none() {
        data.get_entry_from_day(((days_into_year(now) + 1) % 366).into(), island.clone())
            .ok_or_else(|| prayer_error)?
    } else {
        prayer_today.to_owned()
    };

    Ok(Json(
        json!({ "call": &new_call, "timestamp": new_prayer.get_value(new_call), "island": island, "hijri_date": today_date() }),
    ))
}
