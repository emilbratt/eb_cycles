use std::{
    thread,
    time::Duration,
};

use chrono::{Timelike, DateTime};
use chrono_tz;

const SLEEP_DURATION: Duration = Duration::from_millis(100);

use super::time_now;

pub fn sleep(tz: chrono_tz::Tz) -> DateTime<chrono_tz::Tz> {
    while time_now(tz).second() == 0 {
        thread::sleep(SLEEP_DURATION);
    }

    while time_now(tz).second() != 0 {
        thread::sleep(SLEEP_DURATION);
    }

    assert_eq!(time_now(tz).second(), 0);

    time_now(tz)
}
