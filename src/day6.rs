struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn count_winning_times(&self) -> u64 {
        let mut winning_times = 0;

        for hold in 1..self.time {
            let remaining_time = self.time - hold;
            let speed = hold;

            let distance = speed * remaining_time;

            if distance >= self.distance {
                winning_times += 1;
            }
        }

        winning_times
    }
}

pub fn run_day6() {
    println!("Start day 6!");

    let races = vec!(
        Race { time: 48, distance: 390 },
        Race { time: 98, distance: 1103 },
        Race { time: 90, distance: 1112 },
        Race { time: 83, distance: 1360 },
    );

    let winning_counts = races.iter().map(|r| r.count_winning_times());
    let winning_multiple = winning_counts.fold(1, |a, b| a * b);

    println!("Part 1 result: {}", winning_multiple);

    let big_race = Race { time: 48989083, distance: 390110311121360 };

    println!("Part 2 result: {}", big_race.count_winning_times());
}