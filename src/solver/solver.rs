use super::super::types::data::*;
use super::super::types::node::Node;
use super::super::types::state::LiteralState;
use super::super::types::state::SolverState;

use std::collections::HashMap;
use std::vec;

pub struct Solver {
    formula: Cnf,
    selected: Literal,
    level: usize,
    assignment: Assignment,
    trail: Vec<Vec<Literal>>,
}

impl Solver {
    pub fn new(formula: Cnf, nbvars: usize) -> Self {
        let assignment = (0..nbvars).map(|i| Node::new((i + 1) as i32)).collect();
        Solver {
            formula,
            selected: 0,
            level: 0,
            assignment,
            trail: vec![vec![]],
        }
    }

    pub fn solve(&mut self) -> SolverState {
        loop {
            let conflict_clause = self.unit_propagate();
            match conflict_clause {
                Some(clause) => match self.analyze_conflict(&clause, None, None, None) {
                    Some(level) => {
                        self.backtrack(level);
                    }
                    None => {
                        return SolverState::UNSAT;
                    }
                },
                None => {
                    if self.is_satisfied() {
                        return SolverState::SAT;
                    }
                    self.level += 1;
                    self.select_variable();
                }
            }
        }
    }

    fn get_node_value(&self, literal: Literal) -> LiteralState {
        if literal > 0 {
            self.assignment[(literal - 1) as usize].value
        } else {
            self.assignment[(-literal - 1) as usize].negative_value()
        }
    }

    fn get_all_unknown(&mut self) -> Vec<Literal> {
        self.assignment
            .iter()
            .filter(|&n| n.value == LiteralState::UNKNOWN)
            .map(|n| n.literal)
            .collect()
    }

    fn get_unknown_from_clause(&self, clause: &Clause) -> Vec<Literal> {
        clause
            .iter()
            .filter(|&l| self.get_node_value(*l) == LiteralState::UNKNOWN)
            .map(|l| *l)
            .collect()
    }

    fn select_variable(&mut self) {
        let unknowns = self.get_all_unknown();
        self.selected = unknowns[0].abs();
        self.selected = self.most_frequent_literal(unknowns);
        let index = self.literal_to_index(self.selected);
        self.trail.push(vec![]);
        self.assignment[index].parents = vec![];
        if self.selected > 0 {
            self.set_literal(self.selected, LiteralState::TRUE);
        } else if self.selected < 0 {
            self.set_literal(-self.selected, LiteralState::FALSE);
        } else {
            panic!("selected variable is 0");
        }
    }

