use chrono::{DateTime, Datelike, FixedOffset, Local, TimeZone, Timelike, Utc};

pub fn convert_timestamp_to_date(timestamp: i64) -> Result<DateTime<Local>, String> {
    let now = Local::now();

    let new_timestamp: u32 = timestamp.try_into().unwrap();

    let hours = new_timestamp / 60;
    let minutes = new_timestamp % 60;

    Ok(now
        .with_hour(hours)
        .ok_or("Failed to set hour.")?
        .with_minute(minutes)
        .ok_or("Failed to parse minute")?
        .with_second(0)
        .unwrap())
}

pub fn days_into_year(date: chrono::DateTime<Local>) -> u32 {
    if date.month() == 1 && date.day() == 1 {
        return 0;
    }

    let start_of_year = chrono::Local
        .with_ymd_and_hms(date.year(), 1, 1, 0, 0, 0)
        .unwrap();

    (date.signed_duration_since(start_of_year).num_days() as u32 % 366) + 1
}

fn convert_js_timestamp_to_date(timestamp: i64) -> DateTime<Local> {
    DateTime::<Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(timestamp / 1000, 0).unwrap(),
        Utc,
    )
    .with_timezone(&FixedOffset::east_opt(5 * 3600).unwrap())
    .with_timezone(&Local)
}

// pub fn convert_timestamp_to_string(timestamp: i32) -> String {
//     let minutes = timestamp / 60;
//     let seconds = timestamp % 60;
//     format!("{:02}:{:02}", minutes, seconds)
// }
