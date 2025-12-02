use miette::{Context, IntoDiagnostic, Result};
use pathfinding::matrix::directions::S;
use testing::get_data;

type CardIndex = u32;
type CardNumber = u32;

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{digit1,u8 as U8, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, tuple, Tuple}
};

use nom_supreme::ParserExt;

struct  Person {
    userId: String
}


fn aoc((body, file): (String, String)) -> Result<u32> {
    let mut r = 0;

    for (no, line) in body.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        
        let (input,(pattern,list)) = (
            take_till(|c| c ==' '),
            space1.precedes(separated_list1(tag(","), U8) ))
        .parse(dbg!(line))
        .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| e.to_owned())
        .into_diagnostic()?;
        
        let minmax = count_minmax(pattern.as_bytes());
        
        r+= count_posibilities(pattern.as_bytes(), minmax, &list, list.iter().sum());
    }

    Ok(r)
}

fn count_minmax(pattern: &[u8]) -> (u8,u8) {
    let mut a = 0;
    let mut b = 0;
    for &c in pattern {
        if c == b'#' {a+=1}
        else if c == b'?' {b+=1}
    }
    return (a,a+b);
}

fn count_posibilities(pattern: &[u8], (min,max): (u8,u8), list: &[u8], sum: u8) -> u32 {
    if sum == 0 && min == 0 { return 1; }
    if max < sum || sum < min {
        return 0;
    }
    let mut i =0;
    while pattern[i] == b'.' {i+=1;}
    let mut first = list[0];
    if pattern[i] == b'#' {
        loop {
            first -= 1;
            i += 1;
            if pattern.len() <= i {
                if first == 0 {
                    return 1;
                }

            }
                    
                


            
            if pattern[i] == b'.' {

            }

        }
    }
     
    0
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors"),
        21
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
