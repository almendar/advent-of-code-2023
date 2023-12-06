use crate::common::{load_input_as_string, parse_ints_separated_by_space};
use std::ops::Range;

#[derive(Debug)]
struct MapRange {
    dest: i64,
    src: i64,
    len: i64,
}

impl MapRange {
    fn map_to_next(&self, value: i64) -> Option<i64> {
        let src_r = self.src..self.src + self.len + 1;
        let trg_r = self.dest..self.dest + self.len + 1;
        if src_r.contains(&value) {
            let index = value - src_r.start;
            let corresponding_value = trg_r.start + index;
            if trg_r.contains(&corresponding_value) {
                Some(corresponding_value)
            } else {
                panic!("Should not happen");
            }
        } else {
            None
        }
    }
}

fn load(input: &str) -> (Vec<i64>, Vec<Vec<MapRange>>) {
    let parts = load_input_as_string(input)
        .expect(&format!("should be able to load {}", input))
        .split("\n\n")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();

    let seeds = match parts[0].split(": ").last() {
        Some(x) => parse_ints_separated_by_space::<i64>(x).expect("seeds parse error"),
        None => panic!("wrong"),
    };

    let maps: Vec<Vec<MapRange>> = parts
        .iter()
        .skip(1)
        .flat_map(|map| map.split("map:\n").skip(1))
        .map(|all_ranges| {
            all_ranges
                .split('\n')
                .filter(|single_range| !single_range.is_empty())
                .map(|single_range| {
                    let k = parse_ints_separated_by_space::<i64>(single_range)
                        .expect("Parsing single range");
                    if k.len() != 3 {
                        panic!("Wrong parsing of MapRange")
                    }
                    MapRange {
                        dest: k[0],
                        src: k[1],
                        len: k[2],
                    }
                })
                .collect()
        })
        .collect();

    (seeds, maps)
}

pub fn part1(input: &str) -> i64 {
    let (seeds, maps) = load(input);
    let mut min_loc = i64::MAX;
    for seed1 in seeds {
        let mut transformed_to = seed1;
        for ranges in maps.iter() {
            for range in ranges.iter() {
                match range.map_to_next(transformed_to) {
                    Some(x) => {
                        // println!("going from {} to {} with range {:?}", transformed_to, x, range);
                        transformed_to = x;
                        break;
                    }
                    None => {}
                }
            }
        }
        min_loc = std::cmp::min(min_loc, transformed_to);
    }
    min_loc
}

fn get_output_ranges(map: &Vec<MapRange>, input: &Range<i64>) -> Vec<Range<i64>> {
    let mut output_ranges = Vec::new();
    let mut mapped_input_ranges = Vec::new();

    for map_range in map {
        let source_range = map_range.src..map_range.src + map_range.len;
        let dest_range = map_range.dest..map_range.dest + map_range.len;
        let start = std::cmp::max(source_range.start, input.start as i64);
        let end = std::cmp::min(source_range.end, input.end as i64);
        if start <= end {
            mapped_input_ranges.push(start..end);
            let offset = dest_range.start - source_range.start;
            output_ranges.push((start + offset)..(end + offset));
        }
    }

    let mut cuts = Vec::new();
    cuts.push(input.start);
    for range in &mapped_input_ranges {
        cuts.push(range.start);
        cuts.push(range.end);
    }
    cuts.push(input.end);
    cuts.sort_unstable();

    let unmapped_input_ranges = cuts
        .windows(2)
        .filter_map(|window| {
            if window[1] > window[0] {
                Some(window[0]..window[1])
            } else {
                None
            }
        })
        .filter(|range| !mapped_input_ranges.contains(range))
        .collect::<Vec<_>>();

    output_ranges.extend(unmapped_input_ranges);
    output_ranges
}

pub fn part2(input: &str) -> i64 {
    let (seeds_unpacked, maps) = load(input);
    let seeds = seeds_unpacked
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<_>>();

    seeds
        .iter()
        .flat_map(|seed_range| {
            maps.iter().fold(vec![seed_range.clone()], |acc, map| {
                acc.iter()
                    .flat_map(|range| get_output_ranges(map, range))
                    .collect()
            })
        })
        .min_by_key(|range| range.start)
        .unwrap()
        .start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_day5() {
        let part1_sample = part1("input/day5_sample.txt");
        assert_eq!(35, part1_sample);
        let part1_input = part1("input/day5_input.txt");
        assert_eq!(261668924, part1_input);
        println!("day5 part1 sample:{}, input: {}", part1_sample, part1_input);

        let part2_sample = part2("input/day5_sample.txt");
        assert_eq!(46, part2_sample);
        let part2_input = part2("input/day5_input.txt");
        assert_eq!(24261545, part2_input);
        println!("day5 part2 sample {}, input: {}", part2_sample, part2_input);
    }
}
