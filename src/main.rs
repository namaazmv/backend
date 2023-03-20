use actix_web::{
    get,
    http::Error,
    main,
    middleware::Logger,
    web::{self, Json},
    App, HttpServer, Responder, Result,
};
use chrono::Local;
use serde_json::json;
mod lib;

use lib::{
    parser::convert_csv,
    prayer::Prayer,
    structs::{DataQuery, SalatError},
};

use crate::lib::utils::days_into_year;

#[get("/")]
async fn root() -> Result<impl Responder, Error> {
    Ok(Json(json!({
        "status": 200,
        "message": "Hello, world"
    })))
}

#[get("/today")]
async fn today(
    data: web::Data<Prayer>,
    query: web::Query<DataQuery>,
) -> Result<impl Responder, SalatError> {
    let island = &data.get_island(query.island).ok_or_else(|| SalatError {
        message: "Island not found".to_owned(),
    })?;
    let prayer_today = &data.get_today(island.to_owned());

    Ok(Json(json!({
        "island": island,
        "prayer": prayer_today.clone().ok_or_else(|| SalatError {
            message: "Prayer for today not found.".to_owned(),
        })?,
    })))
}

#[get("/next")]
async fn next(
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
            lib::utils::convert_timestamp_to_date(
                prayer_today.get_value(p.to_owned().to_owned()).into(),
            )
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
        json!({ "call": &new_call, "timestamp": new_prayer.get_value(new_call), "island": island }),
    ))
}

#[main]
async fn main() -> eyre::Result<()> {
    let web_data = web::Data::new(Prayer {
        atolls: convert_csv("atolls".to_owned())?,
        islands: convert_csv("islands".to_owned())?,
        prayers: convert_csv("prayertimes".to_owned())?,
        timings: vec![
            "fajr".to_owned(),
            "sunrise".to_owned(),
            "duhr".to_owned(),
            "asr".to_owned(),
            "maghrib".to_owned(),
            "isha".to_owned(),
        ],
    });

    lib::log::init()?;

    HttpServer::new(move || {
        App::new()
            .service(root)
            .service(today)
            .service(next)
            .app_data(web_data.clone())
            .wrap(Logger::default())
            .wrap(actix_cors::Cors::permissive())
    })
    .bind(("127.0.0.1", 2347))?
    .run()
    .await?;

    Ok(())
}
