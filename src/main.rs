use actix_web::{main, middleware::Logger, web::Data, App, HttpServer};
mod lib;
mod routes;

use lib::{parser::convert_csv, prayer::Prayer};

#[main]
async fn main() -> eyre::Result<()> {
    let web_data = Data::new(Prayer {
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
            .service(routes::root::main)
            .service(routes::today::main)
            .service(routes::next::main)
            .app_data(web_data.clone())
            .wrap(Logger::default())
            .wrap(actix_cors::Cors::permissive())
    })
    .bind(("127.0.0.1", 2347))?
    .run()
    .await?;

    Ok(())
}
