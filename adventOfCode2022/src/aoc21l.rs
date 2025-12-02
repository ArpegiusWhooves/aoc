

pub type Num = u64;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum  OperationType {
    Add,
    Substract,
    Multiply,
    Divide,
    Equal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum  Value {
    Number( Num ),
    Reference( usize ),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Info {
    Variable,
    Constant( Num ),
    Operation( OperationType, Value, Value ),
}

// pub enum  Operation<'a> {
//     Noop( Num ),
//     Add( &'a str, &'a str ),
//     Substract( &'a str, &'a str ),
//     Multiply( &'a str, &'a str ),
//     Divide( &'a str, &'a str ),
// }

