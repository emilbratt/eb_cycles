use eb_cycles::{chrono_tz, SleepCycle};

const CYCLE_MAX_COUNT: usize = 10;

#[test]
fn hour_oslo() {
    println!("hour_oslo()");

    let slp_cle = SleepCycle::from_tz_str("Europe/Oslo");

    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.hour();
    }

    println!("completed @ {}", slp_cle.datetime_now());
}

#[test]
fn hour_utc() {
    println!("hour_utc()");

    let slp_cle = SleepCycle::from_tz(chrono_tz::UTC);

    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.hour();
    }

    println!("completed @ {}", slp_cle.datetime_now());
}

#[test]
fn minute_oslo() {
    println!("minute_oslo()");

    let slp_cle = SleepCycle::from_tz(chrono_tz::Europe::Oslo);

    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.minute();
    }

    println!("completed @ {}", slp_cle.datetime_now());
}

#[test]
fn minute_utc() {
    println!("minute_utc()");

    let slp_cle = SleepCycle::from_tz_str("UTC");

    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.minute();
    }

    println!("completed @ {}", slp_cle.datetime_now());
}

#[test]
fn quarter_oslo() {
    println!("quarter_oslo()");

    let slp_cle = SleepCycle::from_tz(chrono_tz::Europe::Oslo);

    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.quarter();
    }

    println!("completed @ {}", slp_cle.datetime_now());
}

#[test]
fn quarter_utc() {
    println!("quarter_utc()");

    let slp_cle = SleepCycle::from_tz(chrono_tz::UTC);

    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, slp_cle.datetime_now());
        slp_cle.quarter();
    }

    println!("completed @ {}", slp_cle.datetime_now());
}
