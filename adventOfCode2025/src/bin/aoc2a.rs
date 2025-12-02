use miette::{IntoDiagnostic, Result};
use itertools::Itertools;
use adventOfCode2025::get_data;

fn double(i:i64)->i64 {
    i.to_string().repeat(2).parse().unwrap()
}

fn aoc((body, _file): (String, String)) -> Result<i64> {
    let mut r = 0;

    for range in body.trim().split(',') {
        if range.is_empty() {continue}
        let Some((a,b)) = range.split('-').next_tuple() else {println!("Invalid range {range}");continue};
        
        let min: i64 = a.parse().into_diagnostic()?;
        let max: i64 = b.parse().into_diagnostic()?;
        let mut start:i64 = if a.len()<2 {1} else {a[0..(a.len())/2].parse().into_diagnostic()?};

        while double(start) < min {
            start += 1;
        }
        
        loop {
            let v = double(start);
            if v > max {break}
            println!("{min} {max} => {v}");
            start += 1;
            r += v;
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
        1227775554
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
