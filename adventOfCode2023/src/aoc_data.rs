use ureq::get;

use miette::{miette, IntoDiagnostic, Result};
use dotenv::dotenv;

pub fn get_data(day: &str) -> Result<(String, String), miette::ErrReport> {
    if &day[0..11] != "src/bin/aoc" {
        return Err(miette!("Bad day name."));
    }
    let day: String = day[11..].chars().take_while(char::is_ascii_digit).collect();

    let filename = format!("input/input{day}.txt");
    let path = std::path::Path::new(&filename);

    if path.exists() {
        return std::fs::read_to_string(path)
            .map(|body| (body, filename))
            .into_diagnostic();
    }
    dotenv().ok();

    let contents = get(&format!("https://adventofcode.com/2023/day/{day}/input"))
    .set("cookie", format!("session={}",std::env::var("session").expect("session must be set.")).as_str())    
    .call().into_diagnostic()?
    .into_string().into_diagnostic()?;

    std::fs::write(path, &contents).into_diagnostic()?;

    Ok((contents, filename))
}
