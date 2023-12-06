use crate::Error;

pub fn parse_num_line_with_prefix<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: &mut T,
) -> anyhow::Result<Vec<u64>> {
    let line = input
        .next()
        .ok_or(Error::InvalidInput("Number line is missing".to_string()))??;
    let (_, num_str) = line.split_once(':').ok_or(Error::InvalidInput(
        "Missing Prefix with ':' prefix in number line".to_string(),
    ))?;

    Ok(num_str
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|it| it.parse())
        .collect::<Result<Vec<u64>, _>>()?)
}
