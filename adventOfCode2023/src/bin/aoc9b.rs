
 
use miette::{IntoDiagnostic, Result, Context};

use testing::get_data;
use std::str::FromStr;
type Index = i32;
// use ndarray::{Array1,s};

// #[derive(Debug,Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
// struct Maping {
//     from: Index,
//     to: Index,
//     size: Index
// }

fn extend_line(mut t: Vec<Index>) -> Index {
    dbg!(&t);
    let size = t.len();
    for window in 1..size  {
        for index in (window..size).rev() { 
            t[index] = t[index] - t[index-1];
        }
        dbg!(&t);
    }
    t.into_iter().rev().reduce(|a,b|b-a).unwrap()
}

fn aoc((body, _file): (String, String)) -> Result<Index> {
    let mut r = 0;

    for (no,line) in body.split("\n").enumerate() {
        if line.is_empty() {continue}
        let data: Vec<_>= line.split(' ')
            .map(Index::from_str)
            .collect::<Result<_,_>>()
            .into_diagnostic().with_context(||format!("Line {no} of {_file}."))?;
        r += extend_line(data);
    }

    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            "\
10 13 16 21 30 45
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors allowed"),
        5
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
