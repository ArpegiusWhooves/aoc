

use std::{str::FromStr, collections::{HashMap, VecDeque}};

use miette::{Result, IntoDiagnostic, miette };

use std::collections::hash_map::Entry::*;

fn main() -> Result<()> { 
 
    let (body,_filename) = testing::get_data(file!())?;
 
    let mut answer_a = 0; 


    let mut droplets = HashMap::new();
 
    let (mut min_x,mut max_x,mut min_y,mut max_y,mut min_z,mut max_z) = (0,0,0,0,0,0);

    for line in body.split('\n') {
        if line.is_empty() {
            continue;
        }
        let [x,y,z] = line.split(',').map(i32::from_str)
            .collect::<Result<Vec<_>,_>>()
            .into_diagnostic()?[..] else {
            return Err(miette!("Wrong number of elements!"))
        };
        droplets.insert((x,y,z),true);

        if min_x > x { min_x = x};
        if max_x < x { max_x = x};
        if min_y > y { min_y = y};
        if max_y < y { max_y = y};
        if min_z > z { min_z = z};
        if max_z < z { max_z = z};
    }

    min_x-=1;
    max_x+=1;
    min_y-=1;
    max_y+=1;
    min_z-=1;
    max_z+=1;

    let mut q = VecDeque::from([(min_x,min_y,min_z)]);

    while let Some((x,y,z)) = q.pop_back() {
        let mut check = |x,y,z|{ 
            let key = (x,y,z);
            match droplets.entry(key) {
                Occupied(o) => if *o.get() {answer_a += 1},
                Vacant(e) => {
                    e.insert(false);
                    q.push_front(key);
                },
            };
        };
        if x<max_x { check(x+1,y,z) }
        if x>min_x { check(x-1,y,z) }
        if y<max_y { check(x,y+1,z) }
        if y>min_y { check(x,y-1,z) }
        if z<max_z { check(x,y,z+1) }
        if z>min_z { check(x,y,z-1) }
    }

    dbg!(answer_a);
 
    Ok(())
}

