
use miette::{Result, IntoDiagnostic };
use std::str::FromStr;

fn part1( numbers: Vec<i32> ) -> Result<i32> {
    let numbers_len = numbers.len() as i32;

    let mut state = numbers.iter().copied().enumerate().collect::<Vec<_>>();
    
    for old_pos in numbers.iter().enumerate() {
        // println!("{:3} {state:?}",old_pos.0); 
        let pos = state.iter().position(|p|p.0==old_pos.0).expect("Number vanished!");
        let new_pos = (pos as i32 + old_pos.1).rem_euclid(numbers_len-1) as usize;
        if new_pos != pos {
            if new_pos < pos {
                state[new_pos..=pos].rotate_right(1);
            } else {
                state[pos..=new_pos].rotate_left(1);
            }
        }
    }
    // println!("END {state:?}");

    let pos_0 = state.iter().position(|p|p.1==0).expect("Number vanished!");
    dbg!(pos_0);

    let pos_a = (pos_0 + 1000).rem_euclid(state.len());
    dbg!(pos_a);

    let pos_b = (pos_0 + 2000).rem_euclid(state.len());
    dbg!(pos_b);

    let pos_c = (pos_0 + 3000).rem_euclid(state.len());
    dbg!(pos_c);

    Ok( dbg!(state[pos_a].1) + dbg!(state[pos_b].1) + dbg!(state[pos_c].1) )
}

fn _part1_error( numbers: Vec<i32> ) -> Result<i32>  {
    let numbers_len = numbers.len() as i32;

    let mut state = numbers.iter().copied().enumerate().collect::<Vec<_>>();
    
    for n in 0 .. state.len() {
        // println!("{state:?}");
        let pos = state.iter().position(|p|p.0==n).expect("Number vanished!"); 

        let value = state[pos].1;

        let mut new_pos = pos as i32 + value;

        if new_pos <= 0 {
            loop {
                new_pos += numbers_len - 1;
                if new_pos >= 0 {break}
            }
            assert!(new_pos>0);
        } else if new_pos >= numbers_len {
            new_pos += 1;
            loop {
                new_pos -= numbers_len - 1;
                if new_pos < numbers_len {break}
            }
            assert!(new_pos<numbers_len);
        } 
        {
            let new_pos = new_pos as usize;
            if new_pos < pos {
                state[new_pos..=pos].rotate_right(1);
            } else {
                state[pos..=new_pos].rotate_left(1);
            }
        }
    }
    // println!("{state:?}");

    let pos_0 = state.iter().position(|p|p.1==0).expect("Number vanished!");
    dbg!(pos_0);

    let pos_a = (pos_0 + 1000).rem_euclid(state.len());
    dbg!(pos_a);

    let pos_b = (pos_0 + 2000).rem_euclid(state.len());
    dbg!(pos_b);

    let pos_c = (pos_0 + 3000).rem_euclid(state.len());
    dbg!(pos_c);

    Ok( dbg!(state[pos_a].1) + dbg!(state[pos_b].1) + dbg!(state[pos_c].1) )
}

fn main() -> Result<()> { 
 
    let (mut body,_filename) = testing::get_data(file!())?;
   
    match body.pop() {
        Some('\n') => {},
        Some(c) => {
            body.push(c);
        }
        None => todo!(),
    }

    let  numbers  = body.split('\n').map( i32::from_str ).collect::<Result<Vec<_>,_>>().into_diagnostic()?;
 
    

    println!("answer_a={}",part1(numbers)?);

    Ok(())
}

