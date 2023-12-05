use std::collections::{HashMap, HashSet};

use crate::math::Point;

#[derive(Debug, PartialEq)]
struct EngineNumber {
    start: Point,
    len: u32,
    value: u32,
}

#[derive(Debug, PartialEq)]
enum SchematicPart {
    Number(EngineNumber),
    Symbol(Point, char),
}

fn to_digit(byte: u8) -> Option<u32> {
    if 48 <= byte && byte <= 57 {
        Some((byte - 48).into())
    } else {
        None
    }
}

fn parse_schematic_part(
    line: &[u8],
    start_index: usize,
    y: u32,
) -> anyhow::Result<(Option<SchematicPart>, usize)> {
    for i in start_index..line.len() {
        if line[i] == '.' as u8 {
            continue;
        }

        if let Some(digit) = to_digit(line[i]) {
            let mut digits = vec![digit];
            for digit_index in i + 1..line.len() {
                match to_digit(line[digit_index]) {
                    Some(next_digit) => {
                        digits.push(next_digit);
                    }
                    None => break,
                }
            }
            let mut value = 0;

            for (digit_index, digit) in digits.iter().rev().enumerate() {
                value += digit * 10_u32.pow(digit_index.try_into()?);
            }

            return Ok((
                Some(SchematicPart::Number(EngineNumber {
                    start: Point {
                        x: i.try_into()?,
                        y: y.try_into()?,
                    },
                    len: digits.len().try_into()?,
                    value,
                })),
                i + digits.len(),
            ));
        } else {
            return Ok((
                Some(SchematicPart::Symbol(
                    Point::new(i.try_into()?, y.try_into()?),
                    line[i] as char,
                )),
                i + 1,
            ));
        }
    }

    Ok((None, line.len()))
}

fn parse_schematic<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: T,
) -> anyhow::Result<(Vec<EngineNumber>, HashMap<Point, char>)> {
    let mut numbers: Vec<EngineNumber> = Vec::new();
    let mut symbols: HashMap<Point, char> = HashMap::new();
    let mut y = 0;
    for line in input {
        let mut index = 0;
        let good_line = line?;
        let byte_line = good_line.as_bytes();

        while index < byte_line.len() {
            let (part, next_index) = parse_schematic_part(byte_line, index, y)?;

            index = next_index;

            match part {
                Some(part) => match part {
                    SchematicPart::Number(num) => numbers.push(num),
                    SchematicPart::Symbol(pos, symbol) => {
                        symbols.insert(pos, symbol);
                    }
                },
                None => break,
            };
        }
        y += 1;
    }

    Ok((numbers, symbols))
}

fn is_part_number(number: &EngineNumber, symbols: &HashMap<Point, char>) -> bool {
    for i in -1..number.len as i32 + 1 {
        if symbols.contains_key(&(number.start + Point::new(i, -1)))
            || symbols.contains_key(&(number.start + Point::new(i, 0)))
            || symbols.contains_key(&(number.start + Point::new(i, 1)))
        {
            return true;
        }
    }
    false
}

fn sum_part_numbers<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: T,
) -> anyhow::Result<u32> {
    let (numbers, symbols) = parse_schematic(input)?;

    Ok(numbers
        .iter()
        .filter(|num| is_part_number(num, &symbols))
        .map(|num| num.value)
        .sum())
}

fn gear_power(position: &Point, symbol: &char, numbers: &Vec<EngineNumber>) -> Option<u32> {
    None
}

fn sum_gear_powers<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: T,
) -> anyhow::Result<u32> {
    let (numbers, symbols) = parse_schematic(input)?;

    Ok(symbols
        .iter()
        .filter_map(|(pos, symbol)| gear_power(pos, symbol, &numbers))
        .sum())
}

pub fn challenge<T: Iterator<Item = Result<String, std::io::Error>>>(
    part: u32,
    input: T,
) -> anyhow::Result<u32> {
    match part {
        1 => sum_part_numbers(input),
        _ => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_parse_number() {
        // Given
        let line = "...123".as_bytes();
        let start_index = 0;
        let y = 0;

        // When
        let (part, next_index) = parse_schematic_part(line, start_index, y).unwrap();

        // Then
        assert_eq!(next_index, 6);
        assert_eq!(
            SchematicPart::Number(EngineNumber {
                start: Point::new(3, 0),
                len: 3,
                value: 123
            }),
            part.unwrap()
        );
    }

    #[test]
    fn verify_parse_symbol() {
        // Given
        let line = ".*".as_bytes();
        let start_index = 0;
        let y = 0;

        // When
        let (part, next_index) = parse_schematic_part(line, start_index, y).unwrap();

        // Then
        assert_eq!(next_index, 2);
        assert_eq!(SchematicPart::Symbol(Point::new(1, 0), '*'), part.unwrap());
    }

    #[test]
    fn verify_parse_schematic() {
        // Given
        let binding = vec!["467..114..", "...*......", "..35..633.", "......#..."];

        let input = binding.iter().map(|line| {
            println!("{}", line);
            Ok::<_, std::io::Error>(line.to_string())
        });

        // When
        let (numbers, symbols) = parse_schematic(input).unwrap();

        // Then
        assert_eq!(
            vec![
                EngineNumber {
                    start: Point::new(0, 0),
                    len: 3,
                    value: 467
                },
                EngineNumber {
                    start: Point::new(5, 0),
                    len: 3,
                    value: 114
                },
                EngineNumber {
                    start: Point::new(2, 2),
                    len: 2,
                    value: 35
                },
                EngineNumber {
                    start: Point::new(6, 2),
                    len: 3,
                    value: 633
                }
            ],
            numbers
        );

        assert_eq!(
            HashMap::from([(Point::new(3, 1), '*'), (Point::new(6, 3), '#')]),
            symbols
        );
    }

    #[test]
    fn verify_sum_part_numbers() {
        // Given
        let binding = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let input = binding.iter().map(|line| {
            println!("{}", line);
            Ok::<_, std::io::Error>(line.to_string())
        });

        // When
        let sum = sum_part_numbers(input).unwrap();

        // Then
        assert_eq!(4361, sum);
    }
}
