use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{character, IResult};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, games) = parse_input(input).expect("input should be valid");
    let sum: i32 = games
        .iter()
        .filter(|game| game.is_valid_for_bag(12, 13, 14))
        .map(|game| game.id)
        .sum();

    Ok(sum.to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, parse_game)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = character::complete::i32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, cube_sets) = separated_list1(tag("; "), parse_cube_set)(input)?;

    Ok((input, Game { id, cube_sets }))
}

fn parse_cube_set(input: &str) -> IResult<&str, CubeSet> {
    let (input, sets) = separated_list1(
        tag(", "),
        separated_pair(
            character::complete::i32,
            tag(" "),
            alt((tag("red"), tag("green"), tag("blue"))),
        ),
    )(input)?;

    let mut set: CubeSet = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    for s in sets {
        match s {
            (count, "red") => {
                set.red = count;
            }
            (count, "green") => {
                set.green = count;
            }
            (count, "blue") => {
                set.blue = count;
            }
            _ => panic!("unrecognized color"),
        }
    }

    Ok((input, set))
}

#[derive(Debug)]
struct Game {
    id: i32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn is_valid_for_bag(&self, red: i32, green: i32, blue: i32) -> bool {
        !self
            .cube_sets
            .iter()
            .any(|set| set.red > red || set.green > green || set.blue > blue)
    }
}

#[derive(Debug)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
