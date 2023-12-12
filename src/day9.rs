fn parse_series(input: Vec<String>) -> anyhow::Result<Vec<Vec<i32>>> {
    let mut series = Vec::new();
    for line in input {
        series.push(
            line.split_whitespace()
                .map(|it| it.parse::<i32>())
                .collect::<Result<_, _>>()?,
        );
    }
    Ok(series)
}

fn diffs(serie: Vec<i32>) -> Vec<Vec<i32>> {
    let mut diffs: Vec<Vec<i32>> = Vec::new();
    let mut current = serie.clone();

    while current.iter().any(|num| *num != 0) {
        let mut next_diff = vec![];

        for i in 1..current.len() {
            next_diff.push(current[i] - current[i - 1]);
        }
        diffs.push(current);
        current = next_diff;
    }

    diffs.push(current);
    diffs
}

fn predict_next(serie: Vec<i32>) -> i32 {
    diffs(serie).iter().map(|it| it.last().unwrap()).sum()
}

fn sum_next(series: Vec<Vec<i32>>) -> i32 {
    series.into_iter().map(predict_next).sum()
}

fn predict_prev(serie: Vec<i32>) -> i32 {
    let diffs: Vec<Vec<i32>> = diffs(serie).into_iter().rev().collect();

    let mut start = 0;

    for i in 1..diffs.len() {
        start = diffs[i].first().unwrap() - start;
    }

    start
}

fn sum_prev(series: Vec<Vec<i32>>) -> i32 {
    series.into_iter().map(predict_prev).sum()
}

pub fn challenge(part: u32, input: Vec<String>) -> anyhow::Result<i32> {
    match part {
        1 => Ok(sum_next(parse_series(input)?)),
        2 => Ok(sum_prev(parse_series(input)?)),
        _ => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Given
        let input = vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ];

        // When
        let sum = challenge(2, input).unwrap();

        // Then
        assert_eq!(2, sum);
    }
}
