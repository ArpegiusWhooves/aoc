

 
 
use std::collections::HashSet;

use miette::{Result, IntoDiagnostic };
use testing::get_data;

#[derive(Debug,Default,Clone,Copy,Hash,PartialEq,Eq)]
struct Point {
    x: i32,
    y: i32
}

impl  Point {
    
    fn step(&mut self,dir:char) {
        match dir {
            'U' => self.y -= 1,
            'D' => self.y += 1,
            'L' => self.x -= 1,
            'R' => self.x += 1,
            _ => {}            
        }
    }
    fn move_to(&mut self, to: &Point) {
        let dx = to.x - self.x;
        let dy = to.y - self.y;

        if dx > 1 {
            self.x += 1;
            self.y += dy.signum();
            return
        } else if dx < -1 {
            self.x += -1;
            self.y += dy.signum();
            return
        } 

        if dy > 1 {
            self.y += 1;
            self.x += dx.signum();
        } else if dy < -1 {
            self.y += -1;
            self.x += dx.signum();
        }
    }
}


fn main() -> Result<()> { 
 
    let (body,_) = get_data(file!())?;
 


    let mut marked_a = HashSet::new();
    let mut marked_b = HashSet::new();

    // let mut head = Point::default();
    let mut rope = [Point::default();10];

    // marked.insert(rope[9].clone());
 
    for line in body.split('\n') {
        let dir  = match line.chars().next() { 
            Some(dir) => dir,
            None => continue,
        };
        let steps = line[2..].parse::<i32>().into_diagnostic()?;
        for _ in 0 .. steps {
            rope[0].step(dir);
            let mut prev = rope[0];
            // dbg!(&rope);
            
            for tail in &mut rope[ 1 .. ] {
                tail.move_to(&prev);
                prev = *tail;
            }
            // dbg!(&rope);
            marked_a.insert(rope[1]);
            marked_b.insert(rope[9]);
            // dbg!(marked.len());
        }
    }
    let answer_a = marked_a.len();
    let answer_b = marked_b.len();


    dbg!(answer_a);
    dbg!(answer_b);

    Ok(())
}

