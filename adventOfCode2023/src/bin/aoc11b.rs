
 
use miette::Result;

use testing::get_data; 


fn aoc((body, _file): (String, String)) -> Result<i64> {
    let mut r = 0; 

    let width = body.find('\n').unwrap();
    let height = body.len() / (width+1);

    let mut columns = std::iter::repeat(1000000-1).take(width).collect::<Vec<_>>();
    let mut rows = std::iter::repeat(1000000-1).take(height).collect::<Vec<_>>();
    let mut galaxies = Vec::new();

    for (y,line) in body.split('\n').enumerate() {
        for (x,c) in line.char_indices() {
            if c == '#' {
                columns[x] = 0;
                rows[y] = 0;
                galaxies.push((x,y));
                // println!("{} - {x},{y}",galaxies.len());
            }
        }
    }
    let mut cm = 0;
    for c in &mut columns {
        cm += *c;
        *c = cm;
    }
    let mut rm = 0;
    for r in &mut rows {
        rm += *r;
        *r = rm;
    } 
    for g in &mut galaxies {
        g.0 += columns[g.0];
        g.1 += rows[g.1];
    }
    for i in 0 .. galaxies.len() {
        for j in i+1 .. galaxies.len() {
            let dx = ( galaxies[i].0 as i64 - galaxies[j].0 as i64 ).abs();
            let dy = ( galaxies[i].1 as i64 - galaxies[j].1 as i64 ).abs();
            let d = dx + dy;
            println!("{}-{} = {d}",i+1,j+1);
            r += d;
        }
    }


    Ok(r)
}

#[test]
fn test1() {
    assert_eq!(
        aoc((
            "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors allowed"),
        8410
    );
}

fn main() -> Result<()> {
    println!("{}", aoc(get_data(file!())?)?);
    Ok(())
}
