use std::option;

use miette::{miette, IntoDiagnostic, Result};
use regex::Regex;
use adventOfCode2025::get_data;



fn find_largest(line: &str, cnt: i32) -> Option<i64> {
    for a in ('1' ..= '9').rev() {
        let Some(posa) = line.find(a) else {continue};
        let Some(b) = (if cnt > 1 {
            find_largest(&line[posa+1 ..], cnt-1)
        } else {
            line[posa+1 ..].chars().max().map(|v| v as i64 - '0' as i64)
        }) else {continue};
        return Some( (a as i64 - '0' as i64) * 10_i64.pow(cnt as u32) + dbg!(b))
    }
    None
}

fn aoc((body, _file): (String, String)) -> Result<i64> {
    let mut r = 0;

    for line in body.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue
        }

        let Some(v) = find_largest(line, 11) else {
            println!("something went wrong");
            continue;
        };
        r += dbg!(v);
    }
    
    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            r##"
987654321111111
811111111111119
234234234234278
818181911112111
            "##
                .to_owned(),
            "test".to_owned()
        ))
        .expect("no errors"),
        3121910778619
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
