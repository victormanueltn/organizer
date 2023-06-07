use chrono::{DateTime, FixedOffset};
use chrono::{LocalResult, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub(crate) struct Time {
    time: DateTime<FixedOffset>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) struct Duration(chrono::Duration);

impl Duration {
    pub(crate) fn new(minutes: i64) -> Self {
        Self(chrono::Duration::minutes(minutes))
    }
}

#[derive(Debug)]
pub struct TimeError {}

impl Time {
    pub(crate) fn new(
        day: u32,
        month: u32,
        year: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Result<Time, TimeError> {
        let year: i32 = year.try_into().unwrap();
        let time = chrono::Utc.with_ymd_and_hms(year, month, day, hour, minute, second);
        if let LocalResult::Single(time) = time {
            Ok(Time { time: time.into() })
        } else {
            Err(TimeError {})
        }
    }

    pub(crate) fn now() -> Time {
        Time {
            time: {
                let now = chrono::Local::now();
                DateTime::parse_from_rfc2822(&now.to_rfc2822()).unwrap()
            },
        }
    }
}

impl From<&str> for Time {
    fn from(time: &str) -> Time {
        Time {
            time: DateTime::parse_from_rfc2822(time).unwrap(),
        }
    }
}

impl<'a, 'b> std::ops::Sub<&'b Time> for &'a Time {
    type Output = Duration;
    fn sub(self, other: &'b Time) -> Self::Output {
        Duration(self.time - other.time)
    }
}

impl std::ops::Div<Self> for Duration {
    type Output = f32;
    fn div(self, rhs: Self) -> Self::Output {
        match rhs.0.num_seconds() {
            0 => panic!(),
            _ => self.0.num_seconds() as f32 / rhs.0.num_seconds() as f32,
        }
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.time.to_rfc2822())
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let description = String::deserialize(deserializer)?;
        match chrono::DateTime::parse_from_rfc2822(&description) {
            Ok(_) => Ok(Time::from(description.as_str())),
            Err(message) => Err(serde::de::Error::custom(message)),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fmt;
    impl fmt::Display for Time {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.time.to_rfc2822())
        }
    }

    #[test]
    fn new_from_string() {
        let time_string = "Sat, 21 Jan 2023 12:25:20 +0100";
        let time = Time::from(time_string);
        assert_eq!(time.to_string(), time_string);
    }

    #[test]
    fn comparison() {
        let before = Time::from("Sat, 21 Jan 2023 12:25:20 +0100");
        let after = Time::from("Sat, 21 Jan 2023 12:25:21 +0100");
        assert!(after > before);

        let before = Time::from("Sat, 21 Jan 2023 12:25:20 +0100");
        let after = Time::now();
        assert!(after > before);
    }

    #[test]
    fn equality() {
        let before = Time::from("Sat, 21 Jan 2023 12:25:20 +0100");
        let after = Time::from("Sat, 21 Jan 2023 13:25:20 +0100");
        let duration_1 = &after - &before;
        let duration_2 = Duration::new(60);
        assert_eq!(duration_1, duration_2);
    }

    #[test]
    fn duration_ratio() {
        let duration_1 = Duration::new(30);
        let duration_2 = Duration::new(15);
        let ratio = duration_1 / duration_2;
        assert!(float_cmp::approx_eq!(f32, ratio, 2.0));

        let duration_1 = Duration::new(30);
        let duration_2 = Duration::new(14);
        let ratio = duration_1 / duration_2;
        assert!(float_cmp::approx_eq!(f32, ratio, 2.142857));

        let duration_1 = Duration::new(0);
        let duration_2 = Duration::new(14);
        let ratio = duration_1 / duration_2;
        assert!(float_cmp::approx_eq!(f32, ratio, 0.));
    }

    #[test]
    #[should_panic]
    fn divide_by_zero_panics() {
        let duration_1 = Duration::new(1);
        let duration_2 = Duration::new(0);
        let _ = duration_1 / duration_2;
    }

    #[test]
    fn new() {
        let year = 2023;
        let month = 4;
        let day = 29;
        let date = chrono::Utc
            .with_ymd_and_hms(year, month, day, 14, 09, 0)
            .unwrap();
        let time_1 = Time::from(date.to_rfc2822().as_str());

        let time_2 = Time::new(29, 4, 2023, 14, 09, 0);
        assert!(time_1 == time_2.unwrap());

        let time_2 = Time::new(29, 4, 2023, 14, 09, 1);
        assert!(time_1 < time_2.unwrap());

        let time_2 = Time::new(29, 4, 2023, 14, 08, 59);
        assert!(time_1 > time_2.unwrap());
    }
}
