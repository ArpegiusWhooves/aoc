
use std::collections::VecDeque;
use std::usize;

use miette::{Result, miette};
use testing::aoc21l::Info::{Variable, Constant, Operation};
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
    if let Operation(op, _, _) = &mut monkeys[root].1  {
        *op = Equal;
    } else {
        return Err(miette!("Ivalid input data!"));
    };
    let human = aid.get("humn");
    monkeys[human].1 = Variable;

    let mut dependencies = Vec::new();
    dependencies.resize(monkeys.len(), Vec::<usize>::new());

    let mut stack = VecDeque::from_iter( 0 .. monkeys.len() );
    
    while let Some(monkey_id) = stack.pop_front() {
        let Operation(operation, mut left_operand, mut right_operand) = monkeys[monkey_id].1.clone() 
            else {continue};
        if let Reference(ref_id) = left_operand {
            if let Constant(c) = monkeys[ref_id].1 {
                left_operand = Number(c);
            } else {
                dependencies[ref_id].push(monkey_id);
            }
        }
        if let Reference(ref_id) = right_operand {
            if let Constant(c) = monkeys[ref_id].1 {
                right_operand = Number(c);
            } else {
                dependencies[ref_id].push(monkey_id);
            }
        }
        match (left_operand,right_operand) {
            (Number(left_value), Number(right_value)) => {
                let result_value = match operation {
                    Add => left_value+right_value,
                    Substract => left_value-right_value,
                    Multiply => left_value*right_value,
                    Divide => { assert!(left_value.rem_euclid(right_value)==0);left_value/right_value},
                    Equal => {assert!(left_value == right_value); 0},
                };
                monkeys[monkey_id].1 = Constant(result_value);
                for r in dependencies[monkey_id].iter() {
                    stack.push_back(*r);
                }
            },
            (Number(_), Reference(_)) |
            (Reference(_), Number(_)) => monkeys[monkey_id].1 = Operation(operation,left_operand,right_operand),
            (Reference(_), Reference(_)) => {},
        }
    } 
    
    let mut stack = match monkeys[root].1 {
        Operation(Equal, Number(value), Reference(ref_id)) 
        | Operation(Equal, Reference(ref_id), Number(value)) 
        => { VecDeque::from([(ref_id,value)]) },
        _ => panic!(), 
    };

    while let Some((monkey_id,value)) = stack.pop_front() {

        if matches!(monkeys[monkey_id].1, Variable) {
            println!("answer b: {value}");
            break;
        }

        let Operation(op, left_operand,right_operand) 
            = monkeys[monkey_id].1.clone() else {panic!()};
        
        stack.push_front( match (left_operand,right_operand) {
            (Number(_), Number(_)) => { panic!() },
            (Number(left_value), Reference(ref_id)) => 
                (ref_id,match op {
                    Add => value - left_value,
                    Multiply => value / left_value,
                    Substract => left_value - value,
                    Divide => left_value / value,
                    Equal => {panic!()},
                }),
            (Reference(ref_id), Number(right_value)) => {
                (ref_id,match op {
                    Add => value - right_value,
                    Multiply => value / right_value,
                    Substract => value + right_value,
                    Divide => value * right_value,
                    Equal => {panic!()},
                })
            },
            (Reference(_), Reference(_)) => {panic!()},
        } );
    }

    Ok(())
}

