

#![feature(extract_if)]

use std::ops::RangeInclusive;

use miette::Result;
use testing::{ParserResultWithCode, get_data};


use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub aoc15p);

fn city_distance( (ax,ay): &(i32, i32), (bx,by): &(i32, i32)  ) -> i32 {
    (ax-bx).abs() + (ay-by).abs()
}

fn main() -> Result<()> { 
    let (body,filename) = get_data(file!())?;
 
    let mut answer_b = 0;
 
    let mut sensors = aoc15p::SensorsParser::new()
        .parse(&body)
        .error_with_source(&filename, &body)?;

    sensors.sort();

    for y in 0 ..= 4000000 {

        let mut ranges: Vec<RangeInclusive<i32>> = Vec::new();
        for (sensor,becon) in &sensors {

            let d = city_distance(sensor,becon);
            let dif_x = d - (sensor.1 - y).abs();

            if dif_x < 0 { continue }

            let min_x = sensor.0 - dif_x;
            let max_x = sensor.0 + dif_x;

            let mut new_range = min_x ..= max_x;

            let _ = ranges.extract_if(|range|{
                if range.end() < new_range.start() { return false }
                if new_range.end() < range.start() { return false }
                new_range = *range.start().min(new_range.start()) ..= *range.end().max(new_range.end());
                true
            });
            ranges.push(new_range);
        }
        if y & 1023 == 0 { println!("{y}"); }
        // for range in &ranges {
        //     print!("|{range:?}");
        // }
        // println!("|");
        if ranges.len() > 1 {
            answer_b = y as u64 + (ranges[0].end()+1) as u64 * 4000000;
            break;
        }
    }

    dbg!(answer_b);
 
    Ok(())
}

