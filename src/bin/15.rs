/*
(Debug)
🎄 Part 1 🎄
5100463 (elapsed: 142.33µs)
🎄 Part 2 🎄
11557863040754 (elapsed: 3.17s)

(Release)
🎄 Part 1 🎄
5100463 (elapsed: 22.39µs)
🎄 Part 2 🎄
11557863040754 (elapsed: 158.59ms)
*/

use std::ops::RangeInclusive;

use advent_of_code::error::AppResult;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn manhattan(&self, other: &Self) -> i32 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SensorPair {
    sensor_coord: Coord,
    beacon_coord: Coord,
}

fn parse_coordinates(input: &str) -> AppResult<Vec<SensorPair>> {
    let mut pairs = Vec::new();
    for line in input.lines() {
        let (sx, sy, bx, by) = eyes::try_parse!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i32,
            i32,
            i32,
            i32
        )
        .unwrap_or_else(|| panic!("Failed to parse line '{}'", line));

        pairs.push(SensorPair {
            sensor_coord: Coord { row: sy, col: sx },
            beacon_coord: Coord { row: by, col: bx },
        });
    }

    Ok(pairs)
}

fn row_ranges(row: i32, pairs: &[SensorPair]) -> Vec<RangeInclusive<i32>> {
    let mut ranges: Vec<_> = pairs
        .iter()
        .flat_map(|pair| beacon_row_range(&pair.sensor_coord, &pair.beacon_coord, row))
        .collect();
    ranges.sort_unstable_by_key(|range| *range.start());

    let mut merged_ranges = vec![ranges[0].clone()];
    for next in &ranges[1..] {
        let last_idx = merged_ranges.len() - 1;
        let last = &merged_ranges[last_idx];
        // check if the two sorted ranges overlap
        if next.start() <= last.end() || last.end() + 1 == *next.start() {
            // replace last with a single bigger range if possible
            if next.end() > last.end() {
                let old = &merged_ranges[last_idx];
                let new = *old.start()..=*next.end();
                merged_ranges[last_idx] = new;
            }
        } else {
            // add to the ranges for this row
            merged_ranges.push(next.clone());
        }
    }

    merged_ranges
}

fn beacon_row_range(sensor: &Coord, beacon: &Coord, row: i32) -> Option<RangeInclusive<i32>> {
    let radius = sensor.manhattan(beacon);
    let offset = radius - (sensor.row - row).abs();
    if offset < 0 {
        None
    } else {
        Some(sensor.col - offset..=sensor.col + offset)
    }
}

fn get_unavailable_beacon_count(input: &str, row: i32) -> Option<u32> {
    let input_coord: Vec<SensorPair> = parse_coordinates(input).expect("Failed to parse input");

    let covered = row_ranges(row, &input_coord)
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<i32>() as usize;

    let beacons = input_coord
        .into_iter()
        .map(|pair| pair.beacon_coord)
        .filter(|beacon| beacon.row == row)
        .map(|beacon| beacon.col)
        .dedup()
        .count();

    Some((covered - beacons).try_into().unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    get_unavailable_beacon_count(input, 2_000_000)
}

fn get_tuning_frequency(input: &str, col_size: u32) -> Option<i64> {
    let input_coord: Vec<SensorPair> = parse_coordinates(input).expect("Failed to parse input");
    let (row, col_ranges) = (0..=col_size)
        // not needed but faster
        .rev()
        .map(|row| (row, row_ranges(row.try_into().unwrap(), &input_coord)))
        // if there is more than one range covering the row, there is a gap!
        .find(|(_, ranges)| ranges.len() > 1)
        .unwrap();

    let col = col_ranges.first().unwrap().end() + 1;

    Some(i64::from(col) * 4_000_000 + i64::from(row))
}

pub fn part_two(input: &str) -> Option<i64> {
    get_tuning_frequency(input, 4_000_000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(get_unavailable_beacon_count(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(get_tuning_frequency(&input, 20), Some(56000011));
    }
}
