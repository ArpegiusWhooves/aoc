

#![feature(extract_if)]
 
use std::collections::VecDeque;
use std::collections::{HashSet, BTreeMap};
 
use itertools::Itertools;
use miette::Result; 
use testing::{ParserResultWithCode, get_data};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub aoc16p);

const TIME_LEFT:i32 = 26;

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

// poor substitute of bitset, for faster node checking
trait ToBitSet {
    type BitSetType : Copy;
    fn to_bit_set( &self ) -> Self::BitSetType;
    fn some_inside_bit_set( &self, bit_set: Self::BitSetType ) -> bool;
    fn add_to_bit_set( &self, bit_set: Self::BitSetType ) -> Self::BitSetType;
    fn all_bellow_inside_bit_set( &self, bit_set: Self::BitSetType ) -> bool;
}

impl ToBitSet for usize {
    type BitSetType = u64;
    fn to_bit_set( &self ) -> Self::BitSetType {
        1 << self
    }
    fn some_inside_bit_set( &self, bit_set: Self::BitSetType ) -> bool {
        bit_set & ( self.to_bit_set() ) != 0
    }
    fn add_to_bit_set( &self, bit_set: Self::BitSetType ) -> Self::BitSetType {
        bit_set | self.to_bit_set()
    }
    fn all_bellow_inside_bit_set( &self, bit_set: Self::BitSetType ) -> bool {
        let all_bits = self.to_bit_set()-1;
        all_bits & bit_set == all_bits
    }
}

#[derive(Debug,Clone)]
struct Step {
    score:i32,
    rate_sum:i32,
    time_left:i32,
    current_position: usize,
    visited_valves: u64,
    // visit_order: Vec<usize>,
}


impl Step
{
    fn new(rate:i32, time_left:i32, current_position: usize ) -> Self {
        Self {
            score: 0,
            rate_sum: rate,
            time_left,
            current_position,
            visited_valves:  current_position.to_bit_set(),
            // visit_order: vec![current_position]
        }
    } 
    fn open(&self, rate:i32, time:i32, new_position:usize)->Self{
        Self {
            score: self.score + self.rate_sum * time,
            rate_sum: self.rate_sum + rate,
            time_left: self.time_left - time,
            current_position: new_position,
            visited_valves: new_position.add_to_bit_set(self.visited_valves),
            // visit_order: self.visit_order.iter().copied().chain(Some(new_position)).collect()
        }
    }
    fn wait(&self)->Self{
        Self {
            score: self.score + self.rate_sum * self.time_left,
            time_left: 0,
            .. self.clone()
        }
    }

    fn println(&self, id:usize, good_valves: &[GoodValve]) {
        print!("Player{id} Score:{} Rate:{} ",self.score, self.rate_sum);
        for (valve_id,valve) in good_valves.iter().enumerate() {
            if valve_id.some_inside_bit_set(self.visited_valves) {
                print!(",{}",valve.name);
            }
        }
        // for v in self.visit_order {
        //     write!(f, "->{}",good_valves[v].name);
        // }
        println!();
    }
}

