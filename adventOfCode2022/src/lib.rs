
mod aoc_data; 

pub use aoc_data::get_data;
mod my_parser_error;
pub use my_parser_error::MyBad;

pub mod aoc7l;
pub mod aoc11l;
pub mod aoc19l;
pub mod aoc21l;
 
pub trait ParserResultWithCode<T> {
    fn error_with_source(self, name: &str, source: &str) -> Result<T,my_parser_error::MyBad>;
}

impl<T, E > ParserResultWithCode<T> for Result<T, E> 
where my_parser_error::MyBad: From<E>  {
    fn error_with_source(self, name: &str, source: &str ) -> Result<T,my_parser_error::MyBad> {
        self.map_err(|e| 
            my_parser_error::MyBad::from(e)
            .with_source(name,source) 
        )
    }
}

#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos{ 
    pub x:usize, 
    pub y:usize 
}

pub trait  MapTrait {
    fn get(&self, pos: &Pos) -> u8;
    fn set(&mut self, pos: &Pos, e: u8) -> bool;

    fn height(&self) -> usize;

    fn width(&self) -> usize;

    fn empty() -> Self;

    fn add_column<L>(&mut self, column: L) 
        where L: Into<Vec<u8>> + IntoIterator<Item=u8>;

    fn add_row<L>(&mut self, row: L) 
        where L: Into<Vec<u8>> + IntoIterator<Item=u8>;
    
    fn print(&self);
    
}


#[derive(Clone,Debug,Default)]
pub  struct  MapColumnMajor(Vec<Vec<u8>>);

impl MapTrait for MapColumnMajor {
    fn get(&self, pos: &Pos) -> u8 {
        self.0[pos.x][pos.y]
    }
    fn set(&mut self, pos: &Pos, e: u8) -> bool {
        let Some(row) = self.0.get_mut(pos.x) else {
            return false;
        };
        let Some(me) = row.get_mut(pos.y) else {
            return false;
        };
        *me = e;
        true
    }
    fn height(&self) -> usize {
        self.0[0].len()
    }

    fn width(&self) -> usize {
        self.0.len() 
    }

    fn empty() -> MapColumnMajor {
        MapColumnMajor(Vec::new())
    }

    fn add_column<L>(&mut self, column: L) 
        where L: Into<Vec<u8>> {
        self.0.push(column.into());
    }

    fn add_row<L>(&mut self, row: L) 
        where L: IntoIterator<Item=u8>{
        for (column,value) in self.0.iter_mut().zip(row.into_iter()) {
            column.push(value);
        }
    }

    fn print(&self) {
        for row in &self.0 {
            for e in row {
                print!("{}",(*e) as char);
            }
            println!();
        }
    }
}


#[derive(Clone,Debug,Default)]
pub  struct  MapRowMajor(Vec<Vec<u8>>);

impl MapTrait for MapRowMajor {

    fn get(&self, pos: &Pos) -> u8 {
        let Some(row) = self.0.get(pos.y) else {
            return 0;
        };
        let Some(e) = row.get(pos.x) else {
            return 0;
        };
        *e
    }

