use miette::{IntoDiagnostic, Result};

use testing::get_data;

type Index = u64;

fn beats_record(time: Index, distance: Index, my: Index) -> bool {
    (time - my) * my > distance
}

fn find_beat(
    time: Index,
    distance: Index,
    min: Index,
    max: Index,
) -> Option<(Index, Index, Index)> {
    if min >= max {
        assert!(min == max);
        if beats_record(time, distance, min) {
            return Some((min, min, min));
        }
        return None;
    }
    let mid = (max + min) / 2;
    let my_distance = (time - mid) * mid;
    if my_distance > distance {
        return Some((min, mid, max));
    }
    for mid2 in mid..=max {
        let my_distance2 = (time - mid2) * mid2;
        if my_distance == my_distance2 {
            continue;
        }
        if my_distance2 > distance {
            return Some((mid, mid2, max));
        }
        if my_distance < my_distance2 {
            return find_beat(time, distance, mid2, max);
        }
        break;
    }
    find_beat(time, distance, min, mid)
}

fn find_left(time: Index, distance: Index, min: Index, max: Index) -> Index {
    if min + 1 >= max {
        assert!(min + 1 == max);
        return max;
    }
    let mid = (max + min) / 2;
    if beats_record(time, distance, mid) {
        return find_left(time, distance, min, mid);
    }
    find_left(time, distance, mid, max)
}

fn find_right(time: Index, distance: Index, min: Index, max: Index) -> Index {
    if min + 1 >= max {
        assert!(min + 1 == max);
        return min;
    }
    let mid = (max + min) / 2;
    if beats_record(time, distance, mid) {
        return find_right(time, distance, mid, max);
    }
    find_right(time, distance, min, mid)
}

fn aoc1((body, _file): (String, String)) -> Result<u64> {
    let mut lines = body.split("\n");
    let time = lines
        .next()
        .expect("Time")
        .chars()
        .skip(5)
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<Index>()
        .into_diagnostic()?;
    let distance = lines
        .next()
        .expect("Distance")
        .chars()
        .skip(10)
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<Index>()
        .into_diagnostic()?;

    let (min, mid, max) = find_beat(time, distance, 0, time).expect("Unbeatable!");
    if min == max {
        panic!("WAT?")
    }

    let left = find_left(time, distance, min, mid);
    let right = find_right(time, distance, mid, max);

    let size = right - left + 1;
    println!("{time} {distance}: {left} # {right} => {size}");

    Ok(size)
}

#[test]
fn test1() {
    assert_eq!(
        aoc1((
            "\
Time:      7  15   30
Distance:  9  40  200
"
            .to_owned(),
            "test1".to_owned()
        ))
        .expect("no errors"),
        71503
    );
}

fn main() -> Result<()> {
    println!("{}", aoc1(get_data(file!())?)?);

    Ok(())
}
