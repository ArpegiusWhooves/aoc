use miette::{miette, IntoDiagnostic, Result};
use regex::{Captures, Regex};
use testing::get_data;

fn match_to_val<'h>(cap: &Captures<'h>) -> i32 {
    for i in 1..=9 {
        if cap.get(i).is_some() {
            return i as i32;
        }
    }
    cap[0].parse().unwrap()
}

fn aoc1((body, file): (String, String)) -> Result<i32> {
    let mut r = 0;
    let re_l = Regex::new(r"(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|[0-9]")
        .into_diagnostic()?;
    let re_r = Regex::new(r"(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)|[0-9]")
        .into_diagnostic()?;

    for (no, line) in body.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        dbg!(line);

        let Some(first) = re_l.captures(line) else {
            continue;
        };

        let r_line: String = line.chars().rev().collect();

        let Some(last) = re_r.captures(&r_line) else {
            return Err(miette!("No match in line {} of {}", no, file));
        };

        let d = match_to_val(&first) * 10 + match_to_val(&last);
        r += dbg!(d);
    }
    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc1((
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen 
            "
            .to_owned(),
            "text1".to_owned()
        ))
        .expect("no errors"),
        281
    );
    assert_eq!(
        aoc1((
            "
            qazonezxc
            1eightwo
            "
            .to_owned(),
            "text1".to_owned()
        ))
        .expect("no errors"),
        23
    );
}

fn main() -> Result<()> {
    println!("{}", aoc1(get_data(file!())?)?);

    Ok(())
}
