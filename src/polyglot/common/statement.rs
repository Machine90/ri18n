use crate::polyglot::common::{field::Field, used_types::Construct};

#[derive(Debug, Clone)]
pub enum RightHand {
    Value(String),
    Construct(Construct),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Assign,
    /// Equal to
    ET,
    /// Not equal to
    NET,
    /// greater or equal to
    GET,
    GT,
    /// Smaller or equal to
    SET,
    ST,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Assign => "=",
            Operation::ET => "==",
            Operation::NET => "!=",
            Operation::GET => ">=",
            Operation::GT => ">",
            Operation::SET => "<=",
            Operation::ST => "<",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub left_hand: Field,
    pub op: Operation,
    pub right_hand: RightHand,
}
