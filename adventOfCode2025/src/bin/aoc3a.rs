use miette::{miette, IntoDiagnostic, Result};
use regex::Regex;
use adventOfCode2025::get_data;

fn aoc((body, _file): (String, String)) -> Result<i32> {
    let mut r = 0;

    for line in body.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue
        }

        for a in ('1' ..= '9').rev() {
            let Some(posa) = line.find(a) else {continue};
            let Some(b) = line[posa+1 ..].chars().max() else {continue};
            let v = (a as i32 - '0' as i32)*10+(b as i32 - '0' as i32);
            r += dbg!(v);
            break
        }
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
        357
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
