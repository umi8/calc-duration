use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use chrono::{Duration, ParseError};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "Time Calculator", version = "1.0.0")]
struct Opts {
    /// Path to the file containing the time to be calculated
    #[clap(name = "FILE")]
    time_file: Option<PathBuf>,
}

fn main() -> Result<(), ParseError> {
    let opts = Opts::parse();

    if let Some(path) = opts.time_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        let mut sum_duration = Duration::zero();
        for line in reader.lines() {
            let line = line.unwrap();
            let v: Vec<&str> = line.split(".").collect();
            let (min, sec) = (Duration::minutes(v[0].parse().unwrap()), Duration::seconds(v[1].parse().unwrap()));
            sum_duration = sum_duration + min + sec;
        }
        let sum_min = sum_duration.num_minutes();
        let sum_sec = sum_duration.num_seconds() % 60;
        println!("{}m{}s", sum_min, sum_sec);
    }

    Ok(())
}
