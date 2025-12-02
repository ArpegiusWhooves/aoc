use std::cmp;

use miette::{Context, IntoDiagnostic, Result};
use testing::get_data;

type GameIndex = u32;
type CubeCount = u32;

use nom::sequence::Tuple;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{pair, preceded},
};

#[derive(Debug, Clone, Copy)]
enum CubeType {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Default)]
struct Cube {
    r: CubeCount,
    g: CubeCount,
    b: CubeCount,
}

fn min_cube(r: Cube, l: Cube) -> Cube {
    Cube {
        r: cmp::max(r.r, l.r),
        g: cmp::max(r.g, l.g),
        b: cmp::max(r.b, l.b),
    }
}

fn map_cube_type(input: &str) -> Result<CubeType> {
    Ok(match input {
        "red" => CubeType::Red,
        "green" => CubeType::Green,
        "blue" => CubeType::Blue,
        _ => panic!("Not such type {}", input),
    })
}
fn map_cube(v: Vec<(u32, CubeType)>) -> Result<Cube, ()> {
    let mut c = Cube::default();
    for (s, t) in v {
        match t {
            CubeType::Red => c.r += s,
            CubeType::Green => c.g += s,
            CubeType::Blue => c.b += s,
        }
    }
    Ok(c)
}

fn parse_game(input: &str) -> Result<(GameIndex, Vec<Cube>)> {
    let (_input, (_, id, _, _, games)) = (
        tag("Game "),
        map_res(digit1, str::parse::<GameIndex>),
        take_while(char::is_whitespace),
        tag(":"),
        separated_list1(
            tag(";"),
            map_res(
                separated_list1(
                    tag(","),
                    pair(
                        preceded(
                            take_while(char::is_whitespace),
                            map_res(digit1, str::parse::<GameIndex>),
                        ),
                        preceded(
                            take_while(char::is_whitespace),
                            map_res(alpha1, map_cube_type),
                        ),
                    ),
                ),
                map_cube,
            ),
        ),
    )
        .parse(input)
        .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| e.to_owned())
        .into_diagnostic()?;

    Ok((id, games))
}

fn aoc2((body, file): (String, String)) -> Result<u32> {
    let mut r = 0;

    for (no, line) in body.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        let (_game_id, cases) =
            parse_game(line).wrap_err_with(|| format!("Line: {} of {}", no, file))?;

        let c = cases.into_iter().fold(Cube::default(), min_cube);
        dbg!(&c);
        r += dbg!(c.r * c.g * c.b);
    }

    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc2((
            "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors"),
        2286
    );
}

fn main() -> Result<()> {
    println!("{}", aoc2(get_data(file!())?)?);

    Ok(())
}
