use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

enum Event {
    Guard(u32),
    Sleep(u32),
    Awake(u32),
}

struct Guard {
    id: u32,
    total_sleeping_minutes: u32,
    sleeping_minutes: HashMap<u32, u32>,
}

impl Guard {
    pub fn new(id: u32) -> Self {
        let total_sleeping_minutes = 0;
        let sleeping_minutes = HashMap::new();

        Guard { id, total_sleeping_minutes, sleeping_minutes }
    }

    fn add_nap(&mut self, sleep: u32, awake: u32) -> &mut Self {
        let duration = awake - sleep - 1;
        self.total_sleeping_minutes += duration;

        for i in sleep..(awake - 1) {
            *self.sleeping_minutes.entry(i).or_insert(0) += 1;
        }

        self
    }

    fn most_slept_minute(&self) -> u32 {
        let mut slept_vec: Vec<_> = self.sleeping_minutes.iter().collect();
        slept_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());

        slept_vec[0].0.clone()
    }
}

fn parse_line(line: String) -> Event {
    let time = line[15..=16].parse::<u32>().unwrap();
    let event = &line[19..];

    match event.as_ref() {
        "falls asleep" => Event::Sleep(time),
        "wakes up"     => Event::Awake(time),
        _              => Event::Guard(event[7..=10].trim().parse::<u32>().unwrap()),
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let buffer = BufReader::new(file);

    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut current_guard: u32 = 0;
    let mut current_sleep: u32 = 0;

    for line in buffer.lines() {
        // [1518-02-24 23:58] Guard #853 begins shift
        // [1518-02-25 00:20] falls asleep
        // [1518-02-25 00:43] wakes up
        let event = parse_line(line.unwrap());

        match event {
            Event::Guard(id) => current_guard = id,
            Event::Sleep(time) => current_sleep = time,
            Event::Awake(time) => {
                guards.entry(current_guard).or_insert(Guard::new(current_guard)).add_nap(current_sleep, time);
            }
        }
    }
    
    println!("Sleepy guards count: {}", guards.len());

    let mut guards_vec: Vec<_> = guards.values().collect();
    guards_vec.sort_by(|a, b| a.total_sleeping_minutes.cmp(&b.total_sleeping_minutes).reverse());

    let guard = guards_vec[0];
    let result = guard.id * guard.most_slept_minute();
    println!("Mr Sleepyhead is guard id {} and has slept {} minutes his most slept minute is {}", guard.id, guard.total_sleeping_minutes, guard.most_slept_minute());
    println!("Result is {}", result);

    Ok(())
}
