use core::time;
use std::{io::{stdin, stdout, Write}, num::NonZeroI128};

use chrono::{self};
use timebloc::*;

fn main() {
    let config_load_result = load_config();
    let config: Config;

    // Check if config is valid, if not create a new one.
    match config_load_result {
        Ok(val) => {
            config = val;
        }
        Err(_err) => {
            config = Config {
                visible_hours_start: 8,
                visible_hours_end: 17,
                visible_hours_subdivision: 4,
            };
            save_config(&config).expect("Error while saving config.");
        }
    }

    assert!(config.visible_hours_end <= 24);
    assert!(config.visible_hours_start < config.visible_hours_end);

    let mut appointments = load_appointments();

    // Fill with test data if needed.
    if appointments.len() == 0 {
        appointments.push(TimeBlock::new(10, 12));
        appointments.push(TimeBlock::new(13, 16));
        appointments.push(TimeBlock::new(23, 24));
        save_appointments(&appointments.to_vec()).expect("Error while saving temp appointments");
    }

    let current_time = chrono::offset::Local::now();

    for i in config.visible_hours_start..config.visible_hours_end {
        let mut is_free = true;
        for timeblock in &appointments {
            if timeblock.within_timeblock(i) {
                is_free = false
            }
        }
        print_timeslot(
            i,
            is_free,
            config.visible_hours_subdivision.into(),
            &current_time,
        );
    }

    loop {
        let mut s = String::new();
        println!("Command: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        println!("Pressed: {}", s);

        let mut input = s.split_whitespace();

        let command = input.next();

        match command {
            Some("clear") => {
                print!("\x1B[2J\x1B[1;1H");
            }
            Some("cls") => {
                print!("\x1B[2J\x1B[1;1H");
            }

            Some("show") => {
                print!("\x1B[2J\x1B[1;1H");

                let mut start = config.visible_hours_start;
                let mut end = config.visible_hours_end;

                let mut option = input.next();
                if option != None{
                    start = option.unwrap().parse().unwrap();
                }
                option = input.next();
                if option != None{
                    end = option.unwrap().parse().unwrap();
                }

                println!("Showing time from {} to {}", start, end);

                for i in start..end {
                    let mut is_free = true;
                    for timeblock in &appointments {
                        if timeblock.within_timeblock(i) {
                            is_free = false
                        }
                    }
                    print_timeslot(
                        i,
                        is_free,
                        config.visible_hours_subdivision.into(),
                        &current_time,
                    );
                }
            }

            Some("exit") => {
                break;
            }

            Some(&_) => todo!(),
            None => todo!(),
        }
    }
}
