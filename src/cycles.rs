use chrono::DateTime;
use chrono_tz;

pub mod hour;
pub mod minute;
pub mod quarter;

pub fn time_now(tz: chrono_tz::Tz) -> DateTime<chrono_tz::Tz> {
    chrono::Utc::now().with_timezone(&tz)
}
