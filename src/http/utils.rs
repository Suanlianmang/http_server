
use std::time::SystemTime;


#[derive(Debug)]
pub struct DateTime{
    day: u8,
    month: u8,
    year: u16,
    hour: u16,
    minute: u16,
    second: u16,
    millisecond: u32,
    timestamp: u128 
}

impl DateTime {
    fn is_leap_year(year: i32) -> bool {
        // if year % 4 == 0 {
        //     return true;
        // }
        // false
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn days_in_month(year: i32, month: i32) -> i32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31, 
            4 | 6 | 9 | 11 => 30,
            2 => if Self::is_leap_year(year) {29} else {28},
            _ => 0
        }
    }
    pub fn now() -> Option<DateTime> {
        if let Ok(epoch_time) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
            return Some(Self::timestamp_to_datetime(epoch_time.as_millis()));
        }
        None

    }
    pub fn timestamp_to_datetime(epoch_time: u128) -> DateTime {
        let millis_in_minute = 60 * 1000;
        let millis_in_hour = 60 * millis_in_minute;
        let millis_in_day = 24 * millis_in_hour;

        let milliseconds = epoch_time % 1000;
        let seconds = (epoch_time / 1000) % 60;
        let minutes = (epoch_time/ millis_in_minute) % 60;
        let hours = (epoch_time/ millis_in_hour) % 24;


        let mut days_since_epoch = epoch_time / millis_in_day; 
        println!("Test: {}", epoch_time % millis_in_day);
        println!("Day: {}", days_since_epoch);
        let mut years = 1970;
        let mut days_in_year = if Self::is_leap_year(years) { 366 } else { 365 };
        while days_since_epoch >= days_in_year {
            days_since_epoch -= days_in_year;
            years += 1;
            days_in_year = if Self::is_leap_year(years) { 366 } else { 365 };
        }

        let mut day_in_this_year : i32 = days_since_epoch as i32;
        let mut month = 1;
        let mut days_in_month = Self::days_in_month(years, month);
        while day_in_this_year >= days_in_month {
            day_in_this_year -= days_in_month; 
            month += 1;
            days_in_month = Self::days_in_month(years, month)
        }

        DateTime{
            timestamp: epoch_time,
            millisecond: milliseconds as u32,
            second: seconds as u16,
            minute: minutes as u16,
            hour: hours as u16,
            day: day_in_this_year as u8 + 1,
            month: month as u8,
            year: years as u16
        }

    }
}
