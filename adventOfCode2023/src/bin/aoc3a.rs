use miette::{IntoDiagnostic, Result};
use regex::Regex;
use testing::get_data;

fn aoc((body, _file): (String, String)) -> Result<i32> {
    let mut r = 0;
    let re = Regex::new(r"([0-9]+)").into_diagnostic()?;

    let mut pp = Vec::new();
    let mut pm: Vec<regex::Match<'_>> = Vec::new();

    for (_no, line) in body.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        let pc: Vec<(usize, char)> = line
            .char_indices()
            .filter(|(_, c)| match c {
                '.' | '0'..='9' => false,
                _ => true,
            })
            .collect();
        for m in pm {
            for (i, c) in &pc {
                if m.start() <= i + 1 && *i <= m.end() {
                    dbg!(c);
                    r += dbg!(m.as_str()).parse::<i32>().into_diagnostic()?;
                    break;
                }
            }
        }
        pm = Vec::new();
        'l: for m in re.find_iter(line) {
            for (i, c) in &pp {
                if m.start() <= i + 1 && *i <= m.end() {
                    dbg!(c);
                    r += dbg!(m.as_str()).parse::<i32>().into_diagnostic()?;
                    continue 'l;
                }
            }
            for (i, c) in &pc {
                if m.start() <= i + 1 && *i <= m.end() {
                    dbg!(c);
                    r += dbg!(m.as_str()).parse::<i32>().into_diagnostic()?;
                    continue 'l;
                }
            }
            pm.push(m);
        }
        pp = pc;
    }
    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .to_owned(),
            "text1".to_owned()
        ))
        .expect("no errors"),
        4361
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
