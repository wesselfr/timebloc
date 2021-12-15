extern crate colored;
use colored::*;

use chrono::{self, DateTime, Local, Timelike};

pub struct Config {
    pub visible_hours_start: u8,
    pub visible_hours_end: u8,
    pub visible_hours_subdivision: u8,
}

#[derive(Copy, Clone)]
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
            current.hour() == hour.into() && current.minute() / division_in_min == i.into();

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
