use regex::Regex;
use std::result::Result;

pub enum LineType {
    NewGuard,
    FallAsleep,
    WakeUp,
}

pub fn get_line_data(line: &str) -> Result<(usize, LineType), String> {
    if let Some(id) = get_guard_id(line) {
        return Ok((id, LineType::NewGuard));
    }

    if let Some(min) = get_fall_asleep_minute(line) {
        return Ok((min, LineType::FallAsleep));
    }

    if let Some(min) = get_wake_up_minute(line) {
        return Ok((min, LineType::WakeUp));
    }

    Err("Unable to parse line".to_owned())
}

const DATE_SECTION: &'static str =
    r"^\[(?P<date>\d{4}-\d{2}-\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\]";

fn get_guard_id(line: &str) -> Option<usize> {
    lazy_static! {
        static ref PARSE_EXPR: Regex =
            Regex::new(&(DATE_SECTION.clone().to_owned() + r" Guard #(?P<id>\d+) begins shift$"))
                .unwrap();
    }

    PARSE_EXPR.captures(line).map(|caps| caps["id"].parse::<usize>().unwrap())
}

fn get_fall_asleep_minute(line: &str) -> Option<usize> {
    lazy_static! {
        static ref PARSE_EXPR: Regex =
            Regex::new(&(DATE_SECTION.clone().to_owned() + r" falls asleep$")).unwrap();
    }

    PARSE_EXPR.captures(line).map(|caps| caps["minute"].parse::<usize>().unwrap())
}

fn get_wake_up_minute(line: &str) -> Option<usize> {
    lazy_static! {
        static ref PARSE_EXPR: Regex =
            Regex::new(&(DATE_SECTION.clone().to_owned() + r" wakes up$")).unwrap();
    }

    PARSE_EXPR.captures(line).map(|caps| caps["minute"].parse::<usize>().unwrap())
}

#[cfg(test)]
mod get_guard_id_tests {
    use super::get_guard_id;

    #[test]
    fn has_guard_id() {
        assert_eq!(
            get_guard_id("[1518-11-01 00:00] Guard #10 begins shift"),
            Some(10)
        );
    }

    #[test]
    fn no_guard_id() {
        assert_eq!(get_guard_id("[1518-11-01 00:05] falls asleep"), None);
        assert_eq!(get_guard_id("[1518-11-01 00:25] wakes up"), None);
    }
}

#[cfg(test)]
mod get_fall_asleep_minute_tests {
    use super::get_fall_asleep_minute;

    #[test]
    fn has_fall_asleep_minute() {
        assert_eq!(
            get_fall_asleep_minute("[1518-11-01 00:05] falls asleep"),
            Some(5)
        );
    }

    #[test]
    fn no_fall_asleep_minute() {
        assert_eq!(
            get_fall_asleep_minute("[1518-11-01 00:00] Guard #10 begins shift"),
            None
        );
        assert_eq!(get_fall_asleep_minute("[1518-11-01 00:25] wakes up"), None);
    }
}

#[cfg(test)]
mod get_wake_up_minute_tests {
    use super::get_wake_up_minute;

    #[test]
    fn has_fall_asleep_minute() {
        assert_eq!(get_wake_up_minute("[1518-11-01 00:25] wakes up"), Some(25));
    }

    #[test]
    fn no_fall_asleep_minute() {
        assert_eq!(
            get_wake_up_minute("[1518-11-01 00:00] Guard #10 begins shift"),
            None
        );
        assert_eq!(get_wake_up_minute("[1518-11-01 00:05] falls asleep"), None);
    }
}