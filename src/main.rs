use adc23::Error;

fn read_std_in() -> anyhow::Result<Vec<String>> {
    Ok(std::io::stdin().lines().collect::<Result<_, _>>()?)
}

fn main() -> anyhow::Result<()> {
    let day: u32 = std::env::args().nth(1).ok_or(Error::MissingArg)?.parse()?;
    let part: u32 = std::env::args().nth(2).ok_or(Error::MissingArg)?.parse()?;

    let result = match day {
        1 => Ok(adc23::day1::sum_calibration_values(part, read_std_in()?)),
        2 => adc23::day2::elf_challenge(part, read_std_in()?),
        3 => adc23::day3::challenge(part, read_std_in()?),
        4 => adc23::day4::challenge(part, read_std_in()?),
        5 => adc23::day5::challenge(part, &mut std::io::stdin().lines()),
        6 => adc23::day6::challenge(part, &mut std::io::stdin().lines()).map(|it| it as u32), // Sketchy but works
        7 => adc23::day7::challenge(part, &mut std::io::stdin().lines()),
        day => Err(Error::InvalidInput(format!("Day {} is not implemented", day)).into()),
    }?;

    println!("Result is: {}", result);

    Ok(())
}
