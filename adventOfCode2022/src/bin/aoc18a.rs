

use std::{str::FromStr, collections::HashSet};

use miette::{Result, IntoDiagnostic, miette };


fn main() -> Result<()> { 
 
    let (body,_filename) = testing::get_data(file!())?;
 
    let mut answer_b = 0;

    let mut droplets = HashSet::new();
 
    for line in body.split('\n') {
        if line.is_empty() {
            continue;
        }
        let [x,y,z] = line.split(',').map(i32::from_str)
            .collect::<Result<Vec<_>,_>>()
            .into_diagnostic()?[..] else {
            return Err(miette!("Wrong number of elements!"))
        };
        droplets.insert((x,y,z));
    }
 
    for &(x,y,z) in droplets.iter() {
        if !droplets.contains(&(x+1,y,z)) {
            answer_b += 1;
        }
        if !droplets.contains(&(x-1,y,z)) {
            answer_b += 1;
        }
        if !droplets.contains(&(x,y+1,z)) {
            answer_b += 1;
        }
        if !droplets.contains(&(x,y-1,z)) {
            answer_b += 1;
        }
        if !droplets.contains(&(x,y,z+1)) {
            answer_b += 1;
        }
        if !droplets.contains(&(x,y,z-1)) {
            answer_b += 1;
        }
    }

    dbg!(answer_b);
 
    Ok(())
}

