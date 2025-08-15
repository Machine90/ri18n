use std::hash::Hash;

use crate::polyglot::common::{field::Field, used_types::Construct};

#[derive(Debug, Clone)]
pub enum RightHand {
    Value(String),
    Construct(Construct),
}

#[allow(unused)]
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

impl PartialEq for Statement {
    fn eq(&self, other: &Self) -> bool {
        self.left_hand == other.left_hand
    }
}

impl Eq for Statement {}

impl Hash for Statement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.left_hand.hash(state);
    }
}

impl PartialOrd for Statement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.left_hand.partial_cmp(&other.left_hand)
    }
}

impl Ord for Statement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.left_hand.cmp(&other.left_hand)
    }
}
