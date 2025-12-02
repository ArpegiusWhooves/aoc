




use miette::Result;

use lalrpop_util::lalrpop_mod;
use testing::ParserResultWithCode;
lalrpop_mod!(pub aoc19p);

use testing::aoc19l::{Cost, Blueprint};

#[derive(Debug,Clone,Copy,Default,PartialEq, Eq, PartialOrd, Ord)]
struct Resouces {
    pub geode: u8,
    pub geode_robot: u8,
    pub obsidian_robot: u8,
    pub obsidian: u8,
    pub clay_robot: u8,
    pub ore_robot: u8,
    pub clay: u8,
    pub ore: u8,
}

impl Resouces {
    fn make_robot( &self, cost: &Cost) -> Option<Self> {
        Some( Self { 
            ore: self.ore.checked_sub(cost.ore)?,
            clay: self.clay.checked_sub(cost.clay)?,
            obsidian: self.obsidian.checked_sub(cost.obsidian)?,
            ..*self
        })
    }
    fn run_robots( &mut self ) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }
}

fn ckeck_blueprint(bp: &Blueprint, time: i32) -> u32 {
    let mut states = Vec::from([Resouces::default()]);
    states[0].ore_robot = 1;

    let mut max_geode = Resouces::default(); 

    let max_ore = bp.clay_robot.ore.max(
        bp.obsidian_robot.ore.max(
            bp.geode_robot.ore
        )
    );

    for minute in 0 .. time {
        let mut new_states = Vec::new();
        for mut state in states {
            if max_geode.geode < state.geode {
                max_geode = state;
            }

            if state.ore_robot < max_ore {
                if let Some(mut new_state) = state.make_robot(&bp.ore_robot) {
                    new_state.run_robots();
                    new_state.ore_robot+=1;
                    new_states.push( new_state );
                }
            }
            if state.clay_robot < bp.obsidian_robot.clay {
                if let Some(mut new_state) = state.make_robot(&bp.clay_robot) {
                    new_state.run_robots();
                    new_state.clay_robot+=1;
                    new_states.push( new_state );
                }
            }
            if state.obsidian_robot < bp.geode_robot.obsidian {
                if let Some(mut new_state) = state.make_robot(&bp.obsidian_robot) {
                    new_state.run_robots();
                    new_state.obsidian_robot+=1;
                    new_states.push( new_state );
                }
            }
            if let Some(mut new_state) = state.make_robot(&bp.geode_robot) {
                new_state.run_robots();
                new_state.geode_robot+=1;
                new_states.push( new_state );
            } else {
                state.run_robots();
                new_states.push( state );
            }
        }
        if max_geode.geode > 0 {
            println!("Minute:{minute} states:{} | {:?}",new_states.len(),&max_geode);
        } else {
            println!("Minute:{minute} states:{}",new_states.len());
        }

        if new_states.len() > 1_000_000 {
            new_states.sort();
            new_states.reverse();
            new_states.truncate(1_000_000);
        }
        states = new_states;
    }
    for state in states {
        if max_geode.geode < state.geode {
            max_geode = state;
        }
    }

    dbg!(max_geode.geode) as u32
}

fn main() -> Result<()> { 
 
    let (body,filename) = testing::get_data(file!())?;
    
    let blueprints =  
        aoc19p::BlueprintsParser::new()
        .parse(&body) 
        .error_with_source(&filename, &body)?;
    
    let mut answer_a: u32 = 0;
    let mut answer_b: u32 = 1;

    for bp in blueprints[..3].iter() {
        let score = ckeck_blueprint(bp,32); 
        answer_b *= score;
    }

    for bp in blueprints {
        let score = ckeck_blueprint(&bp,24) * (bp.id as u32); 
        answer_a += score;
    }

    dbg!(answer_a);
    dbg!(answer_b);
 
    Ok(())
}

