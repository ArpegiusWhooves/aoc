

 
  
use lalrpop_util::lalrpop_mod;
use miette::{Result};
use testing::{ParserResultWithCode, get_data};

lalrpop_mod!(pub aoc4p);
 

fn main() -> Result<()> { 
    let (body,filename) = get_data(file!())?;
 
    let content = aoc4p::PairsParser::new()
        .parse(&body)
        .error_with_source(&filename, &body)?;

    let mut answer_a: u32 = 0;
    let mut answer_b: u32 = 0;
 
    for ((a_from,a_to),(b_from,b_to)) in content {

        if a_from <= b_from && b_to <= a_to 
        || b_from <= a_from && a_to <= b_to {
            answer_a += 1;
        }  
        if a_from <= b_from && b_from <= a_to 
        || b_from <= a_from && a_from <= b_to {
            answer_b += 1;
        } 
    }
     
    dbg!(answer_a,answer_b);

    Ok(())
}
