use eb_cycles::{chrono_tz, SleepCycle};

const CYCLE_MAX_COUNT: usize = 10;

#[test]
fn hour_oslo() {
    let slp_cle = SleepCycle::from_tz_str("Europe/Oslo");

    for i in 1..=CYCLE_MAX_COUNT {
        println!("hour_oslo(): cycle count = {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.hour();
    }

    println!("hour_oslo(): completed @ {}", slp_cle.datetime_now());
}

#[test]
fn hour_utc() {
    let slp_cle = SleepCycle::from_tz(chrono_tz::UTC);

    for i in 1..=CYCLE_MAX_COUNT {
        println!("hour_utc(): cycle count = {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.hour();
    }

    println!("hour_utc(): completed @ {}", slp_cle.datetime_now());
}

#[test]
fn minute_oslo() {
    let slp_cle = SleepCycle::from_tz(chrono_tz::Europe::Oslo);

    for i in 1..=CYCLE_MAX_COUNT {
        println!("minute_oslo(): cycle count = {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.minute();
    }

    println!("minute_oslo(): completed @ {}", slp_cle.datetime_now());
}

#[test]
fn minute_utc() {
    let slp_cle = SleepCycle::from_tz_str("UTC");

    for i in 1..=CYCLE_MAX_COUNT {
        println!("minute_utc(): cycle count = {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.minute();
    }

    println!("minute_utc(): completed @ {}", slp_cle.datetime_now());
}

#[test]
fn quarter_oslo() {
    let slp_cle = SleepCycle::from_tz(chrono_tz::Europe::Oslo);

    for i in 1..=CYCLE_MAX_COUNT {
        println!("quarter_oslo(): cycle count = {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.quarter();
    }

    println!("quarter_oslo(): completed @ {}", slp_cle.datetime_now());
}

#[test]
fn quarter_utc() {
    let slp_cle = SleepCycle::from_tz(chrono_tz::UTC);

    for i in 1..=CYCLE_MAX_COUNT {
        println!("quarter_utc(): cycle count = {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.quarter();
    }

    println!("quarter_utc(): completed @ {}", slp_cle.datetime_now());
}
