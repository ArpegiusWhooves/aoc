use std::collections::VecDeque;


#[derive(Debug)]
pub enum Value {
    Old,
    Number( i64 )
}

#[derive(Debug)]
pub enum  Operation {
    Multiply( Value, Value ),
    Add( Value, Value ),
}

 
#[derive(Debug)]
pub struct Monkey {
    pub nr:i64,
    pub worry_items: VecDeque<i64>,
    pub operation: Operation,
    pub test_divisible_by: i64,
    pub if_true_throw_to: i64,
    pub if_false_throw_to: i64
}
