use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let sum = _input
        .lines()
        .map(find_calibration_number)
        .map(|num| num.parse::<i32>().unwrap())
        .sum::<i32>();

    Ok(sum.to_string())
}

fn find_calibration_number(input: &str) -> String {
    let numbers: Vec<_> = input.chars().filter(|&c| char::is_numeric(c)).collect();
    format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
