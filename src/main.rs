use chrono::{self};
use timebloc::*;

fn main() {
    let config = Config {
        visible_hours_start: 8,
        visible_hours_end: 17,
        visible_hours_subdivision: 4,
    };

    save_config(&config);

    assert!(config.visible_hours_end <= 24);
    assert!(config.visible_hours_start < config.visible_hours_end);

    let appointments = [
        TimeBlock::new(10, 12),
        TimeBlock::new(13, 16),
        TimeBlock::new(23, 24),
    ];

    let current_time = chrono::offset::Local::now();

    for i in config.visible_hours_start..config.visible_hours_end {
        let mut is_free = true;
        for timeblock in appointments {
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
