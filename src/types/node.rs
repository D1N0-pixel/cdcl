use super::state::LiteralState;
use super::data::Literal;

#[derive(Debug)]
pub struct Node {
    pub literal: Literal,
    pub value: LiteralState,
    pub level: usize,
    pub parents: Vec<i32>,
}

impl Node {
    pub fn new(literal: i32) -> Self {
        Node {
            literal,
            value: LiteralState::UNKNOWN,
            level: 0,
            parents: Vec::new(),
        }
    }

    pub fn negative_value(&self) -> LiteralState {
        match self.value {
            LiteralState::TRUE => LiteralState::FALSE,
            LiteralState::FALSE => LiteralState::TRUE,
            LiteralState::UNKNOWN => LiteralState::UNKNOWN,
        }
    }
}
