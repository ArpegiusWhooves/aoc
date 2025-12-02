



use std::collections::HashSet;

use itertools::Itertools;
use core::iter::IntoIterator;
use miette::{Result, IntoDiagnostic, miette};
use testing::MyBad;
use testing::Pos;
use std::str::FromStr;

#[derive(Debug,Clone, Copy)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3
}

fn main() -> Result<()> { 
 
    let (body,filename) = testing::get_data(file!())?;
  
    let mut rocks = HashSet::new();
    let mut bounds = Vec::new();
    let mut line_iter = body.split('\n').enumerate();
    for (y, map_line) in line_iter.by_ref() {
        let Some(start) = map_line.chars().position(|c|c!=' ') else {
            break;
        };
        for (x,c) in  map_line.chars().enumerate().skip(start)  {
            match c {
                '.' => continue,
                '#' => rocks.insert(Pos{x,y}),
                _=> {
                    return Err(MyBad::new(filename, body, (y+1,x+1,1), format!("Invalid character {c:?}")).into())
                }
            };
        }
        bounds.push(dbg!(start ..= map_line.len() - 1 ));
        // assert!(dbg!(bounds.len()) == dbg!(y+1));
    }

    let max_y = bounds.len() - 1;

    let Some((_move_line, moves)) = line_iter.next() else {
        return Err(miette!("No moves found, end of file."));
    };
    
    let mut pos = Pos{x:*bounds[0].start(),y:0};
    let mut dir = Dir::Right;

    while rocks.contains(dbg!(&pos)) {
        pos.x+=1;
    }

    println!("Starting at {pos:?} going {dir:?}.");

    for (is_a_num, characters) in &moves.chars().inspect(|p|println!("Readed {p:?}")).group_by(|c| c.is_numeric()) {
        
        println!("Is a num: {is_a_num:?}");
        if is_a_num {
            let number = characters.into_iter().collect::<String>();
            let distance = usize::from_str(dbg!(&number)).into_diagnostic()?;
            for _cnt in 0 .. distance {
                let new_pos = match dir {
                    Dir::Right => {
                        if &pos.x < bounds[pos.y].end() {
                            Pos{ x: pos.x + 1, y:pos.y }
                        } else {
                            Pos{ x: *bounds[pos.y].start(), y:pos.y }
                        }
                    },
                    Dir::Down => {
                        if pos.y < max_y  && bounds[pos.y+1].contains(&pos.x)  {
                            Pos{ x: pos.x, y:pos.y+1 }
                        } else {
                            Pos{ x: pos.x,
                                y: ( 0 .. pos.y ).find(|&new_y| bounds[new_y].contains(&pos.x)).unwrap()
                            }
                        }
                    },
                    Dir::Left => {
                        if &pos.x > bounds[pos.y].start() {
                            Pos{ x: pos.x - 1, y: pos.y }
                        } else {
                            Pos{ x: *bounds[pos.y].end(), y:pos.y }
                        }
                    },
                    Dir::Up => {
                        if pos.y > 0 && bounds[pos.y-1].contains(&pos.x) {
                            Pos{ x: pos.x, y: pos.y - 1 }
                        } else {
                            Pos{ x: pos.x,
                                 y: (pos.y+1 ..= max_y).rev().find(|&new_y| bounds[new_y].contains(&pos.x)).unwrap()
                            }
                        }
                    },
                };
                if rocks.contains( &new_pos ) {
                    println!("Crashed at {new_pos:?} after {_cnt} moves of {distance} to the {dir:?}.");
                    break;
                }
                pos = new_pos;  
            }
        } else {
            dir = match characters.into_iter().next() {
                Some('R') => match dir {
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                    Dir::Up => Dir::Right,
                },
                Some('L') => match dir {
                    Dir::Right => Dir::Up,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Down,
                    Dir::Up => Dir::Left,
                },
                _ => return Err(miette!("Invalid move sequence.")),
            };
            println!("Turned to {dir:?}.");
        }   
    }
 
    println!("Ended at {pos:?} with facing {dir:?}.");

    let answer = 1000 * (pos.y+1) + 4 * (pos.x+1) + (dir as usize);
    println!("answer:{answer}");
    Ok(())
}

