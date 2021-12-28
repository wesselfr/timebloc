use chrono::{self};
use timebloc::*;

fn main() {

    let config_load_result = load_config();
    let config: Config;

    // Check if config is valid, if not create a new one.
    match config_load_result {
        Ok(val) => {
            config = val;
        },
        Err(_err) => {
            config = Config {
                visible_hours_start: 8,
                visible_hours_end: 17,
                visible_hours_subdivision: 4,
            };
            save_config(&config).expect("Error while saving config.");
        }
    }

    // let config = Config {
    //     visible_hours_start: 8,
    //     visible_hours_end: 17,
    //     visible_hours_subdivision: 4,
    // };

    // save_config(&config);

    assert!(config.visible_hours_end <= 24);
    assert!(config.visible_hours_start < config.visible_hours_end);

    let mut appointments = load_appointments();

    // Fill with test data if needed.
    if appointments.len() == 0{
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
}
