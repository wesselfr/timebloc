extern crate colored;
use core::time;

// not needed in Rust 2018
use colored::*;

use chrono::{self, offset, DateTime, Local, Timelike};

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

fn print_timeslot(hour: u8, is_free: bool, subdivision: u32, current: &DateTime<Local>) {
    let division_in_min: u32 = 60 / subdivision;
    for i in 0..subdivision {
        let is_current_time =
            current.hour() == hour.into() && current.minute() / division_in_min == i.into();

        if is_free {
            if is_current_time {
                println!(
                    "({:2}:{:0>2}) --- {}",
                    &hour,
                    division_in_min * i,
                    "║ xxx xxx xxx xxx ║".red()
                );
            } else {
                println!(
                    "({:2}:{:0>2})  -  {}",
                    &hour,
                    division_in_min * i,
                    "║ --- --- --- --- ║".truecolor(100, 100, 100)
                );
            }
        } else {
            if is_current_time {
                println!(
                    "({:2}:{:0>2})  {}  {}",
                    &hour,
                    division_in_min * i,
                    "►".green(),
                    "╠ ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ ╣".red()
                );
            } else {
                println!(
                    "({:2}:{:0>2})  {}  {}",
                    &hour,
                    division_in_min * i,
                    "►".green(),
                    "╠ ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ ╣".green()
                );
            }
        }
    }
}

fn main() {
    println!("{} {} !", "it".bright_green(), "works".blue().bold());
    let visible_hours_min = 8;
    let visible_hours_max = 24;
    let visible_hours_subdivision = 4;

    assert!(visible_hours_max <= 24);
    assert!(visible_hours_min < visible_hours_max);

    //let visible_hours = [8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
    let appointments = [
        TimeBlock::new(10, 12),
        TimeBlock::new(13, 16),
        TimeBlock::new(23, 24),
    ];

    let current_time = chrono::offset::Local::now();
    let current_hour = current_time.hour();
    let current_minute = current_time.minute();

    let block = TimeBlock::new(10, 12);
    if block.within_timeblock(11) {
        let s1 = &block.start_hour;
        let s2 = &block.end_hour;
        println!(
            "{} Current appointment from {} to {}.",
            "".bright_green(),
            &s1,
            &s2
        );
    }

    for i in visible_hours_min..visible_hours_max + 1 {
        let mut is_free = true;
        for timeblock in appointments {
            if timeblock.within_timeblock(i) {
                is_free = false
            }
        }
        print_timeslot(i, is_free, visible_hours_subdivision, &current_time);
    }
}
