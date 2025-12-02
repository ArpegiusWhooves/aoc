

 
use miette::{Result, miette};
 
use testing::{aoc11l::{Monkey,Operation,Value}, ParserResultWithCode, get_data};
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub aoc11p);

fn print_monkeys_item(monkeys: &Vec<Monkey>) {
    for monkey in monkeys {
        println!("Monkey {}: {:?}",monkey.nr, monkey.worry_items);
    }
}

fn main() -> Result<()> { 
    let (body,filename) = get_data(file!())?;
  
 
    let mut monkeys = aoc11p::MonkeysParser::new()
        .parse(&body)
        .error_with_source(&filename, &body)?;
    
    dbg!(&monkeys);
    print_monkeys_item(&monkeys);

    let mut times: Vec<(i64,i64)> = monkeys.iter().map(|m|(0,m.nr)).collect();

    let manage: i64 = monkeys.iter().map(|m|m.test_divisible_by).product();
    dbg!(manage);

    for _round in 1 ..= 10000 {
        for idx in 0 .. monkeys.len() {
            while let Some(item_level) = monkeys[idx].worry_items.pop_front() {
                times[idx].0 += 1;
                let new_level = match monkeys[idx].operation {
                    Operation::Multiply(Value::Old, Value::Number(b)) => item_level * b,
                    Operation::Multiply(Value::Old, Value::Old) => item_level * item_level,
                    Operation::Add(Value::Old, Value::Number(a)) => item_level + a,
                    Operation::Add(Value::Old, Value::Old) => item_level + item_level,
                    ref x => return Err(miette!("Unsuported operation {:?} for monkey {}.",x,idx)),
                };
                let throw_to = if new_level % monkeys[idx].test_divisible_by == 0 {
                    monkeys[idx].if_true_throw_to
                } else {
                    monkeys[idx].if_false_throw_to
                };
                monkeys[throw_to as usize].worry_items.push_back(new_level % manage);
            }
        }
        if _round % 1000 == 0 {
            dbg!(_round);
            for (monkey_items, monkey_idx) in &times { 
                println!("Monkey {monkey_idx}: {monkey_items}");
            }
        }
    }

    dbg!(&times);
    times.sort();
    times.reverse();
    dbg!(&times);
 
 
    let answer_b = times[0].0 * times[1].0;
 
    dbg!(answer_b);
 

    Ok(())
}

