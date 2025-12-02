

 

use miette::{Result, IntoDiagnostic, miette};
use testing::get_data;
 
struct Cpu {
    clock: i32,
    accu_x: i32,
    screen: [[char;40];6]
}

impl Cpu {
    
    fn step(&mut self) {
        // dbg!(self.clock);
        let pos = (self.clock - 1) as usize;
        let row = pos / 40;
        let col = pos % 40;

        if row >= self.screen.len() {
            return
        }

        self.screen[row][col] = match  col as i32 - self.accu_x {
            1|0|-1 => {'#'},
            _ => {'.'}
        };
    }

    fn print_screen(&self) {
        for line in self.screen{
            let text = line.iter().collect::<String>();
            println!("{text}");
        }
    }

    fn cmd(&mut self, command: &str, attr: &str) -> Result<()> {
        match command {
            "noop" => self.clock += 1,
            "addx" => { 
                self.clock += 1;
                self.step();
                self.clock += 1;
                self.accu_x += attr.parse::<i32>().into_diagnostic()?;
            }
            _ => return Err(miette!("Unknown command {}.",command)),
        }
        self.step();
        Ok(())
    }

}

fn main() -> Result<()> { 
    let (body,_) = get_data(file!())?;
 
    let mut answer_a = 0; 

    let mut cpu  = Cpu{ 
        clock: 1,
        accu_x: 1,
        screen: [['.';40];6]
    };

    cpu.step();
 
    let mut check_iter = (0 .. ).map(|i| 20 + i * 40).peekable();

    for line in body.split('\n') {
        if line.is_empty() {
            continue;
        }

        match check_iter.peek() {
            Some(check) => {
                if cpu.clock == *check || cpu.clock == check - 1 {
                    answer_a += check * cpu.accu_x; 
                    check_iter.next();
                }
            },
            None => panic!(),
        }
        cpu.cmd(&line[0..4], if line.len() > 4 {&line[5..]} else {""})?;
    }
 
    dbg!(answer_a); 

    cpu.print_screen();

    Ok(())
}

