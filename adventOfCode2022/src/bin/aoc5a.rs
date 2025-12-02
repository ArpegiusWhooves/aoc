

 
  
use itertools::Itertools;
use lalrpop_util::lalrpop_mod;

use miette::{Result, miette};

lalrpop_mod!(pub aoc5p);
 
use testing::{ParserResultWithCode, get_data};

fn main() -> Result<()> { 
    let (body,filename) = get_data(file!())?;
  
    let parser = aoc5p::MoveParser::new();
 
    let mut cargo_contetnts : Vec<_> = (0..10).map(|_| Vec::<char>::new() ).collect(); 

    let mut content = body.split('\n');

    for cargo_line in content.take_while_ref(|e| e.contains('[')) {
        for (cargo_pos,cargo_item) in cargo_line.chars().enumerate() {
            if cargo_item < 'A' {continue}
            if cargo_item > 'Z' {continue}
            cargo_contetnts[ cargo_pos / 4 + 1 ].push( cargo_item );
        }
    }
    dbg!(content.next()); 
    dbg!(content.next()); 
    for cargo in &mut cargo_contetnts {
        cargo.reverse();
    } 

    for line_contetnt in content {

        if line_contetnt.is_empty() {break;}

        let (move_size,move_from, move_to) = parser
        .parse(line_contetnt)
        .error_with_source(&filename, line_contetnt)?;

        for i in 0 .. move_size  {
                let Some(c) = cargo_contetnts[ move_from ].pop() else {
                    return Err(miette!("Cargo {} is empty after {} move!", move_from, i))
                };
                cargo_contetnts[ move_to ].push(c);
        } 
    }
     
    dbg!(&cargo_contetnts);

    let result : String = cargo_contetnts.iter().map(|c| c.last().unwrap_or(&' ')).collect();

    dbg!(result);

    Ok(())
}
