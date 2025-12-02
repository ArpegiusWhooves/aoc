use miette::{Context, IntoDiagnostic, Result};
use testing::get_data;

type CardIndex = u32;
type CardNumber = u32;

use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, tuple, Tuple},
};

fn parse_game(input: &str) -> Result<(CardIndex, Vec<CardNumber>, Vec<CardNumber>)> {
    let (_input, (_a, _b, id, _c, _d, (l1, l2), _e)) = (
        tag("Card"),
        take_while(char::is_whitespace),
        map_res(digit1, str::parse::<CardIndex>),
        tag(":"),
        take_while(char::is_whitespace),
        separated_pair(
            separated_list1(
                take_while1(char::is_whitespace),
                map_res(digit1, str::parse::<CardIndex>),
            ),
            tuple((
                take_while(char::is_whitespace),
                tag("|"),
                take_while(char::is_whitespace),
            )),
            separated_list1(
                take_while1(char::is_whitespace),
                map_res(digit1, str::parse::<CardIndex>),
            ),
        ),
        take_while(char::is_whitespace),
    )
        .parse(input)
        .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| e.to_owned())
        .into_diagnostic()?;

    Ok((id, l1, l2))
}

fn aoc1((body, file): (String, String)) -> Result<u32> {
    let mut r = 0;

    for (no, line) in body.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        let (_card_id, winning_numebers, your_numbers) =
            parse_game(line).wrap_err_with(|| format!("Line: {} of {}", no, file))?;

        let win = winning_numebers
            .into_iter()
            .filter(|v| your_numbers.contains(v))
            .count();

        if win > 0 {
            dbg!(_card_id);
            r += dbg!(1 << (win - 1));
        }
    }

    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc1((
            "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors"),
        13
    );
}

fn main() -> Result<()> {
    println!("{}", aoc1(get_data(file!())?)?);

    Ok(())
}
