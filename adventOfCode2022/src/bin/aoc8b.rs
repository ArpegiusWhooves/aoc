

  

use miette::Result;
use testing::get_data; 

fn main() -> Result<()> { 
    let (body,_) = get_data(file!())?;
 

    let mut content: Vec<Vec<u8>> = body.split('\n').map(|x|{
        x.bytes().map(|b|b-b'0').collect()
    }).collect();

    content.pop();

    let width = content[0].len();
    let height = content.len();

    let mut answer_b = 0;

    for x in 1 .. width - 1 {
        for y in  1 .. height - 1 {
            let ct = content[y][x];
            let mut left = 0;
            let mut right = 0;
            let mut top = 0;
            let mut bottom = 0;
            for t in content[y][0 .. x].iter().rev() {
                left += 1;
                if *t >= ct {break}
            } 
            for t in content[y][(x+1) .. width].iter()  {
                right += 1;
                if *t >= ct {break}
            } 
            for t in content[0..y].iter().rev().map(|row| row[x]) {
                top += 1;
                if t >= ct {break}
            } 
            for t in content[(y+1)..height].iter().map(|row| row[x]) {
                bottom += 1;
                if t >= ct {break}
            } 

            let score = left * right * top * bottom;


            if score > answer_b {
                dbg!(x,y,ct,left, right, top, bottom,score); 

                answer_b = score;
            }
        }
    }

    dbg!(answer_b);
    Ok(())
}
