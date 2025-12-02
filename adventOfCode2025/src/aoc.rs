use miette::{miette, IntoDiagnostic, Result};
use regex::Regex;
use adventOfCode2025::get_data;

fn aoc((body, _file): (String, String)) -> Result<i32> {
    let mut r = 0;



    
    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            r##""##
                .to_owned(),
            "test".to_owned()
        ))
        .expect("no errors"),
        142
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
