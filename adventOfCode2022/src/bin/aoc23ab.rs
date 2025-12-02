

#![feature(extract_if)]

use std::{collections::{HashMap, hash_map::Entry::{Occupied, Vacant}}, ops::Add};

use miette::Result;

use glam::IVec2;
use testing::MyBad;
use bitmask_enum::bitmask;

#[bitmask(u8)]
enum ElveMove {
    NE,N,NW,
    E,   W,
    SE,S,SW,
}

const TOP: ElveMove = ElveMove::NE.or(ElveMove::N).or(ElveMove::NW);
const BOTTOM: ElveMove = ElveMove::SE.or(ElveMove::S).or(ElveMove::SW);
const LEFT: ElveMove = ElveMove::NW.or(ElveMove::W).or(ElveMove::SW);
const RIGHT: ElveMove = ElveMove::NE.or(ElveMove::E).or(ElveMove::SE);

const NEIGHBOURS:[(IVec2,ElveMove);8] = [
    (IVec2{x:-1,y:-1}, ElveMove::NW), (IVec2{x: 0,y:-1}, ElveMove::N), (IVec2{x: 1,y:-1}, ElveMove::NE),
    (IVec2{x:-1,y: 0}, ElveMove::W ),                                  (IVec2{x: 1,y: 0}, ElveMove::E ),
    (IVec2{x:-1,y: 1}, ElveMove::SW), (IVec2{x: 0,y: 1}, ElveMove::S), (IVec2{x: 1,y: 1}, ElveMove::SE),
];

const AVAILBE_MOVE: [(ElveMove, ElveMove, IVec2);4] = [
    (TOP,ElveMove::N,IVec2::NEG_Y),
    (BOTTOM,ElveMove::S,IVec2::Y),
    (LEFT,ElveMove::W,IVec2::NEG_X),
    (RIGHT,ElveMove::E,IVec2::X)
];

struct Rectangle {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Rectangle {
    fn extend(&mut self, pos: &IVec2) {
        if pos.x < self.min_x {self.min_x = pos.x}
        if pos.x > self.max_x {self.max_x = pos.x}
        if pos.y < self.min_y {self.min_y = pos.y}
        if pos.y > self.max_y {self.max_y = pos.y}
    }
    fn new() -> Self {
        Rectangle{
            min_x: i32::MAX,
            min_y: i32::MAX,
            max_x: i32::MIN,
            max_y: i32::MIN,
        }
    }
    fn width(&self) -> i32 {
        self.max_x - self.min_x + 1
    }
    fn height(&self) -> i32 {
        self.max_y - self.min_y + 1
    }
}

// fn print_map( rec: &Rectangle, map: &HashMap<IVec2,char> ) {
//     for y in rec.min_y-1 ..= rec.max_y+1 {
//         for x in  rec.min_x-1 ..= rec.max_x+1 {
//             if let Some(c) = map.get(&IVec2{x,y}) {
//                 print!("{c}")
//             } else {
//                 print!(".")
//             }
//         }
//         println!()
//     }
// }

fn get_elve_move_to_char(e:ElveMove) -> char {
    match e {
        ElveMove::N => 'N',
        ElveMove::S => 'S',
        ElveMove::W => 'W',
        ElveMove::E => 'E',
        _ => '?'
    }
}

fn get_elve_stuck_to_char(e:ElveMove) -> char {
    match e {
        ElveMove::N => '^',
        ElveMove::S => 'v',
        ElveMove::W => '>',
        ElveMove::E => '<',
        _ => '?'
    }
}

fn main() -> Result<()> { 
 
    let (source,filename) = testing::get_data(file!())?;

    let mut rec = Rectangle::new();

    let mut elves_pos = Vec::new();
    let mut elves_pos_set = HashMap::new();
    for (y, map_line) in source.split('\n').enumerate() { 
        for (x,c) in  map_line.chars().enumerate() {
            match c {
                '.' => continue,
                '#' =>{
                    let pos = IVec2::new(x as i32, y as i32);
                    rec.extend(&pos);
                    elves_pos.push((pos,0));
                    elves_pos_set.insert(pos,'0');
                },
                _=> {
                    return Err(MyBad::new(filename, source, (y+1,x+1,1), format!("Invalid character {c:?}")).into())
                }
            }
        }
    }

    // print_map(&rec,&elves_pos_set);
    let mut answer_a = - (elves_pos.len() as i32);

    for round in 0..999999 {
        let mut moved_to_pos = Vec::new();
        let mut new_moves_pos = HashMap::new();

        for v in elves_pos_set.values_mut() {
            *v = '0';
        }

        elves_pos.extract_if( |(elve_pos,elve_dir_offset)| { 
            let mut ocuppied = ElveMove::none();
            for (check_pos_diff,checked_direction) in NEIGHBOURS {
                let new_pos = elve_pos.add(check_pos_diff);
                if elves_pos_set.get(&new_pos).is_some() {
                    ocuppied |= checked_direction;
                }
            }
            if ocuppied.is_none() {
                elves_pos_set.insert(*elve_pos, '#');
                return false
            }
            for (check_direction, m, elve_pos_diff) in 
                AVAILBE_MOVE.iter().skip(round%4).chain(AVAILBE_MOVE.iter().take(round%4))
                // AVAILBE_MOVE.iter().skip(*elve_dir_offset).chain(AVAILBE_MOVE.iter().take(*elve_dir_offset)) 
                {
                if ocuppied & *check_direction == 0 {
                    let pos = elve_pos.add(*elve_pos_diff);
                    match new_moves_pos.entry(pos) {
                        Occupied(mut o) => {
                            o.insert( o.get() + 1 );
                            elves_pos_set.insert(*elve_pos, get_elve_stuck_to_char(*m));
                            return false;
                        },
                        Vacant(v) => {
                            v.insert(1);
                            moved_to_pos.push((pos, *elve_pos, *elve_dir_offset, *m));
                            return true;
                        },
                    }
                }
                elves_pos_set.insert(*elve_pos, 'X');
            }    
            false
        });

        let mut noone_moved = true;
        for (new_pos,old_pos, direction_offset, m) in moved_to_pos {
            if let Some(1) = new_moves_pos.get(&new_pos) {
                elves_pos_set.remove(&old_pos);
                elves_pos_set.insert(new_pos,get_elve_move_to_char(m));
                elves_pos.push((new_pos,(direction_offset+1)%4));
                rec.extend(&new_pos);
                noone_moved = false;
            } else {
                elves_pos_set.insert(old_pos,get_elve_stuck_to_char(m));
                elves_pos.push((old_pos,direction_offset));
            }
        }
        println!("{round:-^4}-{}-{:-^3}-", get_elve_move_to_char(AVAILBE_MOVE[round%4].1), elves_pos_set.len());

        if round == 9 {
            for (pos,_) in &elves_pos {
                rec.extend(pos);
            }
            answer_a += rec.width() * rec.height();
        }

        if noone_moved {
            println!("answer_a:{answer_a}");
            println!("answer_b:{}",round+1);
            break;
        }
        // print_map(&rec, &elves_pos_set);
    }

    Ok(())
}

