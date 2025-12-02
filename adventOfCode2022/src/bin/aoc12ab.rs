

  
use miette::Result;
use testing::get_data;

#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos{ x:usize, y:usize }

struct  Map(Vec<Vec<u8>>);

impl Map {
    fn get_altitude(&self, pos: &Pos) -> u8 {
        self.0[pos.y][pos.x]
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    pub(crate) fn new() -> Map {
        Map(Vec::new())
    }

    fn add_row(&mut self, line: Vec<u8>) {
        self.0.push( line );
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

impl Pos {
  fn distance(&self, other: &Pos) -> u32 {
    (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
  }
  fn up(&self) -> Option<Self> { if self.y > 0 { Some(Self{x:self.x,y:self.y-1}) } else {None} }
  fn down(&self, map: &Map) -> Option<Self> { if self.y+1 < map.height() { Some(Self{x:self.x,y:self.y+1}) } else {None} }
  fn left(&self) -> Option<Self> { if self.x > 0 { Some(Self{x:self.x-1,y:self.y}) } else {None} }
  fn right(&self, map: &Map) -> Option<Self> { if self.x+1 < map.width() { Some(Self{x:self.x+1,y:self.y}) } else {None} }

  fn successors(&self, map: &Map) -> Vec<(Pos, u32)> {
    let cur_height = map.get_altitude(self);

    let mut result = Vec::new();

    if let Some(pos) = self.right(map) {
        if map.get_altitude(&pos) - 1 <= cur_height {
            result.push( (pos, 1) );
        }
    }
    if let Some(pos) = self.down(map) {
        if map.get_altitude(&pos) - 1 <= cur_height {
            result.push( (pos, 1) );
        }
    }
    if let Some(pos) = self.up() {
        if map.get_altitude(&pos) - 1 <= cur_height {
            result.push( (pos, 1) );
        }
    }
    if let Some(pos) = self.left() {
        if map.get_altitude(&pos) - 1 <= cur_height {
            result.push( (pos, 1) );
        }
    }
    result
  }
}

fn main() -> Result<()> { 
    let (body,_) = get_data(file!())?; 
 
    let mut answer_b = 99999999;
    
    let mut map: Map = Map::new();

    let  mut start = Pos::default();
    let  mut starts: Vec<Pos> = Vec::new();
    let  mut end = Pos::default();


    for (y,line) in body.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut row = Vec::new();
        
        for (x,c) in line.bytes().enumerate() {
            match c {
                b'a' ..= b'z' => { if c==b'a' {starts.push(Pos{x,y})}; row.push(c)},
                b'S' => { start = Pos{x,y}; row.push(b'a')}, 
                b'E' => { end = Pos{x,y}; row.push(b'z')},
                _ => panic!("wrong data at {x},{y}")
            }
        }
        map.add_row(row);
    }
    dbg!(&start,&end);
    map.print();

    use pathfinding::prelude::astar;

    let Some(result) = astar(&start, 
            |p| p.successors(&map),
            |p| p.distance(&end),
            |p| *p == end) else {
                panic!("No paths!")
            };

    let answer_a = result.1;

    for start in starts {
        print!("{:?}", &start);

        let Some(result) = astar(&start, 
            |p| p.successors(&map),
            |p| p.distance(&end),
            |p| *p == end) else {
                println!(" No paths!");
                continue;
            };

        println!("{}",result.1);

        if result.1 < answer_b {
            answer_b = result.1;
        }

    }
 
    dbg!(answer_a);
    dbg!(answer_b);
 

    Ok(())
}

