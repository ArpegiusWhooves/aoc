use std::collections::BTreeMap;

use miette::{miette, IntoDiagnostic, Result};

use testing::get_data;

type Index = u64;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{space1, u64 as parseIndex},
    multi::separated_list1,
    sequence::Tuple,
};

// #[derive(Debug,Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
// struct Maping {
//     from: Index,
//     to: Index,
//     size: Index
// }

fn parse_maping(input: &str) -> Result<(Index, (Index, Index))> {
    let (_, (to, _, from, _, size)) = (parseIndex, space1, parseIndex, space1, parseIndex)
        .parse(input)
        .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| e.to_owned())
        .into_diagnostic()?;

    Ok((from, (to, size)))
}

fn map_index(val: Index, maping: &BTreeMap<Index, (Index, Index)>) -> Index {
    use std::ops::Bound::*;
    let range = maping.range((Unbounded, Included(val)));
    if let Some((from, (to, size))) = range.last() {
        let offset = val - from;
        if &offset < size {
            return to + offset;
        }
    }
    val
}

fn aoc((body, file): (String, String)) -> Result<Index> {
    // let mut r = 0;

    let mut lines = body.split('\n').enumerate();

    let (_, first_line) = lines
        .next()
        .ok_or_else(|| miette!("No lines in input {}!", file))?;

    let (_, (_, seeds)) = (
        tag("seeds: "),
        separated_list1(take_while1(char::is_whitespace), parseIndex),
    )
        .parse(first_line)
        .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| e.to_owned())
        .into_diagnostic()?;

    let mut translations = Vec::new();
    loop {
        let Some((_no, section)) = lines.next() else {
            break;
        };
        if section.trim().is_empty() {
            continue;
        }
        if !section.ends_with(" map:") {
            return Err(miette!("Expect start of section!"));
        }
        let mut mappings = BTreeMap::new();
        while let Some((_no, line)) = lines.next() {
            if line.trim().is_empty() {
                break;
            }
            let (k, v) = parse_maping(line)?;
            mappings.insert(k, v);
        }
        translations.push((&section[..section.len() - 5], mappings));
    }
    dbg!(&translations);

    let mut min_val = Index::MAX;

    for mut val in seeds {
        for (name, maping) in &translations {
            let new_val = map_index(val, maping);
            println!("{name} {val} => {new_val}");
            val = new_val;
        }
        if val < min_val {
            min_val = val;
        }
    }

    Ok(min_val)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors"),
        35
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
