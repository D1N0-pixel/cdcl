extern crate cdcl;

use cdcl::parser::parser::Parser;
use cdcl::solver::solver::Solver;
use cdcl::types::state::SolverState;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("File not found");
    let mut formula = String::new();
    f.read_to_string(&mut formula).expect("something went wrong reading the file");

    let mut parser = Parser::new(formula.as_str());
    let cnf = parser.parse().expect("Invalid formula");
    let mut solver = Solver::new(cnf, parser.nbvars);
    
    let state = solver.solve();
    match state {
        SolverState::UNSAT => println!("UNSAT"),
        SolverState::SAT => println!("SAT"),
    }
}

#[test]
fn test_sat() {
    for i in 1..=1000 {
        let filename = format!("test/sat/uf50-0{i}.cnf");
        let mut f = File::open(filename).expect("File not found");
        let mut formula = String::new();
        f.read_to_string(&mut formula).expect("something went wrong reading the file");

        let mut parser = Parser::new(formula.as_str());
        let cnf = parser.parse().expect("Invalid formula");
        let mut solver = Solver::new(cnf, parser.nbvars);
        
        let state = solver.solve();
        assert_eq!(state, SolverState::SAT);
    }
}

#[test]
fn test_unsat() {
    for i in 1..=1000 {
        let filename = format!("test/unsat/uuf50-0{i}.cnf");
        let mut f = File::open(filename).expect("File not found");
        let mut formula = String::new();
        f.read_to_string(&mut formula).expect("something went wrong reading the file");

        let mut parser = Parser::new(formula.as_str());
        let cnf = parser.parse().expect("Invalid formula");
        let mut solver = Solver::new(cnf, parser.nbvars);
        
        let state = solver.solve();
        assert_eq!(state, SolverState::UNSAT);
    }
}