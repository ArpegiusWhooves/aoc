

#![feature(extract_if)]

use miette::Result;

fn snafu_to_u64(snafu:&str) -> i64 {
 let mut exp = 1i64;
 let mut val = 0i64;
 for c in snafu.chars().rev() {
    val += exp * match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '-' => -1,
        '=' => -2,
        _ => panic!(),
    };
    exp *= 5;
 }
 val 
}

fn main() -> Result<()> { 

    // SNAFU  Decimal
    assert!(snafu_to_u64("1=-0-2") == 1747);
    assert!(snafu_to_u64("12111") ==      906);
    assert!(snafu_to_u64("2=0=") ==      198);
    assert!(snafu_to_u64("21") ==       11);
    assert!(snafu_to_u64("2=01") ==      201);
    assert!(snafu_to_u64("111") ==       31);
    assert!(snafu_to_u64("20012") ==     1257);
    assert!(snafu_to_u64("112") ==       32);
    assert!(snafu_to_u64("1=-1=") ==      353);
    assert!(snafu_to_u64("1-12") ==      107);
    assert!(snafu_to_u64("12") ==        7);
    assert!(snafu_to_u64("1=") ==        3);
    assert!(snafu_to_u64("122") ==      37);

    let (source,_filename) = testing::get_data(file!())?;

    let answer: i64 = source.split('\n').map(snafu_to_u64).sum();

    dbg!(answer);


    dbg!(num::BigInt::from(answer).to_str_radix(5));
     
//                    12322024001244014000
    let ans  = "2==221=-002=0-02-000";
    dbg!(snafu_to_u64(ans));

    dbg!(num::BigInt::from(snafu_to_u64(ans)).to_str_radix(5));

    assert!( dbg!(snafu_to_u64(ans)) == answer );

    Ok(())
}

