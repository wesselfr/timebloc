extern crate colored;
use std::fs::File;
use std::io::{Read, Write};
use std::{env, fs::create_dir};

use chrono::{self, DateTime, Datelike, Local, Timelike};
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub visible_hours_start: u8,
    pub visible_hours_end: u8,
    pub visible_hours_subdivision: u8,
}

pub fn save_config(config: &Config) -> Result<()> {
    // Serialize it to a JSON string.
    let j = serde_json::to_string(config)?;

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

fn load_config_from_string(contents: &String) -> Config {
    let config: Config = serde_json::from_str(&contents).expect("Error while loading config.");
    config
}

pub fn load_config() -> Result<Config> {
    let config: Config;

    let mut directory = env::temp_dir();
    directory.push("timebloc");

    let file_path = directory.join("config.json");

    let file = File::open(file_path);
    if file.is_ok() {
        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents).unwrap();
        config = load_config_from_string(&contents);
    } else {
        // Create and save default config.
        config = Config {
            visible_hours_start: 8,
            visible_hours_end: 17,
            visible_hours_subdivision: 4,
        };
        save_config(&config).expect("Error while saving config.");
    }
    Ok(config)
}

pub fn save_appointments(appointments: &Vec<TimeBlock>) -> Result<()> {
    let j = serde_json::to_string(&appointments)?;

    let mut directory = env::temp_dir();
    directory.push("timebloc");

    // Create directory if it doesn't exsist.
    if !directory.is_dir() {
        let status = create_dir(&directory);
        if status.is_err() {
            panic!("Could not create directory..")
        }
    }

    println!("Saving appointments to: {}", directory.to_str().unwrap());
    let temp_file = directory.join("appointments.json");

    let mut file = File::create(temp_file).unwrap();
    writeln!(&mut file, "{}", j);

    Ok(())
}

fn load_appointments_from_string(data: &String) -> Result<Vec<TimeBlock>> {
    let appointments = serde_json::from_str(data);
    appointments
}

pub fn load_appointments() -> Vec<TimeBlock> {
    let mut appointments: Vec<TimeBlock> = Vec::new();
    let mut directory = env::temp_dir();
    directory.push("timebloc");

    if !directory.is_dir() {
        return appointments;
    }

    let file_path = directory.join("appointments.json");

    let mut file = File::open(file_path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    appointments = load_appointments_from_string(&contents).expect("Unable to load appointments");
    return appointments;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimeBlock {
    pub name: String,
    pub date: u32, // YYYYMMDD
    pub start_hour: u8,
    pub end_hour: u8,
}

impl TimeBlock {
    pub fn new(start: u8, end: u8) -> TimeBlock {
        TimeBlock {
            name: "New timebloc".to_string(),
            date: get_date_formatted(&chrono::offset::Local::now()),
            start_hour: start,
            end_hour: end,
        }
    }
    pub fn within_timeblock(&self, i: u8) -> bool {
        if i >= self.start_hour && i < self.end_hour {
            return true;
        }
        false
    }
    pub fn is_today(&self) -> bool {
        if self.date == get_date_formatted(&chrono::offset::Local::now()) {
            return true;
        }
        false
    }
}

// Returns the date formatted as YYYY MM DD
pub fn get_date_formatted(date: &DateTime<Local>) -> u32 {
    let format: u32 = date.year() as u32 * 10000 + date.month() * 100 + date.day();
    format
}

pub fn print_all_timeslots(
    current: &DateTime<Local>,
    appointments: &Vec<TimeBlock>,
    start: u8,
    end: u8,
    subdivision: u32,
) {
    for i in start..end {
        let mut is_free = true;
        for timeblock in appointments {
            if timeblock.is_today() {
                if timeblock.within_timeblock(i) {
                    is_free = false
                }
            }
        }
        print_timeslot(i, is_free, subdivision, &current);
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
