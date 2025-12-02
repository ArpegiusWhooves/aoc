
 
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
            Right => panic!(),
            Down => (p.down(map)?,Down),
            Left => panic!(),
        },
        b'-' => match d {
            Up => panic!(),
            Right => (p.right(map)?,Right),
            Down => panic!(),
            Left => (p.left()?,Left),
        },
        b'7' => match d {
            Up => (p.left()?,Left),
            Right => (p.down(map)?,Down),
            Down => panic!(),
            Left => panic!(),
        },
        b'F' => match d {
            Up => (p.right(map)?,Right),
            Right => panic!(),
            Down => panic!(),
            Left => (p.down(map)?,Down),
        },
        b'J' => match d {
            Up => panic!(),
            Right => (p.up()?,Up),
            Down => (p.left()?,Left),
            Left => panic!(),
        },
        b'L' => match d {
            Up => panic!(),
            Right => panic!(),
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

    let mut pipes = Array2D::filled_with(b'.', height,width+1);

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

    pipes.set_pos(&start, match (pos[0].1,pos[1].1) { 
        (Up, Right) => b'L',
        (Up, Down) => b'|',
        (Up, Left) => b'J',
        (Right, Up) => b'L', 
        (Right, Down) => b'F',
        (Right, Left) => b'-',
        (Down, Up) => b'|',
        (Down, Right) => b'F', 
        (Down, Left) => b'7',
        (Left, Up) => b'J',
        (Left, Right) => b'-',
        (Left, Down) => b'7',
        _ => panic!(),
    });


    while pos[0].0 != pos[1].0 {
        for old_p in &mut pos {
            let new_p = move_on_board(old_p.0.clone(),old_p.1,&board)
                .expect("Pipe comes over the board");
            println!("{:?} ={}> {:?}",old_p,board.get_pos(&old_p.0) as char,new_p);

            pipes.set_pos(&old_p.0, board.get_pos(&old_p.0));
            board.set_pos(&old_p.0, b'*');
            *old_p = new_p;
        }
    }

    pipes.set_pos(&pos[0].0, match (pos[0].1,pos[1].1) { 
        (Up, Right) => b'7',
        (Up, Down) => b'|',
        (Up, Left) => b'F',
        (Right, Up) => b'7', 
        (Right, Down) => b'J',
        (Right, Left) => b'-',
        (Down, Up) => b'|',
        (Down, Right) => b'J', 
        (Down, Left) => b'L',
        (Left, Up) => b'F',
        (Left, Right) => b'-',
        (Left, Down) => b'L',
        _ => panic!(),
    });

    board.set_pos(&&start, b'*');
    board.set_pos(&pos[0].0, b'*');
    
    for y in 0.. pipes.height() {
        let mut inside = false; 
        for x in 0.. pipes.width() {
            let a= pipes[(y,x)];
            if a == b'F' {
                inside = !inside;
            } else 
            if a == b'7' {
                inside = !inside;
            } else 
            if a == b'|' {
                inside = !inside; 
            } else 
            if a == b'.' { 
                if inside {
                    r += 1;
                }
            }
            print!("{}", a as char);
        }
        println!(" ={r}");
    }
    

    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors allowed"),
        4
    );
}

#[test]
fn test2() {
    assert_eq!(
        aoc((
            "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"
            .to_owned(),
            "test2".to_owned()
        ))
        .expect("no errors allowed"),
        8
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);
    Ok(())
}