    fn set(&mut self, pos: &Pos, e: u8) -> bool {
        let Some(row) = self.0.get_mut(pos.y) else {
            return false;
        };
        let Some(me) = row.get_mut(pos.x) else {
            return false;
        };
        *me = e;
        true
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn empty() -> MapRowMajor {
        MapRowMajor(Vec::new())
    }

    fn add_column<L>(&mut self, column: L)
        where L: IntoIterator<Item=u8> {
        for (column,value) in self.0.iter_mut().zip(column.into_iter()) {
            column.push(value);
        }
    }

    fn add_row<L>(&mut self, row: L)
        where L: Into<Vec<u8>> {
        self.0.push( row.into() );
    }

    fn print(&self) {
        for row in self.0.iter().rev() {
            for e in row {
                let c = match *e {
                    0 => '.',
                    1 ..= 10 => (*e + b'0') as char,
                    11 ..= 36 => (*e + b'a') as char,
                    37 ..= 63 => (*e + b'A') as char,
                    64 => '#',
                    _ => '?'
                };
                print!("{c}");
            }
            println!();
        }
    }
}

impl Pos {
    pub fn distance(&self, other: &Pos) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }
    pub fn moved_top_right(&self, dx:usize, dy:usize) -> Pos {
        Pos{x:self.x+dx, y:self.y+dy}
    }
    pub fn moved_right(&self, dx:usize) -> Pos {
        Pos{x:self.x+dx, y:self.y}
    }
    pub fn moved_top(&self, dy:usize) -> Pos {
        Pos{x:self.x, y:self.y+dy}
    }
    pub fn move_xy(&mut self, dx:isize, dy:isize) -> bool {
        match (self.x.checked_add_signed(dx),self.y.checked_add_signed(dy)) {
            (Some(x),Some(y)) => {
                self.x = x;
                self.y = y;
                true
            }
            _ => false
        }
    }
    pub fn move_left(&mut self, dx:usize) -> bool {
        if self.x < dx {
            false
        }  else {
            self.x -= dx;
            true
        } 
    }
    pub fn move_down(&mut self, dy:usize) -> bool {
        if self.y < dy {
            false
        }  else {
            self.y -= dy;
            true
        } 
    }
    pub fn move_right(&mut self, dx:usize) {
        self.x += dx;
    }
    pub fn checked_move_right(&mut self, dx:usize, bound_x: usize ) -> bool {
        let nx = self.x + dx;
        if nx < bound_x {
            self.x = nx;
            true
        }  else {
            false
        } 
    }
    pub fn move_up(&mut self, dy:usize) {
        self.y += dy;
    }
    pub fn checked_move_up(&mut self, dy:usize, bound_y: usize) -> bool {
        let ny = self.y + dy;
        if ny < bound_y {
            self.y = ny;
            true
        }  else {
            false
        } 
    }
    pub fn move_x(&mut self, dx:isize) -> bool {
        if let Some(x) = self.x.checked_add_signed(dx) {
            self.x=x;
            true
        } else {false}
    }
    pub fn move_y(&mut self, dy:isize) -> bool {
        if let Some(x) = self.x.checked_add_signed(dy) {
            self.x=x;
            true
        } else {false}
    }
    pub fn up(&self) -> Option<Self> { if self.y > 0 { Some(Self{x:self.x,y:self.y-1}) } else {None} }
    pub fn down<M:MapTrait>(&self, map: &M) -> Option<Self> { if self.y+1 < map.height() { Some(Self{x:self.x,y:self.y+1}) } else {None} }
    pub fn left(&self) -> Option<Self> { if self.x > 0 { Some(Self{x:self.x-1,y:self.y}) } else {None} }
    pub fn right<M:MapTrait>(&self, map: &M) -> Option<Self> { if self.x+1 < map.width() { Some(Self{x:self.x+1,y:self.y}) } else {None} }
    pub fn neighbors<M:MapTrait>(&self, map: &M) -> Vec<Pos> {
        let mut result = Vec::new();
        if let Some(pos) = self.right(map) {
            result.push(pos);
        }
        if let Some(pos) = self.down(map) {
            result.push(pos);
        }
        if let Some(pos) = self.up() {
            result.push(pos);
        }
        if let Some(pos) = self.left() {
            result.push(pos);
        }
        result
    }
}


use std::collections::{HashMap, hash_map::Entry::{Occupied, Vacant}};

#[derive(Debug,Default)]
pub struct AutoIndex<'a> (HashMap<&'a str, usize>);

impl<'a> AutoIndex<'a> {
    pub fn get(&mut self, tag: &'a str) -> usize {
        let r = self.0.len();
        match self.0.entry(tag)  {
            Occupied(o) => *o.get(),
            Vacant(v) => {v.insert(r);r},
        }
    }

}

