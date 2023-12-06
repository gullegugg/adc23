use adc23::Error;

fn day1(part: u32) -> anyhow::Result<()> {
    let lines = std::io::stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let sum = adc23::day1::sum_calibration_values(part, lines.into_iter());

    println!("Sum is {}", sum);

    Ok(())
}

fn day2(part: u32) -> anyhow::Result<()> {
    let lines = std::io::stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let sum = adc23::day2::elf_challenge(part, lines.into_iter())?;

    println!("Sum is {}", sum);

    Ok(())
}

fn day3(part: u32) -> anyhow::Result<()> {
    let sum = adc23::day3::challenge(part, std::io::stdin().lines())?;

    println!("Sum is {}", sum);

    Ok(())
}

fn day4(part: u32) -> anyhow::Result<()> {
    let sum = adc23::day4::challenge(part, std::io::stdin().lines())?;

    println!("Sum is {}", sum);

    Ok(())
}

fn day5(part: u32) -> anyhow::Result<()> {
    let sum = adc23::day5::challenge(part, &mut std::io::stdin().lines())?;

    println!("Sum is {}", sum);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let day: u32 = std::env::args().nth(1).ok_or(Error::MissingArg)?.parse()?;
    let part: u32 = std::env::args().nth(2).ok_or(Error::MissingArg)?.parse()?;

    match day {
        1 => day1(part)?,
        2 => day2(part)?,
        3 => day3(part)?,
        4 => day4(part)?,
        5 => day5(part)?,
        _ => eprintln!("Day {} not implemented", day),
    };

    Ok(())
}
