use miette::{miette, IntoDiagnostic, Result};
use regex::Regex;
use testing::get_data;

fn aoc1((body, file): (String, String)) -> Result<i32> {
    let mut r = 0;
    let re = Regex::new(r"([0-9])").into_diagnostic()?;

    for (no, line) in body.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut matches = re.find_iter(line);
        let Some(first) = matches.next() else {
            return Err(miette!("No match in line {} of {}", no, file));
        };

        r += match matches.last() {
            Some(last) => format!("{}{}", first.as_str(), last.as_str())
                .parse::<i32>()
                .into_diagnostic()?,
            None => format!("{}{}", first.as_str(), first.as_str())
                .parse::<i32>()
                .into_diagnostic()?,
        }
    }
    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc1((
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"
                .to_owned(),
            "text1".to_owned()
        ))
        .expect("no errors"),
        142
    );
}

fn main() -> Result<()> {
    println!("{}", aoc1(get_data(file!())?)?);

    Ok(())
}
