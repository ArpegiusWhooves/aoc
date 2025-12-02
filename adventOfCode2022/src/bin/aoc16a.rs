

#![feature(extract_if)]

use std::{collections::{HashMap, HashSet}, vec};

use miette::Result;
use testing::{ParserResultWithCode, get_data};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub aoc16p);

fn main() -> Result<()> { 
    let (body,filename) = get_data(file!())?;

    let valves: HashMap<&str, (i32, Vec<&str>)> = HashMap::from_iter(
        aoc16p::ValvesParser::new()
        .parse(&body) 
        .error_with_source(&filename, &body)?);
 
    let mut visited = vec![("AA", 0, 0, false, HashSet::from(["AA"]))];
    let mut answer_a = 0;
    for _ in 0 .. 31 {
        let mut new_visited = Vec::new();
        for (name,answer, score, opened, path) in visited {

            if answer_a < answer {
                answer_a = answer;
            }

            let (rate,tunels) = &valves[name];
            for tunel in tunels {
                if path.contains(tunel) { continue }
                let mut new_path = path.clone();
                new_path.insert(tunel);
                new_visited.push((*tunel, answer+score, score, false, new_path));
            }
            if rate > &0 && !opened {
                new_visited.push((name, answer+score, score+rate, true, path));
            }
        }
        visited = new_visited;
    }

    dbg!(answer_a);

    Ok(())
}

