

 

use miette::Result;
use testing::get_data; 

fn main() -> Result<()> { 
    let (body,_) = get_data(file!())?;

    let mut sum = 0; 
 
    for line in body.split('\n') {
        
        match line {
            "A X" => {sum += 1 + 3;}
            "A Y" => {sum += 2 + 6;}
            "A Z" => {sum += 3;}
            "B X" => {sum += 1;}
            "B Y" => {sum += 2 + 3;}
            "B Z" => {sum += 3 + 6;}
            "C X" => {sum += 1 + 6;}
            "C Y" => {sum += 2;}
            "C Z" => {sum += 3 + 3;}
            _ => { println!("unknown line {line}")}
        }
    }
      

    println!("{sum}");

    Ok(())
}
