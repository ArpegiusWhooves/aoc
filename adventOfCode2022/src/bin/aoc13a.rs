

 
use std::cmp::Ordering;

use miette::{Result, IntoDiagnostic};
 

use serde_json::Value;
use testing::get_data;

fn cmp_arrays( la: &[Value], ra: &[Value] ) -> Ordering {
    let  mut li = la.iter();
    let  mut ri = ra.iter();
    loop {  
        let r = match (li.next(),ri.next()) {
            (Some(l),Some(r)) => cmp_json(l, r),
            (Some(_),None) => Ordering::Greater,
            (None,Some(_)) => Ordering::Less,
            (None,None) => return Ordering::Equal
        };
        if r.is_ne() { return r }
    } 
}

fn cmp_json( l: &Value, r: &Value ) -> Ordering {
    use Value::{Number,Array};
    match (l,r) {
        (Number(ln), Number(rn)) => {
            // println!("{ln} ? {rn}");
            ln.as_u64().cmp(&rn.as_u64())
        },
        (Number(_), Array(ra)) => cmp_arrays(&[l.clone()],ra),
        (Array(la), Number(_)) => cmp_arrays(la,&[r.clone()]),
        (Array(la), Array(ra)) => cmp_arrays(la,ra),
        _ => panic!("unimplemented type")
    } 
}

fn main() -> Result<()> { 
    let (body,_) = get_data(file!())?; 
 
    let mut answer_a = 0; 

    let mut lines = body.split('\n');

    let mut idx = 1;
    while let (Some(line_a),Some(line_b)) = (lines.next(),lines.next()) {
        dbg!(line_a,line_b,idx); 

        let a: serde_json::Value = serde_json::from_str(line_a).into_diagnostic()?;
        let b: serde_json::Value = serde_json::from_str(line_b).into_diagnostic()?;

 
        if cmp_json(&a, &b).is_lt() {
            answer_a += idx;
        }

        dbg!(answer_a); 

        idx += 1;

        assert!(Some("") == lines.next());
    }
    
 
    dbg!(answer_a); 
 

    Ok(())
}

