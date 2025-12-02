

 

use std::collections::BTreeSet;

use itertools::Itertools;
use miette::Result;
use testing::get_data;
 
fn main() -> Result<()> { 
    let (body,_) = get_data(file!())?;

    let mut sum: u32 = 0; 
 
    for mut group in &body.split('\n').map(|line| {
        BTreeSet::from_iter(line.bytes())
    }).chunks(3) {

        let mut x = group.next().unwrap();
        for y in group {
            x.retain(|e| y.contains(e) );
        }

        for e in x {
            let c = e as char;
            println!("{c}");
            match e {
                b'a' ..= b'z' => { sum += (e - b'a') as u32 + 1; }
                b'A' ..= b'Z' => { sum += (e - b'A') as u32 + 27; }
                _ => { println!("unknown character '{c}'"); }
            }
        }
    }
      
    println!("{sum}");

    Ok(())
}
