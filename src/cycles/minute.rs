use std::{
    thread,
    time::Duration,
};

use chrono::{Timelike, DateTime};
use chrono_tz;

const SLEEP_DURATION: Duration = Duration::from_millis(100);

use super::datetime_now;

pub fn sleep(tz: chrono_tz::Tz) -> DateTime<chrono_tz::Tz> {
    while datetime_now(tz).second() == 0 {
        thread::sleep(SLEEP_DURATION);
    }

    while datetime_now(tz).second() != 0 {
        thread::sleep(SLEEP_DURATION);
    }

    // We should be on 0 second.
    assert_eq!(datetime_now(tz).second(), 0);

    datetime_now(tz)
}
