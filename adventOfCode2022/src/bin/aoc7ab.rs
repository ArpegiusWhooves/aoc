

 


use miette::{Result, IntoDiagnostic, miette, Context };
use testing::get_data;


#[derive(Default,Debug)]
struct  Directory {
    name: String,
    size: usize,
    content: Vec< (usize,String) >
}

fn main() -> Result<()> { 
    let (body,_) = get_data(file!())?;
 
    let mut stack = Vec::new();
    let mut dir_list = Vec::new();

    let mut current  = Directory::default();
  
    let mut answer_a = 0;
    let mut answer_b = 0;

    for (line_no,line) in body.split('\n').enumerate() {
        if line.is_empty() {
            continue
        }
        if line.starts_with( "$ cd " ) {
            let name = line.chars().skip(5).collect();
            if name != ".." {
                if name != "/" {
                    stack.push( current );
                    current = Directory::default();
                } else {
                    assert!(stack.is_empty());
                }
                current.name = name;
            } else {

                if current.size <= 100000 {
                    answer_a += current.size;
                    dbg!(&current.name);
                }

                dir_list.push(current.size);

                let mut top = stack.pop().ok_or(miette!("Too much cd .."))?;
                top.size += current.size;
                top.content.push( (current.size, current.name) );

                current = top;
            }
        } else if line.starts_with("$ ls") {
            assert!( current.content.is_empty() );
        } else if !line.starts_with("dir") {
            let p = line.find(' ').ok_or(miette!("No space? Line: {}", line_no))?;
            let s = line[..p].parse().into_diagnostic().context(format!("Invalid size name. Line: {line_no}"))?;

            current.size += s;
            current.content.push( (s, line[p+1..].to_owned()) );
        }
    }

    while  !stack.is_empty()  {
        if current.size <= 100000 {
            answer_a += current.size;
            dbg!(&current.name);
        }

        dir_list.push(current.size);

        let mut top = stack.pop().ok_or(miette!("Too much cd .."))?;
        top.size += current.size;
        top.content.push( (current.size, current.name) );

        current = top;
    }
    dbg!(current.name);

    dir_list.sort();

    dbg!(current.size);

    let need_space = current.size - ( 70000000 - 30000000 );
    dbg!(need_space);

    for dir_size in dir_list {
        if dir_size >= need_space {
            answer_b = dir_size;
            break;
        }
    }

    dbg!(answer_a);
    dbg!(answer_b);

    Ok(())
}

