use std::fmt;

use chrono::{DateTime, Local};
use serde::{ser::SerializeStruct, Serialize};

#[derive(Debug, PartialEq, PartialOrd)]
struct Time {
    time: DateTime<Local>,
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
}
