//! The daily trigger.
//!
//! Requires the `daily_trigger` feature.

// #[cfg(feature = "file")]
// use serde::de;
#[cfg(feature = "file")]
use serde_derive::Deserialize;
use std::error::Error;
// #[cfg(feature = "file")]
// use std::fmt;

use chrono::{Datelike, Local};

use crate::append::rolling_file::LogFile;
use crate::append::rolling_file::policy::compound::trigger::Trigger;
#[cfg(feature = "file")]
use crate::file::{Deserialize, Deserializers};

static mut DAY: u32 = 0;

/// Configuration for the daily trigger.
#[cfg(feature = "file")]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DailyTriggerConfig {
}

/// A trigger which rolls the log once it has passed a certain size.
#[derive(Debug)]
pub struct DailyTrigger {
}

impl DailyTrigger {
    /// Returns a new trigger which rolls the log once it has passed the
    /// specified size in bytes.
    pub fn new() -> DailyTrigger {
        DailyTrigger {}
    }
}

impl Trigger for DailyTrigger {
    fn trigger(&self, _file: &LogFile) -> Result<bool, Box<dyn Error + Sync + Send>> {
        let mut _roll = false;
        unsafe {
            let last_day = DAY;
            DAY = Local::today().day();
            _roll = last_day != 0 && DAY != last_day;
        }
        Ok(_roll)
    }
}

/// A deserializer for the `DailyTrigger`.
///
/// # Configuration
///
#[cfg(feature = "file")]
pub struct DailyTriggerDeserializer;

#[cfg(feature = "file")]
impl Deserialize for DailyTriggerDeserializer {
    type Trait = dyn Trigger;

    type Config = DailyTriggerConfig;

    fn deserialize(
        &self,
        _config: DailyTriggerConfig,
        _: &Deserializers,
    ) -> Result<Box<dyn Trigger>, Box<dyn Error + Sync + Send>> {
        Ok(Box::new(DailyTrigger::new()))
    }
}
