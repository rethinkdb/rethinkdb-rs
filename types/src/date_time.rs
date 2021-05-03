use crate::{DateTime, Time};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let time = Time::deserialize(deserializer)?;
        let secs = time.epoch_time.trunc() as i64;
        // RethinkDB timestamps have millisecond precision so we need
        // to convert the milliseconds to nanoseconds first
        let msecs = time.epoch_time.fract().abs() as u32;
        let naive = chrono::NaiveDateTime::from_timestamp(secs, msecs * 1_000_000);
        let dt = chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc);
        Ok(DateTime(dt))
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let reql_type = String::from("TIME");
        let epoch_time = {
            let t = format!(
                "{}.{}",
                self.0.timestamp(),
                self.0.timestamp_subsec_millis()
            );
            t.parse().unwrap()
        };
        let timezone = String::from("+00:00");
        let time = Time {
            reql_type,
            epoch_time,
            timezone,
        };
        time.serialize(serializer)
    }
}

impl Deref for DateTime {
    type Target = chrono::DateTime<chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