fn main() -> Result<()> { 
    // Download and load input
    let (body,filename) = get_data(file!())?;

    let valves =  BTreeMap::from_iter( 
        aoc16p::ValvesParser::new()
        .parse(&body) 
        .error_with_source(&filename, &body)?);

    //filter out only valves that has rate > 0,
    //we will be opening only thouse nodes
    let mut good_valves: Vec<GoodValve> = valves.iter()
            .filter_map( |(&name,(rate,_tunels))| {
                if *rate == 0 { None } else { Some( GoodValve::new( *rate, name ) ) }
            } ).collect_vec();
    let good_valves_count = good_valves.len();
    good_valves.sort();
    good_valves.reverse(); // Sort from best to worst

    // compute map from valve name to index of good_value table
    let valves_indexes = BTreeMap::from_iter( 
            good_valves.iter()
                .enumerate()
                .map( |(idx, valve)| {
                    (valve.name, idx)
                } )
            );

    // First posible valve to open for any player
    let mut first_steps = Vec::new();

    // Compute distances to other good valves 
    for (current_valve_index, valve) in good_valves.iter_mut().enumerate() {
        // resize the vector of distances
        valve.valve_distances.resize(good_valves_count, 0);
        
        //Compute using BFS algorithm
        let mut visited_valves = HashSet::from([valve.name]);
        let mut visiting_valves = VecDeque::from([(valve.name,0)]); // FIFO
        while let Some((visiting_valve_name,distance)) = visiting_valves.pop_back() {
            let (rate,tunels) = valves.get(visiting_valve_name).unwrap();
            if visiting_valve_name == "AA" {
                // If we get to start node, we add current valve to start valves
                first_steps.push( Step::new(
                    valve.rate, 
                    TIME_LEFT - distance - 1,
                    current_valve_index) );
            }
            if rate>&0 { 
                // if this is a good valve serch for index
                let &index = valves_indexes.get(visiting_valve_name).unwrap();
                // and add distance to vector 
                valve.valve_distances[index] = distance;
            }
            for &tunel in tunels {
                if visited_valves.insert(tunel) {
                    //Generate new unvisited steps in BFS
                    visiting_valves.push_front((tunel,distance+1));
                }
            }
        } 
    }
    
    first_steps.reverse();

    // Geterate first steps for both players 
    let mut steps = first_steps.iter().cartesian_product(first_steps.iter()).filter_map(
        |(player1,player2)| {
            if player1.current_position >= player2.current_position {
                None
            } else {
                Some( (player1.clone(), player2.clone()) )
            }
        }
    ).collect_vec(); // FILO
 
    // result
    let mut answer_a = 0;

    // Run DFS algorithms for all posible steps of opening vavles for both players 
    while let Some((player1,player2)) = steps.pop() { 
        let visited_valves = player1.visited_valves | player2.visited_valves;
        if good_valves_count.all_bellow_inside_bit_set( visited_valves ) {
            // visited all valves, check score
            let score = player1.wait().score + player2.wait().score;
            if answer_a < score {
                answer_a = score;
                println!("Score all:{score}");
                player1.println(1, &good_valves);
                player2.println(2, &good_valves);
            }
            continue;
        }

        if player1.time_left == 0 && player2.time_left == 0 {
            // players timeouted, check score
            let score = player1.score + player2.score;
            if answer_a < score {
                answer_a = score;
                println!("Score timeout:{score}");
                player1.println(1, &good_valves);
                player2.println(2, &good_valves);
            }
            continue;
        }
 
        let mut need_wait = false;

        // run player that have more time first
        if player1.time_left >= player2.time_left {
            for player1_next in (0 .. good_valves_count).rev() {
                //check if node already opened
                if player1_next.some_inside_bit_set(visited_valves) {continue} 
                // compute the time to open the valve
                let time_consume = good_valves[player1.current_position]
                                            .valve_distances[player1_next] + 1;
                if player1.time_left < time_consume {
                    // No time left to visit
                    need_wait = true;
                } else {
                    //New step with player 1 opened valve and add to the DSF
                    steps.push( ( 
                        player1.open(
                            good_valves[player1_next].rate,
                            time_consume,
                            player1_next
                        ),
                        player2.clone() 
                    ) )
                }
            }
            if need_wait { 
                //New step with player 1 waited to the end and add it to the DSF
                steps.push( ( 
                    player1.wait(),
                    player2.clone() 
                ) )
            }
        } else {
            for player2_next in (0 .. good_valves_count).rev() {
                // check if node already opened
                if player2_next.some_inside_bit_set(visited_valves) {continue} 
                // compute the time to open the valve
                let time_consume = good_valves[player2.current_position]
                                            .valve_distances[player2_next] + 1;
                if player1.time_left < time_consume {
                    // No time left to visit
                    need_wait = true
                } else {
                    //New step with player 2 opened valve and add to the DSF
                    steps.push( ( 
                        player1.clone(),
                        player2.open(
                            good_valves[player2_next].rate,
                            time_consume,
                            player2_next
                        )
                    ) )
                }
            }
            if need_wait { 
                //New step with player 2 waited to the end and add it to the DSF
                steps.push( (
                    player1.clone(),
                    player2.wait()
                ) )
            }
        }
    }

    println!("Answer 16 part2:{answer_a}");

    Ok(())
}

