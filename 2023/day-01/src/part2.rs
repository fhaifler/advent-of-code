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
    let numbers: Vec<_> = replace_word_numbers(input)
        .chars()
        .filter(|&c| char::is_numeric(c))
        .collect();
    format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
}

fn replace_word_numbers(input: &str) -> String {
    input
        .replace("zero", "z0o")
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
