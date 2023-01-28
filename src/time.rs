use std::fmt;

use chrono::{DateTime, Local};
use serde::{ser::SerializeStruct, Serialize};

#[derive(Debug, PartialEq, PartialOrd)]
struct Time {
    time: DateTime<Local>,
}

#[derive(Debug, PartialEq)]
struct Duration(chrono::Duration);

impl Duration {
    fn new(minutes: i64) -> Self {
        Self(chrono::Duration::minutes(minutes))
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time.to_rfc2822())
    }
}

impl Time {
    fn new(time: &str) -> Time {
        Time {
            time: DateTime::parse_from_rfc2822(time).unwrap().into(),
        }
    }
}

impl std::ops::Sub<Self> for Time {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        Duration(self.time - rhs.time)
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
        let lenth = 1;
        let mut serialized = serializer.serialize_struct("CreationTime", lenth)?;
        serialized.serialize_field("Time", &self.to_string())?;
        serialized.end()
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::ApproxEq;

    use super::*;

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
    }

    #[test]
    fn equality() {
        let before = Time::new("Sat, 21 Jan 2023 12:25:20 +0100");
        let after = Time::new("Sat, 21 Jan 2023 13:25:20 +0100");
        let duration_1 = after - before;
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
}
