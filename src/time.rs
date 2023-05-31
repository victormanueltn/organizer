use chrono::TimeZone;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd)]
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

impl Time {
    pub(crate) fn new(time: &str) -> Time {
        Time {
            time: DateTime::parse_from_rfc2822(time).unwrap(),
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

impl From<(u32, u32, u32, u32, u32, u32)> for Time {
    fn from(day_month_year_hour_minute_second: (u32, u32, u32, u32, u32, u32)) -> Time {
        let year: i32 = day_month_year_hour_minute_second.2.try_into().unwrap();
        let date = chrono::Utc
            .with_ymd_and_hms(
                year,
                day_month_year_hour_minute_second.1,
                day_month_year_hour_minute_second.0,
                day_month_year_hour_minute_second.3,
                day_month_year_hour_minute_second.4,
                day_month_year_hour_minute_second.5,
            )
            .unwrap();
        Time::new(&date.to_rfc2822())
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
            Ok(date) => Ok(Time::new(&date.to_rfc2822())),
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
        let time = Time::new(time_string);
        assert_eq!(time.to_string(), time_string);
    }

    #[test]
    fn comparison() {
        let before = Time::new("Sat, 21 Jan 2023 12:25:20 +0100");
        let after = Time::new("Sat, 21 Jan 2023 12:25:21 +0100");
        assert!(after > before);

        let before = Time::new("Sat, 21 Jan 2023 12:25:20 +0100");
        let after = Time::now();
        assert!(after > before);
    }

    #[test]
    fn equality() {
        let before = Time::new("Sat, 21 Jan 2023 12:25:20 +0100");
        let after = Time::new("Sat, 21 Jan 2023 13:25:20 +0100");
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
    fn iced_date_to_time() {
        let year = 2023;
        let month = 4;
        let day = 29;
        let date = chrono::Utc
            .with_ymd_and_hms(year, month, day, 14, 09, 0)
            .unwrap();
        let time_1 = Time::new(&date.to_rfc2822());

        let time_2 = Time::from((29, 4, 2023, 14, 09, 0));

        assert!(time_1 == time_2);
    }
}
