use crate::DateTime;
use serde::{de, ser};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;
use time::{format_description, OffsetDateTime, UtcOffset};

const NANOS_PER_SEC: i128 = 1_000_000_000;
const NANOS_PER_MSEC: i128 = 1_000_000;

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
struct Time {
    #[serde(rename = "$reql_type$")]
    reql_type: String,
    #[serde(with = "epoch_time")]
    epoch_time: String,
    timezone: String,
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let time = Time::deserialize(deserializer)?;
        let format = match format_description::parse("[offset_hour]:[offset_minute]") {
            Ok(fmt) => fmt,
            Err(error) => {
                return Err(de::Error::custom(error));
            }
        };
        let offset = match UtcOffset::parse(&time.timezone, &format) {
            Ok(offset) => offset,
            Err(error) => {
                return Err(de::Error::custom(error));
            }
        };
        let (secs, msecs) = match time.epoch_time.split_once('.') {
            Some(parts) => parts,
            None => {
                return Err(de::Error::custom("invalid epoch time"));
            }
        };
        let secs = match secs.parse::<i128>() {
            Ok(secs) => match secs.checked_mul(NANOS_PER_SEC) {
                Some(secs) => secs,
                None => {
                    return Err(de::Error::custom("seconds to nanosecond overflow"));
                }
            },
            Err(..) => {
                return Err(de::Error::custom("invalid epoch time seconds"));
            }
        };
        // RethinkDB timestamps have millisecond precision so we need
        // to convert the milliseconds to nanoseconds first
        let msecs = match msecs.parse::<i128>() {
            Ok(int) => {
                let msecs = match msecs.len() {
                    3 => int,
                    2 => int * 10,
                    1 => int * 100,
                    _ => {
                        return Err(de::Error::custom("invalid epoch milliseconds"));
                    }
                };
                match msecs.checked_mul(NANOS_PER_MSEC) {
                    Some(msecs) => msecs,
                    None => {
                        return Err(de::Error::custom("millisecond to nanosecond overflow"));
                    }
                }
            }
            Err(..) => {
                return Err(de::Error::custom("invalid epoch time milliseconds"));
            }
        };
        let timestamp = match secs.checked_add(msecs) {
            Some(timestamp) => timestamp,
            None => {
                return Err(de::Error::custom("timestamp addition overflow"));
            }
        };
        let dt = match OffsetDateTime::from_unix_timestamp_nanos(timestamp) {
            Ok(date_time) => date_time.replace_offset(offset),
            Err(error) => {
                return Err(de::Error::custom(error));
            }
        };
        Ok(DateTime(dt))
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt = &self.0;
        let offset = dt.offset();
        let epoch_time = {
            let seconds = match dt
                .unix_timestamp()
                .checked_add(offset.whole_seconds() as i64)
            {
                Some(secs) => secs,
                None => {
                    return Err(ser::Error::custom("timestamp addition overflow"));
                }
            };
            format!("{}.{:03}", seconds, dt.millisecond())
        };
        let timezone = {
            let (hours, minutes, _) = offset.as_hms();
            format!(
                "{}{:02}:{:02}",
                if offset.is_negative() { '-' } else { '+' },
                hours.abs(),
                minutes.abs(),
            )
        };
        let time = Time {
            reql_type: "TIME".to_owned(),
            epoch_time,
            timezone,
        };
        time.serialize(serializer)
    }
}

impl Deref for DateTime {
    type Target = OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

mod epoch_time {
    use super::*;

    pub fn serialize<S>(epoch_time: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match epoch_time.parse::<f64>() {
            Ok(timestamp) => serializer.serialize_f64(timestamp),
            Err(..) => Err(ser::Error::custom("invalid epoch timestamp")),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = f64::deserialize(deserializer)?;
        Ok(timestamp.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn date_time() {
        let dt = DateTime(datetime!(2042-10-28 17:53:47.060 +1:30));
        let serialized = serde_json::to_string(&dt).unwrap();
        let parsed = serde_json::from_str(&serialized).unwrap();
        assert_eq!(dt, parsed);
    }
}
