use chrono::{DateTime, NaiveDate, Local, Utc};

use log;

/// If the provided dt is special and == "TODAY", this returns
/// a string representing the current day in the Local timezone
fn adjust_dt(dt: &Option<String>) -> String {

    match dt {
        Some(dt) => {return dt.to_string();},
        None => {
            log::warn!("No date provided, setting automatically as TODAY from local timezone, be sure to double check this is correct!");
            let now_local: DateTime<Local> = Local::now();
            log::trace!("Date is set to the default TODAY: {:?}", now_local);
            let now_as_str = now_local.format("%d-%b-%y").to_string();
            log::trace!("Now in local timezone is {:}", now_as_str);
            return now_as_str;
        }
    }
}

/// Matches the provided datetime string to the correct format, or
/// panics if it's not able to.
fn match_dt(dt: &String) -> NaiveDate {
    match NaiveDate::parse_from_str(dt, "%d-%b-%y") {
        Ok(date_only) => { 
            log::trace!("Date parsed to {:?}", date_only);
            return date_only;
        },

        // Catch every error log and exit. These are all unrecoverable
        // and as such we panic if any error is detected
        Err(e) => {
            log::error!("Error parsing date with error {:?}", e);
            log::error!("Provided datetime which caused the error was {:}", dt);
            panic!();
        }
    }
}


/// Gets the current timestamp/epoch
pub fn get_current_timestamp() -> i64 {
    let now_utc: DateTime<Utc> = Utc::now();
    log::trace!("Date in UTC is {:?}", now_utc);
    let timestamp: i64 = now_utc.timestamp();
    log::trace!("Timestamp is {}", timestamp);
    return timestamp;   
}

pub fn parse_command_line_date(dt: &Option<String>) -> NaiveDate {
    let dt: String = adjust_dt(&dt);
    let date_only: NaiveDate = match_dt(&dt);
    return date_only;
}

