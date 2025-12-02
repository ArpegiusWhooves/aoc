use itertools::Itertools;
use miette::{IntoDiagnostic, Result};

use testing::get_data;

use std::{
    collections::BTreeMap,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    J,
    N(u8),
    T,
    Q,
    K,
    A,
}
impl From<u8> for Card {
    fn from(value: u8) -> Self {
        use Card::*;
        match value {
            b'0'..=b'9' => N(value - b'0'),
            b'T' => T,
            b'J' => J,
            b'Q' => Q,
            b'K' => K,
            b'A' => A,
            _ => panic!("Unown card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    rank: Rank,
    cards: Vec<Card>,
}
impl Hand {
    fn new(cards: Vec<Card>) -> Hand {
        let v =cards.iter().counts().values().copied().sorted().rev().collect_vec();
        let rank = match v[0] {
            5 => Rank::Five,
            4 => Rank::Four,
            3 => {
                if v[1] == 2 {
                    Rank::Full
                } else {
                    Rank::Three
                }
            }
            2 => {
                if v[1] == 2 {
                    Rank::Two
                } else {
                    Rank::One
                }
            }
            _ => Rank::High,
        };
        Hand { rank, cards }
    }
}

fn aoc1((body, _file): (String, String)) -> Result<u32> {
    let mut r = 0;

    let mut tr = BTreeMap::new();
    for line in body.split('\n') {
        if line.is_empty() {
            continue;
        }
        let cards: Vec<Card> = line[..5].bytes().map(From::from).collect();
        let value = u32::from_str(&line[6..]).into_diagnostic()?;

        tr.insert(dbg!(Hand::new(cards)), value);
    }

    let mut i = 1;
    for (k, v) in tr {
        dbg!(k);
        r += dbg!(i) * dbg!(v);
        i += 1;
    }

    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc1((
            "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors"),
        6440
    );
}

fn main() -> Result<()> {
    println!("{}", aoc1(get_data(file!())?)?);

    Ok(())
}
