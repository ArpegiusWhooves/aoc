use miette::{Context, IntoDiagnostic, Result};
use nom::FindSubstring;
use testing::get_data;


use testing::{MapTrait,Pos,Direction};
use array2d::Array2D;

struct MirrorTest {
    left: Pos,
    right: Pos,
    mid_offset: i32,
}

impl MirrorTest {
    fn new<T:MapTrait>(start:usize, end:usize) -> Self {
        Self {
            left: Pos{ x: todo!(), y: todo!() },
            right: Pos{ x: todo!(), y: todo!() },
            mid_offset: (end/2) as i32,
        }
    }

    fn check<T:MapTrait>(&mut self, map: &T) -> bool {
        let left = map.get_pos(&self.left);
        let right = map.get_pos(&self.right);
        



        false
    }
}


fn aoc((body, file): (String, String)) -> Result<u32> {
    let mut r = 0;
    let mut offset = 0;
    loop {
        let block = &body.as_bytes()[offset..];
        let width = block.find_substring("\n").unwrap();
        let size = block.find_substring("\n\n").unwrap();
        let height = (size+1) / (width+1);
        assert_eq!((width+1)*height, (size+1));

        let board = Array2D::<u8>::from_row_major(
            block,
            height,width+1
        ).unwrap();



        offset += size + 2;
        break;
    }

    for (no, block) in body.split("\n\n").enumerate() {
        let width = block.find('\n').unwrap();
        let height = (block.len()+1) / (width+1);
        dbg!(no,width,height);

        let mut board = Array2D::<u8>::from_row_major(
            block.as_bytes(),
            height,width+1
        ).unwrap();

    }

    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors"),
        405
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);

    Ok(())
}
