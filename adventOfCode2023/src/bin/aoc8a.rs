use std::{collections::BTreeMap, fmt::{Display, Write}};


use miette::{miette, IntoDiagnostic, Result};

use testing::get_data;

// use nom::{ 
//     character::complete::{space1,alpha1},
//     multi::separated_list1,
//     sequence::Tuple,
// };
// use nom_supreme::{ParserExt,tag::complete::tag};

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Clone, Copy, Hash)]
struct Index( [u8;3] );

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0[0] as char)?;
        f.write_char(self.0[1] as char)?;
        f.write_char(self.0[2] as char)
    }
}
const START:Index = Index([b'A',b'A',b'A']);
const END:Index = Index([b'Z',b'Z',b'Z']);

fn parse_maping(input: &str) -> Result<(Index, Index, Index)> {
        let i: [u8;3] = input.as_bytes()[0..3].try_into()
                        .into_diagnostic()?;
        let j: [u8;3] = input.as_bytes()[7..10].try_into()
                        .into_diagnostic()?;
        let k: [u8;3] = input.as_bytes()[12..15].try_into()
                        .into_diagnostic()?;
     Ok((Index(i),Index(j),Index(k)))
}

fn aoc((body, file): (String, String)) -> Result<u32> {
    let mut r = 0;

    let mut lines = body.split('\n').enumerate();

    let mut map = BTreeMap::new();
    let (_, first_line) = lines
        .next()
        .ok_or_else(|| miette!("No lines in input {}!", file))?;

    dbg!(first_line);

    if !lines.next().unwrap().1.is_empty() {
        return Err(miette!("Second line should be empty."));
    }

    for (_no,line) in lines {
        if line.is_empty() {continue}
        let (i,j,k) = parse_maping(line)?;
        if !map.insert(i, (j,k)).is_none() {
            return Err(miette!("Repeated node {}",i));
        }
    }
    let mut pos = START;
    loop {
        for w in first_line.bytes() {
            let Some(way) = map.get(&pos) else {
                return Err(miette!("Unkown node {}",pos));
            };
            let new_pos = match w {
                b'L' => way.0,
                b'R' => way.1,
                _ => return Err(miette!("Unkown dir {}",w as char)),
            };
            r+=1;
            println!("{r}: From {pos} to {} {new_pos}",w as char);
            if new_pos == END {
                return Ok(r);
            }
            pos = new_pos;
        }
    }
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors"),
        2
    );
}

#[test]
fn test2() {
    assert_eq!(
        aoc((
            "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors"),
        6
    );
}
fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
