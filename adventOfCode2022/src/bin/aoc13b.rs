

 
use std::cmp::Ordering;
use miette::{Result, IntoDiagnostic};
 
use serde_json::{Value, json};
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
  
    let mut items = Vec::<Value>::new();

    for line in body.split('\n') {
        if line.is_empty() { continue }
        items.push( serde_json::from_str::<Value>(line).into_diagnostic()? );
    }

    let start = json!([[2]]);
    items.push(start.clone());
    let end = json!([[6]]);
    items.push(end.clone());
    

    items.sort_by(cmp_json );
    
    for item in &items {
        println!( "{}",  serde_json::to_string(item).into_diagnostic()? );
    }
 
    let mut answer_b = 1 + items.iter().position(|v| v == &start ).unwrap();
    answer_b *= 1 + items.iter().position(|v| v == &end ).unwrap(); 
 
    dbg!(answer_b);

    Ok(())
}