    fn most_frequent_literal(&self, literals: Vec<Literal>) -> Literal {
        let mut count: HashMap<i32, usize> = HashMap::new();
        for f in self.formula.clone() {
            for &literal in &literals {
                if f.contains(&literal) {
                    *count.entry(literal).or_default() += 1;
                }
                if f.contains(&-literal) {
                    *count.entry(-literal).or_default() += 1;
                }
            }
        }
        count
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k)
            .unwrap()
    }

    fn set_literal(&mut self, literal: Literal, value: LiteralState) {
        match value {
            LiteralState::TRUE => {
                if !self.trail[self.level].contains(&literal) {
                    self.trail[self.level].push(literal);
                }
            }
            LiteralState::FALSE => {
                if !self.trail[self.level].contains(&-literal) {
                    self.trail[self.level].push(-literal);
                }
            }
            _ => {}
        }
        self.assignment
            .get_mut((literal - 1) as usize)
            .unwrap()
            .level = self.level;
        self.assignment
            .get_mut((literal - 1) as usize)
            .unwrap()
            .value = value;
    }

    fn unit_propagate(&mut self) -> Option<Clause> {
        loop {
            let mut unit_clauses: Vec<(Literal, Clause)> = Vec::new();
            for clause in &self.formula {
                let clause_value = self.calculate_clause(clause);
                match clause_value {
                    LiteralState::TRUE => continue,
                    LiteralState::FALSE => {
                        return Some(clause.clone());
                    }
                    LiteralState::UNKNOWN => {
                        let unknown_clause = self.get_unknown_from_clause(clause);
                        if unknown_clause.len() == 1 {
                            unit_clauses.push((unknown_clause[0], clause.clone()));
                        }
                    }
                }
            }
            if unit_clauses.len() == 0 {
                break;
            }
            for (unit_clause, parent_clause) in unit_clauses.iter().rev() {
                let index = self.literal_to_index(*unit_clause);
                if *unit_clause > 0 {
                    self.set_literal(*unit_clause, LiteralState::TRUE);
                } else if *unit_clause < 0 {
                    self.set_literal(-unit_clause, LiteralState::FALSE);
                } else {
                    panic!("unit clause with 0 literal");
                }
                self.assignment[index].parents = parent_clause.clone();
            }
        }
        None
    }

    fn calculate_clause(&self, clause: &Clause) -> LiteralState {
        let values: Vec<LiteralState> = clause
            .iter()
            .map(|&literal| self.get_node_value(literal))
            .collect();
        if values.contains(&LiteralState::TRUE) {
            LiteralState::TRUE
        } else if values.contains(&LiteralState::UNKNOWN) {
            LiteralState::UNKNOWN
        } else {
            LiteralState::FALSE
        }
    }

    fn analyze_conflict(
        &mut self,
        clause: &Clause,
        current_literals: Option<Vec<Literal>>,
        past_literals: Option<Vec<Literal>>,
        checked: Option<Vec<Literal>>,
    ) -> Option<usize> {
        if self.level == 0 {
            return None;
        }
        let mut checked = checked.unwrap_or(vec![]);
        let mut current_literals = current_literals.unwrap_or(vec![]);
        let mut past_literals = past_literals.unwrap_or(vec![]);
        for literal in clause {
            if self.assignment[self.literal_to_index(*literal)].level == self.level {
                if !current_literals.contains(literal) {
                    current_literals.push(*literal);
                }
            } else if !past_literals.contains(literal) {
                past_literals.push(*literal);
            }
        }
        current_literals.retain(|&l| !checked.contains(&l) && !checked.contains(&-l));
        past_literals.retain(|&l| !checked.contains(&l) && !checked.contains(&-l));
        if current_literals.len() == 0 {
            panic!("no current literals");
        }
        if current_literals.len() == 1 {
            self.formula.push(
                current_literals
                    .iter()
                    .cloned()
                    .chain(past_literals.iter().cloned())
                    .collect(),
            );
            if past_literals.len() == 0 {
                return Some(self.level - 1);
            } else {
                return Some(
                    past_literals
                        .iter()
                        .map(|l| self.assignment[self.literal_to_index(*l)].level)
                        .max()
                        .unwrap(),
                );
            }
        } else {
            let latest_literal = self.get_latest_assignment(&current_literals).unwrap().abs();
            checked.push(latest_literal);
            let new_clause = self.assignment[self.literal_to_index(latest_literal)]
                .parents
                .iter()
                .filter(|&l| !checked.contains(&(l.abs())))
                .filter(|&l| self.get_node_value(*l) != LiteralState::UNKNOWN)
                .map(|l| *l)
                .collect();
            return self.analyze_conflict(
                &new_clause,
                Some(current_literals),
                Some(past_literals),
                Some(checked),
            );
        }
    }

    fn literal_to_index(&self, literal: Literal) -> usize {
        if literal > 0 {
            (literal - 1) as usize
        } else {
            (-literal - 1) as usize
        }
    }

    fn get_latest_assignment(&self, literals: &Vec<Literal>) -> Option<Literal> {
        let current_trail = self.trail.get(self.level).unwrap();
        let abs_literals: Vec<Literal> = literals.iter().map(|l| l.abs()).collect();
        for history in current_trail.iter().rev() {
            if abs_literals.contains(&history.abs()) {
                return Some(*history);
            }
        }
        None
    }

    fn backtrack(&mut self, level: usize) {
        let mut level_list: Vec<usize> = Vec::new();
        for node in &mut self.assignment {
            level_list.push(node.level);
            if node.level > level {
                node.value = LiteralState::UNKNOWN;
                node.level = 0;
                node.parents = vec![];
            }
        }
        self.trail.truncate(level + 1);
        self.level = level;
    }

    fn is_satisfied(&self) -> bool {
        for clause in &self.formula {
            if self.calculate_clause(clause) == LiteralState::FALSE
                || self.calculate_clause(clause) == LiteralState::UNKNOWN
            {
                return false;
            }
        }
        true
    }
}
