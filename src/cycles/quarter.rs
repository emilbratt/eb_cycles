use std::{
    thread,
    time::Duration,
};

use chrono::{Timelike, DateTime};
use chrono_tz;

const SLEEP_DURATION: Duration = Duration::from_millis(200);

use super::datetime_now;

pub fn sleep(tz: chrono_tz::Tz) -> DateTime<chrono_tz::Tz> {
    let now = datetime_now(tz);

    let target_minute = match now.minute() {
        0..=14 => 15,
        15..=29 => 30,
        30..=44 => 45,
        45..=60 => 0, // 60 to include leap seconds
        m => panic!("'{}' is an invalid value for minute", m),
    };

    while datetime_now(tz).minute() != target_minute {
        thread::sleep(SLEEP_DURATION);
    }

    // We should be on 0 second and on target minute.
    let now = datetime_now(tz);
    assert_eq!(now.second(), 0);
    assert_eq!(now.minute(), target_minute);

    datetime_now(tz)
}
