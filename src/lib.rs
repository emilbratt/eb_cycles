//! `eb_cycles` provides an easy way to put your thread to sleep during set intervals, or.. "cycles".
//!
//! ## Examples
//!
//! ```
//! use eb_cycles::{chrono_tz, SleepCycle};
//!
//! // First we need to choose a chrono_tz timezone to work with (re-exported from this library)
//! // See https://docs.rs/chrono-tz/latest/chrono_tz/ for docs about timezones.
//! // Instantiate the sleep cycle struct with selected timezone.
//! let slp_cle = SleepCycle::from_tz(chrono_tz::Europe::Oslo);
//! // Or instantiate by sending a string literal with the timzone identifier.
//! let slp_cle = SleepCycle::from_tz_str("Europe/London");
//!
//! // Sleep until next full hour.
//! // ..if time is 15:55:10 it will sleep for 4 minutes and 50 seconds e.g. until time is 16:00)
//! slp_cle.hour();
//!
//! // Sleep until next quarter.
//! // ..if time is 11:35 it will sleep until time is 11:45)
//! slp_cle.quarter();
//!
//! // Sleep until next minute.
//! // ..if time is 09:37:19 it will sleep until time is 09:38:00)
//! slp_cle.minute();
//!
//! // This is how you would do stuff every full hour.
//! loop {
//!     slp_cle.hour(); // sleep until time is XX:00
//!     println!("time to do some stuff!");
//!     do_some_stuff_at_every_hour(); // Do recurring task.
//!     // use slp_cle.quarter() or slp_cle.minute() for every quarter or minute respectively.
//! }
//!
//! // By the way, you can easily get the current timestamp for your chosen timezone.
//! println!("Time is: {}", slp_cle.datetime_now());

#![allow(missing_docs)]

use chrono::DateTime;
pub use chrono_tz;

pub mod cycles;

pub struct SleepCycle {
    tz: chrono_tz::Tz,
}

impl SleepCycle {
    pub fn from_tz(tz: chrono_tz::Tz) -> Self {
        Self { tz }
    }

    pub fn from_tz_str(tz_str: &str) -> Self {
        let tz = if tz_str.to_lowercase() == "utc" {
            // Make it easy to choose Utc without having to explicitly write "UTC" or "Etc/UTC"
            chrono_tz::UTC
        } else {
            // The default branch where we parse the tz string..
            tz_str.parse::<chrono_tz::Tz>().unwrap()
        };

        Self { tz }
    }

    pub fn datetime_now(&self) -> DateTime<chrono_tz::Tz> {
        cycles::datetime_now(self.tz)
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
