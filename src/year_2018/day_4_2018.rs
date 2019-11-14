use crate::util::read_input_as_string;
use chrono::{NaiveDateTime, Timelike};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Shift {
    guard: u32,
    hours: Vec<Status>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Status {
    WakeUp(NaiveDateTime),
    Sleep(NaiveDateTime),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Guard {
    guard: u32,
    times_sleep_awake_per_minute: Vec<(u32, u32, Status)>,
    total_sleep: u32,
    total_awake: u32,
}

impl Guard {
    fn increment_sleep(&mut self, sleep_hours: u32) {
        self.total_sleep += sleep_hours;
    }

    fn increment_awake(&mut self, awake_hours: u32) {
        self.total_awake += awake_hours;
    }

    fn update_guard_hours(&mut self, shift_hours: Vec<Status>) {
        for index in 0..60 {
            match (
                self.times_sleep_awake_per_minute.clone().get(index),
                shift_hours.get(index),
            ) {
                (Some((_, count, Status::Sleep(_))), Some(Status::Sleep(time))) => {
                    self.times_sleep_awake_per_minute.remove(index);
                    self.times_sleep_awake_per_minute.insert(
                        index,
                        (index as u32, count + 1, Status::Sleep(time.clone())),
                    );
                }
                (Some((_, _, Status::WakeUp(_))), Some(Status::Sleep(time))) => {
                    self.times_sleep_awake_per_minute.remove(index);
                    self.times_sleep_awake_per_minute
                        .insert(index, (index as u32, 1, Status::Sleep(time.clone())));
                }
                _ => {}
            }
        }
    }
}

pub fn day_4_1_2018() -> Option<u32> {
    let day_4_input: Vec<String> = read_input_as_string("day_4_2018.txt");
    let mut parsed: Vec<(NaiveDateTime, Option<u32>, Option<Status>)> =
        day_4_input.into_iter().map(normalize_point).collect();
    parsed.sort_by(|&first, second| first.0.cmp(&second.0));

    let normalized = parse_shift_sorted(parsed);
    let hours_guard = get_hours_by_guard(normalized);
    let max_guard = hours_guard
        .iter()
        .max_by_key(|guard| guard.total_sleep)
        .unwrap();

    let minute = max_guard
        .times_sleep_awake_per_minute
        .iter()
        .max_by_key(|minute| minute.1)
        .unwrap();

    Some(max_guard.guard * minute.0)
}

fn normalize_point(line: String) -> (NaiveDateTime, Option<u32>, Option<Status>) {
    let first_split: Vec<&str> = line.split(" ").clone().collect();

    let date_time_str_first = first_split.clone().get(0).unwrap().to_owned();

    let date_time_str_second = first_split.clone().get(1).unwrap().to_owned();

    let date_time_str = format!(
        "{first}{second}",
        first = date_time_str_first,
        second = date_time_str_second
    );

    let date_time = NaiveDateTime::parse_from_str(&date_time_str, "[%Y-%m-%d %H:%M]").unwrap();
    let _data = line.to_owned();

    let status = match format!(
        "{} {}",
        first_split.clone().get(2).unwrap().to_owned(),
        first_split.clone().get(3).unwrap().to_owned()
    )
    .as_ref()
    {
        "wakes up" => Some(Status::WakeUp(date_time)),
        "falls asleep" => Some(Status::Sleep(date_time)),
        _x => None,
    };

    let id = match status.clone() {
        None => Some(
            u32::from_str(
                first_split
                    .clone()
                    .get(3)
                    .unwrap()
                    .split("#")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap(),
            )
            .unwrap(),
        ),
        _ => None,
    };

    return (date_time, id, status);
}

fn parse_shift_sorted(logs: Vec<(NaiveDateTime, Option<u32>, Option<Status>)>) -> Vec<Shift> {
    let mut shifts: Vec<Shift> = Vec::with_capacity(logs.len());
    for log in logs {
        match log {
            (time, Some(id), _) => {
                let shift = Shift {
                    guard: id,
                    hours: (0..60).map(|_| Status::WakeUp(time)).collect(),
                };
                shifts.push(shift)
            }
            (_time, _, Some(status)) => {
                let mut shift = shifts.pop().unwrap();
                let mut prev_hours = shift.hours;

                match status {
                    Status::WakeUp(wake_time) => {
                        let mins = wake_time.minute() as usize;
                        for index in mins..60 {
                            prev_hours.remove(index);
                            prev_hours.insert(index, Status::WakeUp(wake_time))
                        }
                    }
                    Status::Sleep(sleep_time) => {
                        let mins = sleep_time.minute() as usize;
                        for index in mins..60 {
                            prev_hours.remove(index);
                            prev_hours.insert(index, Status::Sleep(sleep_time))
                        }
                    }
                }
                shift.hours = prev_hours;
                shifts.push(shift);
            }
            _ => (),
        };
    }
    shifts
}

fn get_hours_by_guard(shifts: Vec<Shift>) -> Vec<Guard> {
    let mut result: Vec<Guard> = Vec::with_capacity(shifts.len());

    for shift in shifts {
        let sleep_count = shift
            .hours
            .iter()
            .filter(|&status| match status {
                Status::Sleep(_) => true,
                _ => false,
            })
            .count();
        let awake_count = shift
            .hours
            .iter()
            .filter(|&g| match g {
                Status::WakeUp(_) => true,
                _ => false,
            })
            .count();

        match result
            .as_mut_slice()
            .into_iter()
            .find(|guard| guard.guard == shift.guard)
            .as_mut()
        {
            Some(guard_hours) => {
                guard_hours.increment_sleep(sleep_count as u32);
                guard_hours.increment_awake(awake_count as u32);
                guard_hours.update_guard_hours(shift.hours);
            }
            None => {
                let mut counter = 0;
                result.push(Guard {
                    guard: shift.guard,
                    total_awake: awake_count as u32,
                    total_sleep: sleep_count as u32,
                    times_sleep_awake_per_minute: shift
                        .hours
                        .into_iter()
                        .map(|status| {
                            let result = (counter, 1, status);
                            counter += 1;
                            result
                        })
                        .collect(),
                });
            }
        }
    }
    result
}

pub fn day_4_2_2018() -> Option<u32> {
    let day_4_input: Vec<String> = read_input_as_string("day_4_2018.txt");
    let mut parsed: Vec<(NaiveDateTime, Option<u32>, Option<Status>)> =
        day_4_input.into_iter().map(normalize_point).collect();
    parsed.sort_by(|&first, second| first.0.cmp(&second.0));

    let normalized = parse_shift_sorted(parsed);
    let hours_guard = get_hours_by_guard(normalized);

    let max_guard = hours_guard
        .iter()
        .max_by_key(|guard| {
            guard
                .times_sleep_awake_per_minute
                .iter()
                .max_by_key(|minute| minute.1)
                .unwrap()
                .1
        })
        .unwrap();

    let minute = max_guard
        .times_sleep_awake_per_minute
        .iter()
        .max_by_key(|minute| minute.1)
        .unwrap();

    Some(max_guard.guard * minute.0)
}

#[test]
fn probando_day_4_1_2018() {
    assert_eq!(Some(12169), day_4_1_2018())
}

#[test]
fn probando_day_4_2_2018() {
    assert_eq!(Some(16164), day_4_2_2018())
}
