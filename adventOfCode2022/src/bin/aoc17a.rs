



use miette::{Result, miette };

 
use testing::Pos;
use testing::MapTrait;
use testing::MapRowMajor as Map;

#[derive(Debug,Clone,Copy)]
enum Shape {
    Horisontal,
    Cross,
    Corner,
    Vertical,
    Rectangle
}

#[derive(Debug,Clone)]
struct Rock {
    shape: Shape,
    pos: Pos
}

impl Rock {
    
    fn ocupied_pos(&self) -> Box<dyn Iterator<Item=Pos>> {
        let pos = self.pos.clone();
        match self.shape {
            Shape::Horisontal => Box::new((0..4).map(move |d| pos.moved_right(d) )),
            Shape::Cross => Box::new([
                pos.moved_right(1),
                pos.moved_top(1),
                pos.moved_top_right(1,1),
                pos.moved_top_right(1,2),
                pos.moved_top_right(2,1),
                ].into_iter()),
            Shape::Corner => Box::new((0..5).map(move |d|
                if d > 2 { pos.moved_top_right(2,d-2) } else { pos.moved_right(d) })),
            Shape::Vertical => Box::new((0..4).map(move |d| pos.moved_top(d) )),
            Shape::Rectangle => Box::new([
                pos.clone(),
                pos.moved_right(1),
                pos.moved_top(1),
                pos.moved_top_right(1,1),
                ].into_iter()),
        }
        
    }

    fn width(&self) -> usize {
        match self.shape {
            Shape::Horisontal => 4,
            Shape::Cross => 3,
            Shape::Corner => 3,
            Shape::Vertical => 1,
            Shape::Rectangle => 2,
        }
    }

}


fn main() -> Result<()> { 
 
    let (body,_filename) = testing::get_data(file!())?;
 
    let mut answer_a = 0; 
 
    let line = body.split('\n').next().expect("No first line.");
 
    const MAP_WIDTH: usize = 7;

    let mut map = Map::empty();
    // for _i in 0 .. 3 {
    //     map.add_row([0u8;MAP_WIDTH]);
    // }

    use Shape::*;
    let mut shapes = [ 
            Horisontal,
            Cross,
            Corner,
            Vertical,
            Rectangle 
        ].into_iter().cycle();

    let mut falling_rock = None;

    let mut rock_count = 1;

    for c in line.chars().cycle() {

        let rock = falling_rock.get_or_insert_with( || Rock{ 
                shape: shapes.next().unwrap(),
                pos: Pos{ x: 2, y: map.height()+3 }
            }
        ); 

        let moved = if c == '<' {
            rock.pos.move_left(1)
        } else if c == '>' {
            rock.pos.checked_move_right(1,MAP_WIDTH - rock.width() + 1)
        } else {
            return Err(miette!( "Unknown characted '{}'", c ));
        };

        if moved && rock.ocupied_pos().any(|p| map.get(&p) > 0 ) {
            if c == '<' {
                rock.pos.move_right(1);
            } else if c == '>' {
                rock.pos.move_left(1);
            } else { panic!(); }
        }
        if rock.pos.move_down(1)  {
            if rock.ocupied_pos().all(|p| map.get(&p) == 0 ) {
                continue;
            }
            rock.pos.move_up(1);
        }
        for pos in rock.ocupied_pos() {
            while map.height() <= pos.y {
                map.add_row([0;MAP_WIDTH]);
                answer_a = map.height();
            }
            map.set(&pos, 64);
        }

        println!("{rock_count:-^7}");
        // map.print();

        falling_rock.take();
        if rock_count == 2022 {
            break;
        }
        rock_count += 1;
    }
 
    dbg!(answer_a); 
 

    Ok(())
}

