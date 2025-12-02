
use miette::{Result, IntoDiagnostic}; 
use testing::get_data;

fn main() -> Result<()> { 

    let (body,_) = get_data(file!())?;


    let mut sum = 0;
    let mut max = 0;

    let mut maxlist = Vec::new();
    
    for cal in body.split('\n') {
        
        if cal.is_empty() {
            if sum > max {
                maxlist.push(sum);
                maxlist.sort();
                maxlist.reverse();
                dbg!( sum, max, &maxlist );
                if maxlist.len() > 3 {
                    maxlist.pop();
                }
                max = *maxlist.last().unwrap();
            }
            sum = 0;
            dbg!( sum, max );
            continue;
        }

        let val = cal.parse::<i32>().into_diagnostic()?;
        sum += val;

    }

    dbg!(max, &maxlist );

    let result : i32 = maxlist.iter().sum();

    println!("{result}");

    Ok(())
}
