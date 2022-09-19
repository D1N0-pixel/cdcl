pub struct Parser<'a> {
    input: Vec<&'a str>,
    pos: usize,
    line: Option<&'a str>,
    clause: bool,
    pub nbvars: usize,
    pub nbclauses: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            input: input.lines().collect::<Vec<&'a str>>(),
            pos: 0,
            line: None,
            clause: false,
            nbvars: 0,
            nbclauses: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Vec<i32>>, &'static str> {
        let mut formula = Vec::new();
        loop {
            self.next_line();
            if self.line.is_none() {
                break;
            }
            match self.parse_line() {
                Ok(l) => {
                    match l {
                        Some(l) => formula.push(l),
                        None => continue,
                    }
                },
                Err(e) => return Err(e),
            }
        }
        Ok(formula)
    }

    fn parse_line(&mut self) -> Result<Option<Vec<i32>>, &'static str> {
        match self.line {
            Some(l) => {
                return self.handle_line(l);
            }
            None => {
                return Ok(None);
            }
        }
    }

    fn handle_line(&mut self, line: &str) -> Result<Option<Vec<i32>>, &'static str> {
        if line.len() == 0 {
            return Ok(None);
        }
        match line.chars().next().unwrap() {
            'c' => {
                if self.clause {
                    Err("Unexpected comment after clause")
                } else {
                    Ok(None)
                }
            }
            'p' => {
                if !self.clause {
                    self.clause = true;
                    let mut parts = line.split_whitespace();
                    if parts.next().unwrap() != "p" {
                        Err("Invalid problem format")?;
                    }
                    if parts.next().unwrap() != "cnf" {
                        Err("Invalid problem format")?;
                    }
                    match parts.next().unwrap().parse::<usize>() {
                        Ok(n) => self.nbvars = n,
                        Err(_) => Err("Invalid problem format")?,
                    }
                    match parts.next().unwrap().parse::<usize>() {
                        Ok(n) => self.nbclauses = n,
                        Err(_) => Err("Invalid problem format")?,
                    }
                    Ok(None)
                } else {
                    Err("Invalid problem usage")
                }
            }
            '1'..='9' | '-' | ' ' => {
                if self.clause {
                    match self.parse_clause(line) {
                        Ok(clause) => Ok(Some(clause)),
                        Err(e) => Err(e),
                    }
                } else {
                    Err("Invalid clause usage")
                }
            }
            _ => {
                Ok(None)
                // Err("Invalid line")
            }
        }
    }

    fn parse_clause(&self, line: &str) -> Result<Vec<i32>, &'static str> {
        let mut clause = Vec::new();
        for literal in line.split_whitespace() {
            let literal = literal.trim();
            if literal.len() == 0 {
                continue;
            }
            let literal = match literal.parse::<i32>() {
                Ok(l) => l,
                Err(_) => return Err("Invalid literal"),
            };
            if literal == 0 {
                break;
            }
            clause.push(literal);
        }
        Ok(clause)
    }

    fn next_line(&mut self) {
        if self.pos >= self.input.len() {
            self.line = None;
        } else {
            self.line = Some(self.input[self.pos]);
            self.pos += 1;
        }
    }
}