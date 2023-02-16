use std::io::stdin;
use std::num::ParseIntError;

use chrono::prelude::*;
use chrono::LocalResult::Single;
use chrono::{DateTime, ParseError, Utc};

use clap::Parser;
use thiserror::Error;

use crate::DateTimeError::MultipleErrors;

/// Tries to parse an input from standard input or from first parameter as
/// ISO 8601 date or as epoch (milli)seconds and prints the parsed date as
/// - ISO 8601 datetime,
/// - epoch seconds and
/// - epoch milliseconds.
/// NOTE:
/// Numeric values will be handled as epoch seconds if the year of the result is less than 3000.
/// Otherwise, they will be handled as epoch milliseconds.
#[derive(Parser)]
#[clap(verbatim_doc_comment)]
struct CliParams {
    /// Input to be parsed. If omitted standard input is used.
    #[clap()]
    date_time: Option<String>,
}

#[derive(Error, Debug)]
enum DateTimeError {
    #[error("{0}")]
    NumberFormatError(#[from] ParseIntError),
    #[error("{0}")]
    DateFormatError(#[from] ParseError),
    #[error("Invalid epoch time: {0}")]
    InvalidEpochTime(i64),
    #[error("Multiple errors: \"{0}\" and \"{1}\"")]
    MultipleErrors(Box<DateTimeError>, Box<DateTimeError>),
}

fn main() {
    for line in input_iterator() {
        match get_datetime(line.as_str()) {
            Ok(datetime) => print_time(datetime),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn input_iterator() -> Box<dyn Iterator<Item = String>> {
    match CliParams::parse().date_time {
        Some(date_time) => Box::new([date_time].into_iter()),
        _ => Box::new(stdin().lines().flatten()),
    }
}

fn print_time(datetime: DateTime<Utc>) {
    let iso_time = datetime.to_rfc3339_opts(SecondsFormat::Millis, true);
    let epoch_seconds = datetime.timestamp();
    let epoch_milliseconds = datetime.timestamp_millis();
    println!("┌────────────────────┬──────────────────────────┐");
    println!("│ ISO 8601 timestamp │ {:24} │", iso_time);
    println!("├────────────────────┬──────────────────────────┤");
    println!("│ Epoch seconds      │ {:24} │", epoch_seconds);
    println!("├────────────────────┬──────────────────────────┤");
    println!("│ Epoch milliseconds │ {:24} │", epoch_milliseconds);
    println!("└────────────────────┴──────────────────────────┘");
}

fn get_datetime(input: &str) -> Result<DateTime<Utc>, DateTimeError> {
    let trimmed = input.trim();
    match (iso_to_datetime(trimmed), epoch_to_datetime(trimmed)) {
        (Ok(datetime), _) => Ok(datetime),
        (_, Ok(datetime)) => Ok(datetime),
        (Err(e1), Err(e2)) => Err(MultipleErrors(Box::new(e1), Box::new(e2))),
    }
}

fn iso_to_datetime(input: &str) -> Result<DateTime<Utc>, DateTimeError> {
    Ok(DateTime::parse_from_rfc3339(input)?.into())
}

fn epoch_to_datetime(input: &str) -> Result<DateTime<Utc>, DateTimeError> {
    let epoch_time = str::parse(input)?;
    match (
        Utc.timestamp_opt(epoch_time, 0),
        Utc.timestamp_millis_opt(epoch_time),
    ) {
        (Single(datetime), _) if datetime.year() < 3000 => Ok(datetime),
        (_, Single(datetime)) => Ok(datetime),
        _ => Err(DateTimeError::InvalidEpochTime(epoch_time)),
    }
}

#[cfg(test)]
mod tests {
    use crate::get_datetime;

    #[test]
    fn iso_8601_inclusive_milliseconds() {
        let input = "2023-02-16T12:34:56.789Z";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896789);
    }

    #[test]
    fn iso_8601_inclusive_milliseconds_starting_with_spaces() {
        let input = " 2023-02-16T12:34:56.789Z";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896789);
    }

    #[test]
    fn iso_8601_inclusive_milliseconds_ending_with_spaces() {
        let input = "2023-02-16T12:34:56.789Z ";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896789);
    }

    #[test]
    fn iso_8601() {
        let input = "2023-02-16T12:34:56Z";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896000);
    }

    #[test]
    fn iso_8601_starting_with_spaces() {
        let input = " 2023-02-16T12:34:56Z";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896000);
    }

    #[test]
    fn iso_8601_ending_with_spaces() {
        let input = "2023-02-16T12:34:56Z ";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896000);
    }

    #[test]
    fn epoch_seconds() {
        let input = "1676550896";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896000);
    }

    #[test]
    fn epoch_seconds_starting_with_spaces() {
        let input = " 1676550896";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896000);
    }

    #[test]
    fn epoch_seconds_ending_with_spaces() {
        let input = "1676550896 ";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896000);
    }

    #[test]
    fn epoch_milliseconds() {
        let input = "1676550896789";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896789);
    }

    #[test]
    fn epoch_milliseconds_starting_with_spaces() {
        let input = " 1676550896789";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896789);
    }

    #[test]
    fn epoch_milliseconds_ending_with_spaces() {
        let input = "1676550896789 ";
        let date_time = get_datetime(input).unwrap();
        assert_eq!(date_time.timestamp_millis(), 1676550896789);
    }
}
