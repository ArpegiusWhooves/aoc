

#![feature(extract_if)]

use std::collections::HashSet;

use miette::Result;

use glam::IVec2;
use ndarray::Array2;
use testing::MyBad;
use bitmask_enum::bitmask;

#[bitmask(u8)]
pub enum MapDirection {
    NE,N,NW,
    E,   W,
    SE,S,SW,
}

#[bitmask(u8)]
pub enum MapBoundaries {
    Top = 0x2,
    Left = 0x8,
    Right = 0x10,
    Bottom = 0x40,

    TopLeft = 0x2 | 0x8,
    TopRight = 0x2 | 0x10,
    BottomLeft = 0x40 | 0x8,
    BottomRight = 0x40 | 0x10,
}

const NEIGHBOURS: [(MapDirection, IVec2);4] = [
    (MapDirection::N,IVec2::NEG_Y),
    (MapDirection::S,IVec2::Y),
    (MapDirection::W,IVec2::NEG_X),
    (MapDirection::E,IVec2::X),
];

fn get_elve_stuck_to_char(e:MapDirection) -> char {
    match e {
        MapDirection::N => '^',
        MapDirection::S => 'v',
        MapDirection::W => '<',
        MapDirection::E => '>',
        _ => 
            match e.bits().count_ones() {
                0 => '.',
                2 => '2',
                3 => '3',
                4 => '4',
                _ => '?'
            }
    }
}

#[derive(Debug)]
pub struct Map {
    board: Array2<MapDirection>, //Dim<[usize; 2]>>,
}

type MapPos = (usize,usize);

impl  Map {
    
    pub fn new() -> Self {
        Self { board: Array2::from_elem((120,25),MapDirection::none()) }
    }

    pub fn print(&self) {
        let s = self.board.dim();
        for y in  0 .. s.1 {
            for x in 0 .. s.0 {
                    print!( "{}", get_elve_stuck_to_char(self.board[(x,y)]) );
            }
            println!();
        }
    }

    pub fn width(&self) -> usize {
        self.board.dim().0
    }

    pub fn height(&self) -> usize {
        self.board.dim().1
    }

    fn get_pos(&self, p: MapPos) -> MapDirection {
        self.board[ p ]
    }

    pub fn check_boundaries(&self, p: MapPos) -> MapBoundaries {
        let s = self.board.dim();
        let mut v = MapBoundaries::none();
        if p.0 == 0 { v = MapBoundaries::Left }
        if p.0 == s.0 - 1 { v = MapBoundaries::Right }
        if p.1 == 0 { v = MapBoundaries::Top }
        if p.1 == s.1 - 1 { v = MapBoundaries::Bottom }
        v
    }

    pub fn left(&self, p:MapPos) -> Option<MapPos> {
        if p.0 != 0 {
            Some((p.0-1,p.1))
        } else {None}
    }
    pub fn up(&self, p:MapPos) -> Option<MapPos> {
        if p.1 != 0 {
            Some((p.0,p.1-1))
        } else {None}
    }
    pub fn right(&self,p: MapPos ) -> Option<MapPos> {
        if p.0 != self.width()-1 {
            Some((p.0+1,p.1))
        } else {None}
    }
    pub fn down(&self,p: MapPos ) -> Option<MapPos> {
        if p.1 != self.height()-1 {
            Some((p.0,p.1+1))
        } else {None}
    }

    fn evaluate_pos( &self, pos: MapPos ) -> MapDirection {
        let s = self.board.dim();
        let mut v = MapDirection::none();
        for (dir,of) in NEIGHBOURS {
            let mut nx = pos.0 as i32 - of.x; // substract as oposing moves
            if nx < 0 {
                nx += s.0 as i32;
            } else if nx >= s.0 as i32 {
                nx -= s.0 as i32;
            }
            let mut ny = pos.1 as i32 - of.y; // substract as oposing moves
            if ny < 0 {
                ny += s.1 as i32;
            } else if ny >= s.1 as i32 {
                ny -= s.1 as i32;
            }
            v |= self.board[(nx as usize ,ny as usize)] & dir;
        }
        v
    }
    fn evaluate(&mut self) {
        let s = self.board.dim();
        let mut new_board = Array2::from_elem(s,MapDirection::none());
        for (pos,value) in new_board.indexed_iter_mut() {
            *value = self.evaluate_pos(pos)
        }
        self.board = new_board
    }

}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

fn main() -> Result<()> { 
 
    let (source,filename) = testing::get_data(file!())?;

    let mut lines = source.split('\n');
    lines.next();

    let mut map = Map::new();

    // map.board[(0,0)] = MapDirection::none();

    for (y, map_line) in lines.enumerate() { 
        let mut chars_line= map_line.chars();
        dbg!(chars_line.next());
        for (x,c) in  chars_line.enumerate() {
            match c {
                '.' => continue,
                '#' =>{
                    break;
                },
                '<' =>{
                    map.board[(x,y)] = MapDirection::W;
                },
                '^' =>{
                    map.board[(x,y)] = MapDirection::N;
                },
                '>' =>{
                    map.board[(x,y)] = MapDirection::E;
                },
                'v' =>{
                    map.board[(x,y)] = MapDirection::S;
                },
                _=> {
                    return Err(MyBad::new(filename, source, (y+1,x+1,1), format!("Invalid character {c:?}")).into())
                }
            }
        }
    }
    
    let mut time = 0usize;
    loop {
        map.evaluate();
        time += 1;
        if map.get_pos((0,0)).is_none() {break}
    }

    let mut state = vec![(0usize,0usize)];

    let mut first_end = false;
    let mut second_start = false;

    loop {
        map.evaluate();
        time += 1;
        let mut new_state = HashSet::new();
        for pos in state { 
            if map.get_pos(pos).is_none() {
                new_state.insert(pos);
            }
            if let Some(new_pos) = map.left(pos) {
                if map.get_pos(new_pos).is_none() {
                    new_state.insert(new_pos);
                }
            }
            if let Some(new_pos) = map.up(pos) {
                if map.get_pos(new_pos).is_none() {
                    new_state.insert(new_pos);
                }
            } else if first_end && !second_start && map.left(pos).is_none() {
                loop {
                    map.evaluate();
                    time += 1;
                    if map.get_pos(pos).is_none() {break}
                }
                new_state.clear();
                new_state.insert(pos);
                second_start=true;
                break;
            }
            if let Some(new_pos) = map.right(pos) {
                if map.get_pos(new_pos).is_none() {
                    new_state.insert(new_pos);
                }
            }
            if let Some(new_pos) = map.down(pos) {
                if map.get_pos(new_pos).is_none() {
                    new_state.insert(new_pos);
                }
            } else if map.right(pos).is_none() {
                if !first_end {
                    loop {
                        map.evaluate();
                        time += 1;
                        if map.get_pos(pos).is_none() {break}
                    }
                    new_state.clear();
                    new_state.insert(pos);
                    first_end=true;
                    break;
                } else if second_start {
                    println!("Time:{time}");
                    return Ok(())
                }
            }
        }

        println!( "Time:{time}, ns:{}", new_state.len() );

        state = Vec::from_iter(new_state.into_iter());
    } 
}

