fn parse_calibration_value(line: String) -> Option<u32> {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: u32 = 0;
    for character in line.chars() {
        if let Some(digit) = character.to_digit(10) {
            match first_digit {
                None => {
                    first_digit = Some(digit);
                    last_digit = digit;
                }
                Some(_) => last_digit = digit,
            }
        }
    }
    first_digit.map(|it| it * 10 + last_digit)
}

fn sum_values_part_1<T: Iterator<Item = String>>(lines: T) -> u32 {
    return lines.filter_map(parse_calibration_value).sum();
}

struct Digit(&'static str, u32);

fn get_digit(index: usize, line: &String) -> Option<u32> {
    let digits = vec![
        Digit("one", 1),
        Digit("two", 2),
        Digit("three", 3),
        Digit("four", 4),
        Digit("five", 5),
        Digit("six", 6),
        Digit("seven", 7),
        Digit("eight", 8),
        Digit("nine", 9),
    ];
    let current = line.split_at(index).1;
    for Digit(spelled, value) in digits {
        if current.starts_with(spelled) {
            return Some(value);
        } else if current.starts_with(char::from_u32(value + 48).unwrap()) {
            return Some(value);
        }
    }
    None
}

fn parse_calibration_value_advanced(line: String) -> Option<u32> {
    let current_line = line.clone();
    let mut first_digit: Option<u32> = None;
    let mut last_digit: u32 = 0;

    for index in 0..line.len() {
        if let Some(digit) = get_digit(index, &current_line) {
            match first_digit {
                None => {
                    first_digit = Some(digit);
                    last_digit = digit;
                }
                Some(_) => last_digit = digit,
            }
        }
    }

    first_digit.map(|it| it * 10 + last_digit)
}

fn sum_values_part_2<T: Iterator<Item = String>>(lines: T) -> u32 {
    return lines.filter_map(parse_calibration_value_advanced).sum();
}

pub fn sum_calibration_values<T: Iterator<Item = String>>(part: u32, lines: T) -> u32 {
    match part {
        1 => sum_values_part_1(lines),
        2 => sum_values_part_2(lines),
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        // Given
        let lines = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];

        // When
        let sum = sum_calibration_values(1, lines.into_iter());

        // Then
        assert_eq!(sum, 142);
    }

    #[test]
    fn part2_example() {
        // Given
        let lines = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];

        // When
        let sum = sum_calibration_values(2, lines.into_iter());

        // Then
        assert_eq!(sum, 29 + 83 + 13 + 24 + 42 + 14 + 76);
    }

    #[test]
    fn part2_advanced() {
        // Given
        let input = "ckmb52fldxkseven3fkjgcbzmnr7
        gckhqpb6twoqnjxqplthree2fourkspnsnzxlz1
        2onetwocrgbqm7
        frkh2nineqmqxrvdsevenfive
        four9two
        six7sixqrdfive3twonehsk
        xkvsone2
        one65
        rggxsff1seven
        djbcgrrtqdshpqqzj43rgcr
        br89fivetwoqggnxjfourtl3
        zoneight47five5sixjxd74
        4five1
        5seveneighteightzzbnzsvdjnkvndsxlttfour
        htdcmsl12ninethreepkqtdlvtl
        twocghtvtdlfchfqnjhrfour19
        rptwofiveonecvlldmppxtrvj3
        6gqsvsqpzxj
        5twomgkzsvg
        4ninedflntfsn1
        4threethree
        43two6eight9
        4gqnkntjthree9one45
        24";

        // When
        let sum = sum_calibration_values(2, input.split_whitespace().map(|it| it.to_string()));

        // Then
        assert_eq!(
            sum,
            57 + 61
                + 27
                + 25
                + 42
                + 61
                + 12
                + 15
                + 17
                + 43
                + 83
                + 14
                + 41
                + 54
                + 13
                + 29
                + 23
                + 66
                + 52
                + 41
                + 43
                + 49
                + 45
                + 24
        )
    }
}
