use miette::{IntoDiagnostic, Result};
use itertools::Itertools;
use adventOfCode2025::get_data;
use std::collections::HashSet;


fn aoc((body, _file): (String, String)) -> Result<i64> {
    let mut r = 0;

    for range in body.trim().split(',') {
        if range.is_empty() {continue}
        let Some((a,b)) = range.split('-').next_tuple() else {println!("Invalid range {range}");continue};
        
        let min: i64 = a.parse().into_diagnostic()?;
        let max: i64 = b.parse().into_diagnostic()?;

        let mut start = 1;
        let mut used = HashSet::new();

        loop {
            let mut repeated = 1;
            let ss = start.to_string();
            

            loop {
                repeated += 1;
                let v: i64 = ss.repeat(repeated).parse().unwrap();
                if v < min {continue}
                if v > max {break}
                if used.insert(v) {
                    println!("{min} {max} => {v}");
                    r += v;
                }
            }
            if repeated == 2 {break}
            start += 1;
        }
        

    }

    
    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            r##"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"##
                .to_owned(),
            "test".to_owned()
        ))
        .expect("no errors"),
        4174379265
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
