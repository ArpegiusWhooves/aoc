


use miette::Result;
use testing::get_data; 
 
fn main() -> Result<()> { 

    let (body,_) = get_data(file!())?;
 

    let mut content: Vec<&[u8]> = body.split('\n').map(|x|x.as_bytes()).collect();
    content.pop();

    let width = content[0].len();
    let height = content.len();

    let mut answer_a = 2*width + 2* height - 4;

    for x in 1 .. width - 1 {
        for y in  1 .. height - 1 {
            let t = content[y][x];
            if content[y][0 .. x].iter().all(|x| x < &t ) {
                answer_a += 1;
                continue;
            }
            if content[y][x+1 .. width].iter().all(|x| x < &t ) {
                answer_a += 1;
                continue;
            }
            if content[0..y].iter().all(|row| row[x] < t ) {
                answer_a += 1;
                continue;
            }
            if content[y+1..height].iter().all(|row| row[x] < t ) {
                answer_a += 1;
                continue;
            }
        }
    }

    dbg!(answer_a);
    Ok(())
}
