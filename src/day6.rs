use crate::{parse::parse_num_line_with_prefix, Error};

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn num_ways_to_beat(&self) -> u64 {
        let mut sum = 0;
        for hold_time in 1..self.time {
            let distance = hold_time * (self.time - hold_time);
            if distance > self.distance {
                sum += 1;
            }
        }
        sum
    }
}

fn parse_races<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: &mut T,
) -> anyhow::Result<Vec<Race>> {
    let times = parse_num_line_with_prefix(input)?;
    let distances = parse_num_line_with_prefix(input)?;

    Ok(times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect())
}

fn num_beat_races<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: &mut T,
) -> anyhow::Result<u64> {
    let races = parse_races(input)?;

    Ok(races.iter().map(|race| race.num_ways_to_beat()).product())
}

fn parse_line<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: &mut T,
) -> anyhow::Result<u64> {
    let line = input
        .next()
        .ok_or(Error::InvalidInput("Number line is missing".to_string()))??;
    let (_, num_str) = line.split_once(':').ok_or(Error::InvalidInput(
        "Missing Prefix with ':' prefix in number line".to_string(),
    ))?;

    Ok(num_str.replace(' ', "").parse()?)
}

fn parse_race<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: &mut T,
) -> anyhow::Result<Race> {
    let time = parse_line(input)?;
    let distance = parse_line(input)?;

    Ok(Race { time, distance })
}

fn num_beat_race<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: &mut T,
) -> anyhow::Result<u64> {
    let race = parse_race(input)?;

    Ok(race.num_ways_to_beat())
}

pub fn challenge<T: Iterator<Item = Result<String, std::io::Error>>>(
    part: u32,
    input: &mut T,
) -> anyhow::Result<u64> {
    match part {
        1 => num_beat_races(input),
        2 => num_beat_race(input),
        _ => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        // Given
        let binding = vec!["Time:      7  15   30", "Distance:  9  40  200"];

        // When
        let sum = challenge(
            1,
            &mut binding
                .iter()
                .map(|it| Ok::<_, std::io::Error>(it.to_string())),
        )
        .unwrap();

        assert_eq!(288, sum);
    }

    #[test]
    fn part1_real() {
        // Given
        let binding = vec![
            "Time:        34     90     89     86",
            "Distance:   204   1713   1210   1780",
        ];

        // When
        let sum = challenge(
            1,
            &mut binding
                .iter()
                .map(|it| Ok::<_, std::io::Error>(it.to_string())),
        )
        .unwrap();

        assert_eq!(633080, sum);
    }

    #[test]
    fn part2_example() {
        // Given
        let binding = vec!["Time:      7  15   30", "Distance:  9  40  200"];

        // When
        let sum = challenge(
            2,
            &mut binding
                .iter()
                .map(|it| Ok::<_, std::io::Error>(it.to_string())),
        )
        .unwrap();

        assert_eq!(71503, sum);
    }

    #[test]
    fn part2_real() {
        // Given
        let binding = vec![
            "Time:        34     90     89     86",
            "Distance:   204   1713   1210   1780",
        ];

        // When
        let sum = challenge(
            2,
            &mut binding
                .iter()
                .map(|it| Ok::<_, std::io::Error>(it.to_string())),
        )
        .unwrap();

        assert_eq!(20048741, sum);
    }
}
