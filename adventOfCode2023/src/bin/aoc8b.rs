use std::{collections::{BTreeMap, HashMap}, fmt::{Display, Write, Error}};


use itertools::Itertools;
use miette::{miette, Result};

use num::integer::lcm;
use testing::get_data;

// use nom::{ 
//     character::complete::{space1,alpha1},
//     multi::separated_list1,
//     sequence::Tuple,
// };
// use nom_supreme::{ParserExt,tag::complete::tag};

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Clone, Copy, Hash)]
struct Index( u32 );

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from_u32((self.0>>16)&0xff).ok_or(Error)?)?;
        f.write_char(char::from_u32((self.0>>8)&0xff).ok_or(Error)?)?;
        f.write_char(char::from_u32(self.0&0xff).ok_or(Error)?)
    }
}

impl Index {
    fn new(t: &[u8]) -> Index {
        // let t: &[u8] = s.as_bytes();
        Index( ((t[0] as u32) << 16) + ((t[1] as u32) << 8) + t[2] as u32 )
    }
    fn is_start(&self) -> bool{
        self.0&0xff == b'A' as u32
    }
    fn is_end(&self) -> bool{
        self.0&0xff == b'Z' as u32
    }
}

fn parse_maping(input: &str) -> Result<(Index, Index, Index)> {
    let t = input.as_bytes();
    Ok((Index::new(&t[0..3]),Index::new(&t[7..10]),Index::new(&t[12..15])))
}

fn aoc((body, file): (String, String)) -> Result<u64> {
    let mut r = 0;

    let mut lines = body.split('\n').enumerate();

    let mut map = BTreeMap::new();
    let mut start_pos = Vec::new();

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
        if i.is_start() {
            start_pos.push(i);
        }
        if !map.insert(i, (j,k)).is_none() {
            return Err(miette!("Repeated node {}",i));
        }
    }
    let mut pos = start_pos.clone();
    let mut loops = pos.iter().map(|_|HashMap::new()).collect_vec();
    let mut loops_offset = vec![0;pos.len()]; 
    let mut loops_size = vec![0;pos.len()]; 
    let mut loops_count = 0;
    loop {
        for w in first_line.bytes() {
            r+=1;
            // println!("{r} {}:",w as char);
            for (i,p) in pos.iter_mut().enumerate() {
                let Some(way) = map.get(p) else {
                    return Err(miette!("Unkown node {}",p));
                };
                let new_p = match w {
                    b'L' => way.0,
                    b'R' => way.1,
                    _ => return Err(miette!("Unkown dir {}",w as char)),
                };
                if new_p.is_end() {
                    // dbg!(i,new_p,start_pos[i]);

                    let last = loops[i].insert(new_p, r);

                    if let Some(lr) = last { // we have a loop
                        if loops_offset[i] == 0 {
                            loops_count = dbg!(loops_count) + 1;
                            loops_offset[i] = dbg!(lr);
                            loops_size[i] = dbg!(dbg!(r)-lr);
                        }                        
                    }
                    
                }
                // println!("  From {p} to {new_p}");
                *p = new_p;
            }
            // if end {
            //     return Ok(r)
            // }
            if loops_count == start_pos.len() {
                let v = loops_size.into_iter().reduce(lcm).unwrap(); 
 
                return Ok(v);
            }
        }
    }
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
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
