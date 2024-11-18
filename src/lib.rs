//! `eb_cycles` provides an easy way to put your thread to sleep during set intervals, or.. "cycles".

//! # Examples to come..
//!
//!

#![allow(missing_docs)]

use chrono::DateTime;
pub use chrono_tz;

pub mod cycles;

pub trait TCycle {
    fn new(tz: chrono_tz::Tz) -> Self;

    fn now(&self) -> DateTime<chrono_tz::Tz>;

    fn sleep(&mut self) -> DateTime<chrono_tz::Tz>;
}

pub struct SleepCycle {
    tz: chrono_tz::Tz,
}

impl SleepCycle {
    pub fn new(tz: chrono_tz::Tz) -> Self {
        Self { tz }
    }

    pub fn time_now(&self) -> DateTime<chrono_tz::Tz> {
        cycles::time_now(self.tz)
    }

    pub fn hour(&self) -> DateTime<chrono_tz::Tz> {
        cycles::hour::sleep(self.tz)
    }

    pub fn minute(&self) -> DateTime<chrono_tz::Tz> {
        cycles::minute::sleep(self.tz)
    }

    pub fn quarter(&self) -> DateTime<chrono_tz::Tz> {
        cycles::quarter::sleep(self.tz)
    }
}
