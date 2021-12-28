extern crate colored;
use std::fs::File;
use std::io::{self, Write};
use std::{env, fs::create_dir};

use chrono::{self, DateTime, Local, Timelike};
use colored::*;
use serde::{ser, Deserialize, Serialize};
use serde_json::{Error, Result};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub visible_hours_start: u8,
    pub visible_hours_end: u8,
    pub visible_hours_subdivision: u8,
}

pub fn save_config(config: &Config) -> Result<()> {
    // Serialize it to a JSON string.
    let j = serde_json::to_string(config)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    let mut directory = env::temp_dir();
    directory.push("timebloc");

    // Create directory if it doesn't exsist.
    if !directory.is_dir() {
        let status = create_dir(&directory);
        if status.is_err() {
            panic!("Could not create directory..")
        }
    }

    println!("Saving config to: {}", directory.to_str().unwrap());
    let temp_file = directory.join("config.json");

    let mut file = File::create(temp_file).unwrap();
    writeln!(&mut file, "{}", j);

    Ok(())
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TimeBlock {
    pub start_hour: u8,
    pub end_hour: u8,
}

impl TimeBlock {
    pub fn new(start: u8, end: u8) -> TimeBlock {
        TimeBlock {
            start_hour: start,
            end_hour: end,
        }
    }
    pub fn within_timeblock(self, i: u8) -> bool {
        if i >= self.start_hour && i < self.end_hour {
            return true;
        }
        false
    }
}

pub fn print_timeslot(hour: u8, is_free: bool, subdivision: u32, current: &DateTime<Local>) {
    let division_in_min: u32 = 60 / subdivision;
    for i in 0..subdivision {
        let is_current_time =
            current.hour() as u8 == hour && current.minute() / division_in_min == i;

        if is_free {
            if is_current_time {
                println!(
                    "({:0>2}:{:0>2}) --- {}",
                    &hour,
                    division_in_min * i,
                    "║ xxx xxx xxx xxx ║".red()
                );
            } else {
                println!(
                    "({:0>2}:{:0>2})  -  {}",
                    &hour,
                    division_in_min * i,
                    "║ --- --- --- --- ║".truecolor(100, 100, 100)
                );
            }
        } else {
            if is_current_time {
                println!(
                    "({:0>2}:{:0>2})  {}  {}",
                    &hour,
                    division_in_min * i,
                    "►".green(),
                    "╠ ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ ╣".red()
                );
            } else {
                println!(
                    "({:0>2}:{:0>2})  {}  {}",
                    &hour,
                    division_in_min * i,
                    "►".green(),
                    "╠ ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ ╣".green()
                );
            }
        }
    }
}
