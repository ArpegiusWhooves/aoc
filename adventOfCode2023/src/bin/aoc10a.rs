
 
use miette::Result;

use num::Integer;
use testing::get_data; 

use testing::{MapTrait,Pos,Direction};
use array2d::Array2D;


fn move_on_board(p:Pos,d:Direction, map:&Array2D<u8>) -> Option<(Pos,Direction)> {
    use Direction::*;
    Some(match map.get_pos(&p) {
        b'|' => match d {
            Up => (p.up()?,Up),
            Right => todo!(),
            Down => (p.down(map)?,Down),
            Left => todo!(),
        },
        b'-' => match d {
            Up => todo!(),
            Right => (p.right(map)?,Right),
            Down => todo!(),
            Left => (p.left()?,Left),
        },
        b'7' => match d {
            Up => (p.left()?,Left),
            Right => (p.down(map)?,Down),
            Down => todo!(),
            Left => todo!(),
        },
        b'F' => match d {
            Up => (p.right(map)?,Right),
            Right => todo!(),
            Down => todo!(),
            Left => (p.down(map)?,Down),
        },
        b'J' => match d {
            Up => todo!(),
            Right => (p.up()?,Up),
            Down => (p.left()?,Left),
            Left => todo!(),
        },
        b'L' => match d {
            Up => todo!(),
            Right => todo!(),
            Down => (p.right(map)?,Right),
            Left => (p.up()?,Up),
        }, 
        _=> panic!("End Of a road at {:?}",p)
    })
}

fn aoc((body, _file): (String, String)) -> Result<i32> {
    let mut r = 0;
    dbg!(&body);
    let width = body.find('\n').unwrap();
    let height = body.len() / (width+1);

    let start = Pos::from_yx(body.find('S').unwrap().div_mod_floor(&(width+1)));

    let mut board = Array2D::<u8>::from_row_major(
            body.as_bytes(),
            height,width+1
    ).unwrap();
 
    println!("{:?} = {}", start, board.get_pos(&start) as char);

    board.print();

    let mut pos = Vec::new();

    use Direction::*;
    if let Some(p) = start.right(&board) {
        if [b'-',b'7',b'J'].contains(&board.get_pos(&p)) {
            pos.push((p,Right));
        }
    }
    if let Some(p) = start.down(&board) {
        if [b'|',b'J',b'L'].contains(&board.get_pos(&p)) {
            pos.push((p,Down));
        }
    }
    if let Some(p) = start.up() {
        if [b'|',b'7',b'F'].contains(&board.get_pos(&p)) {
            pos.push((p,Up));
        }
    }
    if let Some(p) = start.left() {
        if [b'-',b'L',b'F'].contains(&board.get_pos(&p)) {
            pos.push((p,Left));
        }
    }
    assert_eq!(pos.len(),2);

    while pos[0].0 != pos[1].0 {
        r+=1;
        for old_p in &mut pos {
            let new_p = move_on_board(old_p.0.clone(),old_p.1,&board)
                .expect("Pipe comes over the board");
            println!("{:?} ={}> {:?}",old_p,board.get_pos(&old_p.0) as char,new_p);

            board.set_pos(&old_p.0, b'*');
            *old_p = new_p;
        }
    }

    board.set_pos(&&start, b'*');
    board.set_pos(&pos[0].0, b'*');
    board.print();

    Ok(r+1)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            "\
.....
.S-7.
.|.|.
.L-J.
.....
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors allowed"),
        4
    );
    assert_eq!(
        aoc((
            "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors allowed"),
        8
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);
    Ok(())
}
