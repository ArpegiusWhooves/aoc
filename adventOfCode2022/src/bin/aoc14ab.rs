



use std::collections::HashSet;

use itertools::Itertools;
use miette::Result;


use lalrpop_util::lalrpop_mod;
use testing::{get_data, ParserResultWithCode};
lalrpop_mod!(pub aoc14p);

fn main() -> Result<()> { 
    let (body,filename) = get_data(file!())?;
 
    let mut answer_a = 0;
    let mut answer_b = 0;

    let mut blocked = HashSet::<(i32,i32)>::new();
    let mut lowest_block = 0;
 
    for line in body.split('\n') {
        if line.is_empty() {
            continue;
        }

        let p = aoc14p::ListParser::new().parse(line).error_with_source(&filename, &body)?;

        for (a,b) in p.iter().tuple_windows() { 

            let x_min =  a.0.min(b.0);
            let x_max =  a.0.max(b.0);
            let y_min =  a.1.min(b.1);
            let y_max =  a.1.max(b.1);

            if lowest_block < y_max { lowest_block = y_max }

            for block in (x_min ..= x_max).cartesian_product(y_min ..= y_max) {
                blocked.insert(block);
            }
        }
    }

    lowest_block += 1;

    let mut current_sand = (500,0);

    loop {
        let (x,y) = current_sand;

        if y == lowest_block {
            if answer_a == 0 { answer_a = answer_b };
            answer_b+=1;
            blocked.insert(current_sand);
            current_sand = (500,0);
            continue
        }

        let down = (x,y+1);
        if  blocked.get(&down).is_none() {
            current_sand = down;
            continue
        }
        let left = (x-1,y+1);
        if  blocked.get(&left).is_none() {
            current_sand = left;
            continue
        }
        let right = (x+1,y+1);
        if  blocked.get(&right).is_none() {
            current_sand = right;
            continue
        }
 
        answer_b+=1;
        if y == 0 {break}

        blocked.insert(current_sand);
        current_sand = (500,0); 
    }
 
    dbg!(answer_a);
    dbg!(answer_b);
 

    Ok(())
}

