use std::collections::BTreeMap;

use miette::{miette, IntoDiagnostic, Result};

use testing::get_data;

type Index = u64;

use nom::{
    bytes::complete::tag,
    character::complete::{space1, u64 as parseIndex},
    multi::separated_list1,
    sequence::{separated_pair, Tuple},
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

fn aoc((body, file): (String, String)) -> Result<Index> {
    // let mut r = 0;

    let mut lines = body.split('\n').enumerate();

    let (_, first_line) = lines
        .next()
        .ok_or_else(|| miette!("No lines in input {}!", file))?;

    let (_, (_, mut seeds)) = (
        tag("seeds: "),
        separated_list1(space1, separated_pair(parseIndex, space1, parseIndex)),
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

    for (name, maping) in &translations {
        let mut new_seeds = Vec::new();
        'seed: for (mut start, mut seed_size) in seeds {
            println!("{name} {start}+{seed_size}:");
            use std::ops::Bound::*;
            let range_left = maping.range((Unbounded, Included(start)));
            if let Some((from, (to, mapping_size))) = range_left.last() {
                let offset = start - from;
                if &offset < mapping_size {
                    let max_size = mapping_size - offset;
                    if max_size >= seed_size {
                        println!("    {start}+{seed_size} => {}", to + offset);
                        new_seeds.push((to + offset, seed_size));
                        continue 'seed;
                    }
                    println!("    {start}+{max_size} => {}", to + offset);
                    new_seeds.push((to + offset, max_size));
                    start += max_size;
                    seed_size -= max_size;
                }
            }
            let range_right = maping.range((Included(start), Unbounded));
            for (from, (to, mapping_size)) in range_right {
                if &start < from {
                    let gap_size = from - start;
                    if gap_size >= seed_size {
                        println!("    {start}+{seed_size} => {start}");
                        new_seeds.push((start, seed_size));
                        continue 'seed;
                    }
                    println!("    {start}+{gap_size} => {start}");
                    new_seeds.push((start, gap_size));
                    start += gap_size;
                    seed_size -= gap_size;
                }
                assert!(&start == from);
                if mapping_size >= &seed_size {
                    println!("    {start}+{seed_size} => {}", to);
                    new_seeds.push((*to, seed_size));
                    continue 'seed;
                }
                println!("    {start}+{mapping_size} => {}", to);
                new_seeds.push((*to, *mapping_size));
                start += mapping_size;
                seed_size -= mapping_size;
            }
            println!("    {start}+{seed_size} => {start}");
            new_seeds.push((start, seed_size));
        }
        seeds = new_seeds;
    }
    let mut min_val = Index::MAX;
    for (start, _seed_size) in seeds {
        if start < min_val {
            min_val = start;
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
        46
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
