
use std::collections::VecDeque;
use std::usize;

use miette::Result;
use testing::aoc21l::Info::{Constant, Operation};
use testing::aoc21l::OperationType::{Add, Substract, Multiply, Divide, Equal};
use testing::aoc21l::Value::{Number, Reference};
use testing::{AutoIndex, ParserResultWithCode, get_data};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub aoc21p);

fn main() -> Result<()> { 
    let (body,filename) = get_data(file!())?;

    let mut aid = AutoIndex::default();

    let mut monkeys = 
        aoc21p::MathMonkeysParser::new()
        .parse(&mut aid,&body) 
        .error_with_source(&filename, &body)?;

    monkeys.sort();

    let root = aid.get("root");

    let mut dependencies = Vec::new();
    dependencies.resize(monkeys.len(), Vec::<usize>::new());

    let mut stack = VecDeque::from_iter( 0 .. monkeys.len() );
    
    while let Some(id) = stack.pop_front() {
        let Operation(op, mut lv, mut rv) = monkeys[id].1.clone() else {continue};
        if let Reference(ref_id) = lv {
            if let Constant(c) = monkeys[ref_id].1 {
                lv = Number(c);
            } else {
                dependencies[ref_id].push(id);
            }
        }
        if let Reference(ref_id) = rv {
            if let Constant(c) = monkeys[ref_id].1 {
                rv = Number(c);
            } else {
                dependencies[ref_id].push(id);
            }
        }
        if let (Number(l), Number(r)) = (lv,rv) {
            let v = match op {
                Add => l+r,
                Substract => l-r,
                Multiply => l*r,
                Divide => {assert!(l%r==0);l/r},
                Equal => unimplemented!(),
            };
            monkeys[id].1 = Constant(v);
            for r in dependencies[id].iter() {
                stack.push_back(*r);
            }
        }
    }

    println!("{:?}",monkeys[root].1);

    Ok(())
}

