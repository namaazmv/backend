use hijri_date::HijriDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct IsHijriDate {
    pub day: usize,
    pub month: usize,
    pub year: usize,
    pub day_name: String,
    pub month_name: String,
    pub month_name_eng: String,
}

pub fn today_date() -> IsHijriDate {
    let t = HijriDate::today();

    IsHijriDate {
        day: t.day,
        month: t.month,
        year: t.year,
        day_name: t.day_name,
        month_name: t.month_name,
        month_name_eng: is_month_eng(t.month),
    }
}

pub fn is_month_eng(month: usize) -> String {
    match month {
        1 => "Muḥarram".to_owned(),
        2 => "Safar".to_owned(),
        3 => "Rabīʿ al-Awwal".to_owned(),
        4 => "Rabī’ al-Ākhir".to_owned(),
        5 => "Jumādá al-Ūlá".to_owned(),
        6 => "Jumādá al-Ākhirah".to_owned(),
        7 => "Rajab".to_owned(),
        8 => "Sha‘bān".to_owned(),
        9 => "Ramaḍān".to_owned(),
        10 => "Shawwāl".to_owned(),
        11 => "Dhū al-Qa‘dah".to_owned(),
        12 => "Dhū al-Ḥijjah".to_owned(),
        _ => "".to_owned(),
    }
}
