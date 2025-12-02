


use miette::Result;
use testing::get_data;

fn main() -> Result<()> { 
    let (body,_) = get_data(file!())?;
 
    let mut sum_a = 0;
    let mut sum_b = 0;

    for line_contetnt in body.split('\n') {
        if line_contetnt.is_empty() {break}
  
        for (index,mark) in line_contetnt.as_bytes().windows(4).enumerate() {
            if mark.iter()
                .enumerate().skip(1)
                .any(|(i,c)| 
                    mark[..i].contains(c) 
                ) { continue }
            
            sum_a += index + 4;
            break;
        }    
        for (index,mark) in line_contetnt.as_bytes().windows(14).enumerate() {
            if mark.iter()
                .enumerate().skip(1)
                .any(|(i,c)| 
                    mark[..i].contains(c) 
                ) { continue }
            
            sum_b += index + 14;
            break;
        }    
    }

    dbg!(sum_a);
    dbg!(sum_b);
  
    Ok(())
}
