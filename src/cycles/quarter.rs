use std::{
    thread,
    time::Duration,
};

use chrono::{Timelike, DateTime};
use chrono_tz;

const SLEEP_DURATION: Duration = Duration::from_millis(200);

use super::time_now;

pub fn sleep(tz: chrono_tz::Tz) -> DateTime<chrono_tz::Tz>{
    let old_minute = time_now(tz).minute();

    let target_minute = match old_minute {
        45..61 => 0, // 61 to include leap seconds (60)
        0..15 => 15,
        15..30 => 30,
        30..45 => 45,
        m => panic!("Minute is {}, panic!", m),
    };

    while time_now(tz).minute() != target_minute {
        thread::sleep(SLEEP_DURATION);
    }

    assert_eq!(time_now(tz).second(), 0);
    assert!(old_minute != time_now(tz).minute());

    time_now(tz)
}
