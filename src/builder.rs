use std::error;
use std::fmt;

use self::super::Environment;
use self::super::Num;
use self::super::Prior;
use self::super::bricks;
use self::super::math;
use self::super::bricks::Brick;

use self::super::UnaryOperationMap;
use self::super::BinaryOperationMap;
use self::super::AppendedOperationMap;

#[derive(Debug)]
pub enum Error {
    SyntaxError,
    OperationNotFoundError,
}
impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::SyntaxError => "There were some invalid token combinations",
            Error::OperationNotFoundError => "The provided operation was not contained by the environment",
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Fix with description()
        let msg = match *self {
            Error::SyntaxError => "There were some invalid token combinations",
            Error::OperationNotFoundError => "The provided operation was not contained by the environment",
        };
        write!(f, "{}", msg)
    }
}

impl Environment {
    fn new() -> Environment {
        let mut env = Environment::raw();
        env.add_binary("+", 8f32, Box::new(|a, b| a + b));
        env.add_binary("-", 8f32, Box::new(|a, b| a - b));
        env.add_binary("*", 6f32, Box::new(|a, b| a * b));
        env.add_binary("/", 6f32, Box::new(|a, b| a / b));
        env.add_binary("^", 4f32, Box::new(|a, b| a.powf(b)));
        env.add_unary("sqrt", 4f32, Box::new(|a| a.sqrt()));
        env.add_unary("-", 1f32, Box::new(|a| -a));
        env.add_appended("!", Box::new(math::fac));
        env.add_unary("abs", 1f32, Box::new(|a| a.abs()));
        env
    }
    fn raw() -> Environment {
        Environment {un_operations: UnaryOperationMap::new(),
            bin_operations: BinaryOperationMap::new(),
            app_operations: AppendedOperationMap::new()}
    }
    fn add_unary(&mut self, qualifier: &str, priority: Prior, operation: Box<Fn(Num) -> Num>) {
        self.un_operations.insert(qualifier.to_string(), (operation, priority));
    }
    fn add_binary(&mut self, qualifier: &str, priority: Prior, operation: Box<Fn(Num, Num) -> Num>) {
        self.bin_operations.insert(qualifier.to_string(), (operation, priority));
    }
    fn add_appended(&mut self, qualifier: &str, operation: Box<Fn(Num) -> Num>) {
        self.app_operations.insert(qualifier.to_string(), operation);
    }
}

/// Calculate the value of the given equation
/// Returns Error in case of a wrong input
pub fn solve(env: &Environment, calc: String) -> Result<Num, Error> {
    let mut segment = String::new();
    let mut parsing_unary = false;
    let mut value: Option<Num> = None;
    let mut last_brick: Box<Brick> = Box::new(bricks::Bracket::new(None));
    let mut started = false; // Comes into play when making new bricks
    // build bricks
    for i in calc.chars() {
        // Skip whitespaces
        if i == ' ' {
            continue;
        } else if i == '(' { // opening bracket
            if started {
                last_brick = Box::new(bricks::Bracket::new(Some(last_brick)));
            } else {
                last_brick = Box::new(bricks::Bracket::new(None));
            }
        } else if i == ')' { // closing bracket
            if let Some(v) = value {
                let res = last_brick.resolve(v);
                value = Some(res.0);
                last_brick = res.1.unwrap();
            } else {
                return Err(Error::SyntaxError)
            }
        } else if let Some(v) = value { // Parsing binary operator or unary appended
            if i.is_numeric() {
                // binary operator
                let result = env.bin_operations.get(&segment[..]);

                if result.is_some() {
                    let (ref operation, ref priority) = *result.unwrap();
                    if started {
                        last_brick = Box::new(bricks::BinaryBrick::new(Some(last_brick),
                                                                       v, *operation, *priority));
                    } else {
                        last_brick = Box::new(bricks::BinaryBrick::new(None,
                                                                       v, *operation, *priority));
                    }
                } else if let Some(operation) = env.app_operations.get(&segment[..]) { // Appended unary
                    value = Some(operation(v));
                } else {
                    return Err(Error::OperationNotFoundError);
                }
            } else {
                segment.push(i);
            }
        } else if parsing_unary {
            let result = env.un_operations.get(&segment[..]);

            if i.is_numeric() {
                // Build unary block
            } else if result.is_some() { // finished unary operator
                let (operation, priority) = *result.unwrap();

                        if started {
                            last_brick = Box::new(bricks::UnaryBrick::new(Some(last_brick), operation, priority));
                        } else {
                            last_brick = Box::new(bricks::UnaryBrick::new(None, operation, priority));
                }
                segment.clear();
                parsing_unary = false;
            } else {
                segment.push(i);
            }
        } else { // parsing number or beginning unary operator
            if i.is_numeric() {
                segment.push(i);
            } else if segment.is_empty() { // beginning of unary operator
                parsing_unary = true;
                segment.push(i);
            } else { // end of number
                value = Some(segment.parse().unwrap());
                segment.clear();
            }
        }
        started = true;
    }
    let mut value: Num = segment.parse().unwrap();
    while let (new_value, Some(new_brick)) = last_brick.resolve(value) {
        value = new_value;
        last_brick = new_brick;
    }
    Ok(value)
}