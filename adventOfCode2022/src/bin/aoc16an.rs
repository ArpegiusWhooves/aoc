

#![feature(extract_if)]
 
use std::collections::VecDeque;
use std::collections::{HashSet, BTreeMap}; 
 
use itertools::Itertools;
use miette::Result; 
use testing::{ParserResultWithCode, get_data};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub aoc16p);


const TIME_LEFT:i32 = 30;

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
struct GoodValve<'a> {
    rate: i32,
    name: &'a str,
    valve_distances: Vec<i32>,
}

impl<'a> GoodValve<'a> {
    fn new(rate:i32, name: &'a str) -> Self {
        Self {
            rate,
            name,
            valve_distances: Vec::new()
        }
    }
}

struct Step {
    score:i32,
    rate_sum:i32,
    time_left:i32,
    current_valve_index:usize,
    visited_valves: u64,
    visit_order: Vec<usize>,
}

impl Step {
    fn new(rate:i32, distance:i32, current_valve_index:usize ) -> Self {
        Self {
            score: 0,
            rate_sum: rate,
            time_left: TIME_LEFT - distance - 1,
            current_valve_index,
            visited_valves: 1u64 << current_valve_index,
            visit_order: vec![current_valve_index]
        }
    }
    fn open(&self, rate:i32, time:i32, valve_index:usize)->Self{
        Self {
            score: self.score + self.rate_sum * time,
            rate_sum: self.rate_sum + rate,
            time_left: self.time_left - time,
            current_valve_index: valve_index,
            visited_valves: self.visited_valves | 1u64 << valve_index,
            visit_order: self.visit_order.iter().copied().chain(Some(valve_index)).collect()
        }
    }
}

fn main() -> Result<()> { 
    let (body,filename) = get_data(file!())?;

    let valves =  BTreeMap::from_iter( 
        aoc16p::ValvesParser::new()
        .parse(&body) 
        .error_with_source(&filename, &body)?);

    let mut good_valves: Vec<GoodValve> = valves.iter().filter_map( |(&name,(rate,_tunels))| {
        if *rate == 0 { None } else { Some( GoodValve::new( *rate, name ) ) }
    } ).collect_vec();
    good_valves.sort();
    good_valves.reverse();
    let good_valves_count = good_valves.len();

    let valves_indexes = BTreeMap::from_iter( 
            good_valves.iter()
                .enumerate()
                .map( |(idx, valve)| {
                    (valve.name, idx)
                } )
            );

    dbg!(&valves_indexes);

    let mut first_steps = Vec::new();

    for (current_valve_index, valve) in good_valves.iter_mut().enumerate() {
        assert!(Some(&current_valve_index) == valves_indexes.get(valve.name));
        valve.valve_distances.resize(good_valves_count, 0);
        let mut visited_valves = HashSet::from([valve.name]);
        let mut visiting_valves = VecDeque::from([(valve.name,0)]);
        while let Some((visiting_valve_name,distance)) = visiting_valves.pop_back() {
            let (rate,tunels) = valves.get(visiting_valve_name).unwrap();
            if visiting_valve_name == "AA" {
                first_steps.push( Step::new(valve.rate, distance, current_valve_index) );
            }
            if rate>&0 {
                let &index = valves_indexes.get(visiting_valve_name).unwrap();
                valve.valve_distances[index] = distance;
            }
            for &tunel in tunels {
                if visited_valves.insert(tunel) {
                    visiting_valves.push_front((tunel,distance+1));
                }
            }
        } 
    }
    
    let mut steps = first_steps;
    steps.reverse();
    let mut answer_a = 0;

    while let Some(step) = steps.pop() {
        if step.visited_valves + 1 == 1u64 << good_valves_count {
            // visited all valves
            let score = step.score + step.rate_sum * step.time_left;
            if answer_a < score {
                answer_a = score;
                print!("Score:{score}");
                for v in step.visit_order {
                    print!("->{}",good_valves[v].name);
                }
                println!();
            }
            continue;
        }

        // if step.visit_order == [2,3] {
        //     println!("test");
        // }

        let mut need_finish = true;

        let valve = &good_valves[step.current_valve_index];
        for valve_index_to in (0 .. good_valves_count).rev() {
            if step.visited_valves & ( 1 << valve_index_to ) == 0 {
                let time_consume = valve.valve_distances[valve_index_to] + 1;

                if step.time_left <= time_consume {
                    // No time left to visit
                    continue;
                }
                need_finish=false;
                steps.push( step.open(good_valves[valve_index_to].rate, time_consume, valve_index_to) );
            }
        }
        if need_finish {
            let score = step.score + step.rate_sum * step.time_left;
            if answer_a < score {
                answer_a = score;
                println!("Timeout:{score}");
                for v in step.visit_order {
                    print!("->{}",good_valves[v].name);
                }
                println!();
            }
        }

    }


    Ok(())
}

