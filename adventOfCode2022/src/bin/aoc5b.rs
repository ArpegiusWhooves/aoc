

 
  
use itertools::Itertools;

use miette::{Result, miette};


use lalrpop_util::{lalrpop_mod};
use testing::{get_data, ParserResultWithCode};
lalrpop_mod!(pub aoc5p);
  

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

    dbg!(&cargo_contetnts);

    for line_contetnt in content {

        if line_contetnt.is_empty() {break;}

        let (move_size,move_from, move_to) = parser
        .parse(line_contetnt)
        .error_with_source(&filename,line_contetnt)?;

        if move_from == move_to {
            return Err( miette!("Cannot move to the same location!") );
        }
 
        // dbg!(move_size,move_from, move_to);
        // dbg!(&cargo_contetnts[move_from],&cargo_contetnts[move_to]);


        // split Vec and take two mutable reference
        let (cargo_form,cargo_to) = if move_from < move_to  {
            let (a,b) = cargo_contetnts.split_at_mut( move_to );
            (&mut a[move_from], &mut b[0])
        } else {
            let (a,b) = cargo_contetnts.split_at_mut( move_from );
            (&mut b[0],&mut a[move_to])
        };
        // dbg!(&cargo_form,&cargo_to);
  
        if cargo_form.len() < move_size { 
            return Err(miette!("Cargo {} dont have enought elements, needed {}, have {}.",move_from,move_size,cargo_form.len()));
        }

        cargo_to.extend_from_slice( &cargo_form[ cargo_form.len() - move_size .. cargo_form.len() ] );
        cargo_form.truncate(cargo_form.len() - move_size);
    }
 
     
    dbg!(&cargo_contetnts);

    let result : String = cargo_contetnts.iter().map(|c| c.last().unwrap_or(&' ')).collect();

    dbg!(result);

    Ok(())
}
