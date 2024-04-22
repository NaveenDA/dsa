use std::{cmp, collections::HashMap, vec};

pub fn valid_parentheses(s: String) -> bool {
    let mut brackets: HashMap<char, char> = HashMap::new();
    brackets.insert(')', '(');
    brackets.insert('}', '{');
    brackets.insert(']', '[');
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        let has_end: Option<&char> = brackets.get(&c);
        match has_end {
            Some(&v) => {
                if (stack.len() > 0 && stack[stack.len() - 1] != v) || stack.len() <= 0 {
                    return false;
                }
                stack.pop();
            }
            None => {
                stack.push(c);
            }
        }
    }
    stack.len() == 0
}

pub struct MinStack {
    main: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        Self {
            main: vec![],
            min: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        let last = self.min.last();
        let min = match last {
            Some(&v) => v,
            None => val,
        };
        self.main.push(val);
        self.min.push(cmp::min(val, min));
    }

    pub fn pop(&mut self) -> () {
        self.min.pop();
        self.main.pop();
    }

    pub fn top(&self) -> i32 {
        match self.main.last() {
            Some(&v) => v,
            None => 0,
        }
    }

    pub fn get_min(&self) -> i32 {
        match self.min.last() {
            Some(&v) => v,
            None => 0,
        }
    }
}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for token in tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().unwrap_or_default();
                let a = stack.pop().unwrap_or_default();
                let result = match token.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => 0,
                };
                stack.push(result);
            }
            _ => stack.push(token.parse().unwrap_or_default()),
        }
    }
    stack.last().copied().unwrap_or_default()
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut stack: Vec<String> = vec![];
    fn backtrack(n: i32, stack: &mut Vec<String>, combination: String, nopen: i32, nclose: i32) {
        if nopen == nclose && nopen == n {
            stack.push(combination);
            return;
        }
        if nopen < n {
            backtrack(n, stack, combination.clone() + "(", nopen + 1, nclose);
        }
        if nopen > nclose {
            backtrack(n, stack, combination + ")", nopen, nclose + 1)
        }
    }
    backtrack(n, &mut stack, String::new(), 0, 0);
    stack
}

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<[i32; 2]> = position
        .iter()
        .zip(speed.iter())
        .map(|(&p, &s)| [p, s])
        .collect();
    let mut stack: Vec<f64> = vec![];
    cars.sort_by_key(|&car| car[0]);
    cars.reverse();
    for [p, s] in cars {
        let time =(target - p) as f64 / s as f64;
        if let Some(&last) = stack.last() {
            if time > last {
                stack.push(time);
            }
        } else {
            stack.push(time);
        }
    }
    stack.len() as i32
}

// car_fleet(10, vec![6,8],vec![3,2]) 
// expected:2
// result:1