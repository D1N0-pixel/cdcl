#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SolverState {
    UNSAT,
    SAT,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralState {
    UNKNOWN,
    TRUE,
    FALSE,
}
