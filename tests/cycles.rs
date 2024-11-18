use chrono_tz;

use eb_cycles::SleepCycle;

const CYCLE_MAX_COUNT: usize = 10;

fn setup(tz: chrono_tz::Tz) -> SleepCycle {
    eb_cycles::SleepCycle::new(tz)
}

#[test]
fn hour_oslo() {
    println!("hour_oslo()");

    let cycle = setup(eb_cycles::chrono_tz::Europe::Oslo);
    
    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, cycle.time_now());
        cycle.hour();
    }
    println!("completed @ {}", cycle.time_now());
}


#[test]
fn hour_utc() {
    println!("hour_utc()");

    let cycle = setup(eb_cycles::chrono_tz::UTC);
    
    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, cycle.time_now());
        cycle.hour();
    }
    println!("completed @ {}", cycle.time_now());
}

#[test]
fn minute_oslo() {
    println!("minute_oslo()");

    let cycle = setup(eb_cycles::chrono_tz::Europe::Oslo);
    
    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, cycle.time_now());
        cycle.minute();
    }
    println!("completed @ {}", cycle.time_now());
}


#[test]
fn minute_utc() {
    println!("minute_utc()");

    let cycle = setup(eb_cycles::chrono_tz::UTC);
    
    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, cycle.time_now());
        cycle.minute();
    }
    println!("completed @ {}", cycle.time_now());
}


#[test]
fn quarter_oslo() {
    println!("quarter_oslo()");

    let cycle = setup(eb_cycles::chrono_tz::Europe::Oslo);
    
    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, cycle.time_now());
        cycle.quarter();
    }
    println!("completed @ {}", cycle.time_now());
}

#[test]
fn quarter_utc() {
    println!("quarter_utc()");

    let cycle = setup(eb_cycles::chrono_tz::UTC);
    
    let mut i = 0;
    while i < CYCLE_MAX_COUNT {
        i += 1;
        println!("Cycle count: {}/{} - starting new sleep cycle @ {}", i, CYCLE_MAX_COUNT, cycle.time_now());
        cycle.quarter();
    }
    println!("completed @ {}", cycle.time_now());
}
