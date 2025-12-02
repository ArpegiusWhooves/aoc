use miette::{IntoDiagnostic, Result};
use regex::Regex;
use testing::get_data;

#[derive(Debug, Clone, Copy)]
enum Gear {
    Empty,
    One(i32),
    Two(i32, i32),
    NotAGear,
}

fn aoc((body, _file): (String, String)) -> Result<i32> {
    let mut r = 0;
    let re = Regex::new(r"([0-9]+)").into_diagnostic()?;

    let mut pp = Vec::new();
    let mut pm: Vec<regex::Match<'_>> = Vec::new();

    for (_no, line) in body.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut pc: Vec<(usize, Gear)> = line
            .char_indices()
            .filter_map(|(i, c)| match c {
                '*' => Some((i, Gear::Empty)),
                _ => None,
            })
            .collect();
        for m in pm {
            for (i, g) in &mut pc {
                let v = m.as_str().parse::<i32>().into_diagnostic()?;
                if m.start() <= *i + 1 && *i <= m.end() {
                    *g = match g {
                        Gear::Empty => Gear::One(v),
                        Gear::One(v2) => Gear::Two(*v2, v),
                        Gear::Two(_, _) => Gear::NotAGear,
                        Gear::NotAGear => Gear::NotAGear,
                    }
                }
            }
        }
        pm = Vec::new();
        for m in re.find_iter(line) {
            for (i, g) in &mut pp {
                let v = m.as_str().parse::<i32>().into_diagnostic()?;
                if m.start() <= *i + 1 && *i <= m.end() {
                    *g = match g {
                        Gear::Empty => Gear::One(v),
                        Gear::One(v2) => Gear::Two(*v2, v),
                        Gear::Two(_, _) => Gear::NotAGear,
                        Gear::NotAGear => Gear::NotAGear,
                    }
                }
            }
            for (i, g) in &mut pc {
                let v = m.as_str().parse::<i32>().into_diagnostic()?;
                if m.start() <= *i + 1 && *i <= m.end() {
                    *g = match g {
                        Gear::Empty => Gear::One(v),
                        Gear::One(v2) => Gear::Two(*v2, v),
                        Gear::Two(_, _) => Gear::NotAGear,
                        Gear::NotAGear => Gear::NotAGear,
                    }
                }
            }
            pm.push(m);
        }
        for (_i, g) in pp {
            match g {
                Gear::Two(v1, v2) => {
                    r += dbg!(v1) * dbg!(v2);
                }
                _ => {}
            };
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
        467835
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
