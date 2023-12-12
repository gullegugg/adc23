use adc23::Error;

fn read_std_in() -> anyhow::Result<Vec<String>> {
    Ok(std::io::stdin().lines().collect::<Result<_, _>>()?)
}

fn main() -> anyhow::Result<()> {
    let day: u32 = std::env::args().nth(1).ok_or(Error::MissingArg)?.parse()?;
    let part: u32 = std::env::args().nth(2).ok_or(Error::MissingArg)?.parse()?;

    let result = match day {
        1 => Ok(adc23::day1::sum_calibration_values(part, read_std_in()?) as u128),
        2 => adc23::day2::elf_challenge(part, read_std_in()?).map(|it| it as u128),
        3 => adc23::day3::challenge(part, read_std_in()?).map(|it| it as u128),
        4 => adc23::day4::challenge(part, read_std_in()?).map(|it| it as u128),
        5 => adc23::day5::challenge(part, &mut std::io::stdin().lines()).map(|it| it as u128),
        6 => adc23::day6::challenge(part, &mut std::io::stdin().lines()).map(|it| it as u128), // Sketchy but works
        7 => adc23::day7::challenge(part, &mut std::io::stdin().lines()).map(|it| it as u128),
        8 => adc23::day8::challenge(part, read_std_in()?),
        day => Err(Error::InvalidInput(format!("Day {} is not implemented", day)).into()),
    }?;

    println!("Result is: {}", result);

    Ok(())
}
