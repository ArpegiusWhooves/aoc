

 

use miette::Result; 
use miette::miette;
use testing::get_data;

fn main() -> Result<()> { 
    let (body,_) = get_data(file!())?;

    let mut sum: u32 = 0; 
 
    for line in body.split('\n') {
        let s = line.len()/2;
        dbg!(line);
        'x: for a in line[0..s].bytes() {
            for b in line[s..line.len()].bytes() { 
                if a == b {
                    match a {
                        b'a' ..= b'z' => { sum += (a - b'a') as u32 + 1; }
                        b'A' ..= b'Z' => { sum += (a - b'A') as u32 + 27; }
                        _ => { return Err(miette!("unknown character '{a}'")); }
                    }
                    dbg!(a as char);
                    break 'x;
                }
            }

        }
    }

    println!("{sum}");

    Ok(())
}
