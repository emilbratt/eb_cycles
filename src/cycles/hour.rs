use std::{
    thread,
    time::Duration,
};

use chrono::{Timelike, DateTime};
use chrono_tz;

const SLEEP_DURATION: Duration = Duration::from_millis(200);

use super::datetime_now;

pub fn sleep(tz: chrono_tz::Tz) -> DateTime<chrono_tz::Tz> {
    while datetime_now(tz).minute() == 0 {
        thread::sleep(SLEEP_DURATION);
    }

    let now = datetime_now(tz);

    let next_hour = (now + chrono::Duration::hours(1))
        .with_minute(0).unwrap()
        .with_second(0).unwrap();

    let sleep_time = (next_hour - now).num_seconds() - 1;

    // Sleep for the long duration until almost next hour (notice the -1 above).
    thread::sleep(Duration::from_secs(sleep_time as u64));

    // Sleep for the remaining period (ca. 1 second).
    while datetime_now(tz).second() != 0 {
        thread::sleep(SLEEP_DURATION);
    }

    // We should be on 0 for both minute and second.
    let now = datetime_now(tz);
    assert_eq!(now.minute(), 0);
    assert_eq!(now.second(), 0);

    datetime_now(tz)
}
