use adventOfCode2025::get_data;
use miette::{IntoDiagnostic, Result, miette};

fn aoc((body, _file): (String, String)) -> Result<i32> {
    let mut r = 0;
    let mut p = 50;

    for line in body.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let i: i32 = line[1..].parse().into_diagnostic()?;
        match line.chars().next() {
            Some('L') => {
                if p == 0 {p=100}
                p -= i;
                while p <= 0 {
                    p += 100;
                    r += 1;
                }
            }
            Some('R') => {
                if p == 100 {p=0}
                p += i;
                while p >= 100 {
                    p -= 100;
                    r += 1;
                }
            }
            _ => return Err(miette!("Unown rotation: {line:?}")),
        }
    }
    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            r##"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"##
                .to_owned(),
            "test".to_owned()
        ))
        .expect("no errors"),
        6
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
