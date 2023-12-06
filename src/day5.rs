use std::str::FromStr;

use crate::Error;

struct Range {
    source_start: u32,
    destination_start: u32,
    len: u32,
}

impl Range {
    fn convert(&self, input: u32) -> Option<u32> {
        if input < self.source_start {
            return None;
        }

        let start_diff = input - self.source_start;

        if start_diff >= self.len {
            return None;
        }

        Some(self.destination_start + start_diff)
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u32> = s
            .split(' ')
            .map(|num_str| num_str.parse::<u32>())
            .collect::<Result<_, _>>()
            .map_err(|err| Error::InvalidInput(format!("Unable to parse range: {}", err)))?;

        if nums.len() < 3 {
            return Err(Error::InvalidInput(
                "Range contains to few numbers to be able to parse".to_string(),
            ));
        }

        Ok(Range {
            destination_start: nums[0],
            source_start: nums[1],
            len: nums[2],
        })
    }
}

struct CategoryMap {
    ranges: Vec<Range>,
}

impl CategoryMap {
    fn convert(&self, source: u32) -> u32 {
        for range in self.ranges.iter() {
            if let Some(converted) = range.convert(source) {
                return converted;
            }
        }
        source
    }
}

struct Almenac {
    seeds: Vec<u32>,
    category_maps: Vec<CategoryMap>,
}

impl Almenac {
    fn seed_to_location(&self, seed: u32) -> u32 {
        let mut current = seed;
        for map in self.category_maps.iter() {
            current = map.convert(current);
        }
        return current;
    }
}

fn parse_almenac<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: &mut T,
) -> anyhow::Result<Almenac> {
    let seeds: Vec<u32> = input
        .next()
        .ok_or(Error::InvalidInput("Seeds line missing".into()))??
        .trim()
        .split(' ')
        .skip(1)
        .map(|str_num| str_num.parse::<u32>())
        .collect::<Result<_, _>>()?;

    let mut category_maps: Vec<CategoryMap> = Vec::new();

    while let Some(line_res) = input.next() {
        let line = line_res?;
        if line.is_empty() {
            continue;
        }

        if line.contains(':') {
            let mut ranges: Vec<Range> = vec![];
            while let Some(num_line_res) = input.next() {
                let num_line = num_line_res?;
                if num_line.is_empty() {
                    break;
                }
                ranges.push(num_line.parse()?);
            }
            category_maps.push(CategoryMap { ranges })
        }
    }

    Ok(Almenac {
        seeds,
        category_maps,
    })
}

fn lowest_location(almenac: Almenac) -> anyhow::Result<u32> {
    almenac
        .seeds
        .iter()
        .map(|seed| almenac.seed_to_location(*seed))
        .min()
        .ok_or(Error::InvalidInput("No seeds in input".to_string()).into())
}

pub fn challenge<T: Iterator<Item = Result<String, std::io::Error>>>(
    part: u32,
    input: &mut T,
) -> anyhow::Result<u32> {
    let almenac = parse_almenac(input)?;
    match part {
        1 => lowest_location(almenac),
        _ => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_with_range() {
        // Given
        let range = Range {
            source_start: 98,
            destination_start: 50,
            len: 2,
        };

        // When - Then

        assert_eq!(None, range.convert(97));
        assert_eq!(Some(50), range.convert(98));
        assert_eq!(Some(51), range.convert(99));
        assert_eq!(None, range.convert(100));
    }

    #[test]
    fn test_convert_with_map() {
        // Given
        let map = CategoryMap {
            ranges: vec![
                Range {
                    source_start: 98,
                    destination_start: 50,
                    len: 2,
                },
                Range {
                    source_start: 50,
                    destination_start: 52,
                    len: 48,
                },
            ],
        };

        // When - Then
        assert_eq!(49, map.convert(49));
        assert_eq!(52, map.convert(50));
        assert_eq!(99, map.convert(97));
        assert_eq!(50, map.convert(98));
        assert_eq!(51, map.convert(99));
        assert_eq!(100, map.convert(100));
    }

    #[test]
    fn part1_example() {
        // Given
        let binding = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];

        // When
        let lowest = challenge(
            1,
            &mut binding
                .iter()
                .map(|it| Ok::<_, std::io::Error>(it.to_string())),
        )
        .unwrap();

        // Then
        assert_eq!(35, lowest);
    }
}
