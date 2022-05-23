use std::collections::HashMap;

enum Command {
    SetVar(String),
    Add(Vec<Value>),
    Subtract(Vec<Value>),
    Multiply(Vec<Value>),
    Divide(Vec<Value>),
    Power(Vec<Value>),
    Modulo(Vec<Value>),
}

#[derive(Clone, PartialEq, Debug)]
enum Value {
    Nothing,
    Operand(f64),
    Variable(String),
}

#[derive(Debug)]
enum EngineError {
    TooManyVariableNames,
    MissingVariableName,
    MissingOperands,
    // MismatchType,
    UnknownCommand(String),
    MissingVariable(String),
    EvaluatorAnswerShouldNotBeValueVariable,
    NoValuesInQueue,
}

struct Evaluator {
    vars: HashMap<String, f64>,
    answers: Vec<Value>, // Saving answers to display at the end, not used in evaluation.
    answer: Value, // The main accumulator
}

impl Evaluator {
    fn new() -> Evaluator {
        Self {
            vars: HashMap::new(),
            answers: vec![],
            answer: Value::Nothing,
        }
    }

    fn operate(&self, mut operands: Vec<Value>, operator: fn(f64, f64) -> f64) -> Result<Value, EngineError> {
        operands.insert(0, self.answer.clone());
        let mut get_var_error_flag = false;
        let mut get_var_error_name: String = "".to_string(); 
        let result = Ok(Value::Operand(operands.into_iter()
            .filter_map(|v|
                match v {
                    Value::Nothing => None,
                    Value::Operand(num) => Some(num),
                    Value::Variable(var_name) => match self.vars.get(&var_name) {
                        Some(var_val) => Some(var_val.clone()),
                        None => {
                            get_var_error_flag = true;
                            get_var_error_name = var_name;
                            None
                        }
                    }
                }
            )
            .reduce(|acc: f64, x: f64| operator(acc, x)).unwrap()));

        if get_var_error_flag {
            return Err(EngineError::MissingVariable(get_var_error_name.into()));
        } else {
            return result;
        }
    }

    fn evaluate(mut self, commands: &[Command]) -> Result<Value, EngineError> {
            
        fn add(acc: f64, x: f64) -> f64 {
            acc + x
        }
        fn subtract(acc: f64, x: f64) -> f64 {
            acc - x
        }
        fn multiply(acc: f64, x: f64) -> f64 {
            acc * x
        }
        fn divide(acc: f64, x: f64) -> f64 {
            acc / x
        }
        fn power(acc: f64, x: f64) -> f64 {
            acc.powf(x)
        }
        fn modulo(acc: f64, x: f64) -> f64 {
            acc % x
        }

        for command in commands {
            match command {
                Command::SetVar(name) => {
                    match self.answer {
                        Value::Nothing => return Err(EngineError::NoValuesInQueue),
                        Value::Operand(num) => {self.vars.insert(name.into(), num.clone());}
                        Value::Variable(_) => return Err(EngineError::EvaluatorAnswerShouldNotBeValueVariable)
                    }
                    self.answer = Value::Nothing;
                    self.answers.push(self.answer.clone());
                }
                Command::Add(operands) => {
                    self.answer = self.operate(operands.to_vec(), add)?;
                    self.answers.push(self.answer.clone());
                }
                Command::Subtract(operands) => {
                    self.answer = self.operate(operands.to_vec(), subtract)?;
                    self.answers.push(self.answer.clone());
                }
                Command::Multiply(operands) => {
                    self.answer = self.operate(operands.to_vec(), multiply)?;
                    self.answers.push(self.answer.clone());
                }
                Command::Divide(operands) => {
                    self.answer = self.operate(operands.to_vec(), divide)?;
                    self.answers.push(self.answer.clone());
                }
                Command::Power(operands) => {
                    self.answer = self.operate(operands.to_vec(), power)?;
                    self.answers.push(self.answer.clone());
                }
                Command::Modulo(operands) => {
                    self.answer = self.operate(operands.to_vec(), modulo)?;
                    self.answers.push(self.answer.clone());
                }
            }
        }
        Ok(self.answer)
    }
}

fn parse_float(input: &str) -> Result<Value, EngineError> {
    let result = input.parse::<f64>();

    match result {
        Ok(x) => Ok(Value::Operand(x)),
        _ => Ok(Value::Variable(input.into())),
    }
}

fn parse_operands(operand_strings: &[&str]) -> Result<Vec<Value>, EngineError> {
    Ok(operand_strings.iter().map(|s| parse_float(s).unwrap()).collect())
}

fn parse_var_name(var_name: &str) -> Result<String, EngineError> {
    Ok(var_name.into())
}

fn parse_set_var(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() <= 1 {
        return Err(EngineError::MissingVariableName);
    }
    if input.len() >= 3 {
        return Err(EngineError::TooManyVariableNames);
    }

    let var_name = parse_var_name(input[1])?;
    
    Ok(Command::SetVar(var_name))
} 

fn parse_add(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() <= 1 {
        return Err(EngineError::MissingOperands);
    }

    let operands = parse_operands(input.split_last().unwrap().1)?;

    Ok(Command::Add(operands))
}

fn parse_subtract(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() <= 1 {
        return Err(EngineError::MissingOperands);
    }

    let operands = parse_operands(input.split_last().unwrap().1)?;

    Ok(Command::Subtract(operands))
}

fn parse_multiply(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() <= 1 {
        return Err(EngineError::MissingOperands);
    }

    let operands = parse_operands(input.split_last().unwrap().1)?;

    Ok(Command::Multiply(operands))
}

fn parse_divide(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() <= 1 {
        return Err(EngineError::MissingOperands);
    }

    let operands = parse_operands(input.split_last().unwrap().1)?;

    Ok(Command::Divide(operands))
}

fn parse_power(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() <= 1 {
        return Err(EngineError::MissingOperands);
    }

    let operands = parse_operands(input.split_last().unwrap().1)?;

    Ok(Command::Power(operands))
}

fn parse_modulo(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() <= 1 {
        return Err(EngineError::MissingOperands);
    }

    let operands = parse_operands(input.split_last().unwrap().1)?;

    Ok(Command::Modulo(operands))
}

fn parse(input: &str) -> Result<Vec<Command>, EngineError> {
    let mut output = vec![];

    for line in input.lines() {
        let command: Vec<_> = line.split_whitespace().collect();

        match command.first() { // If the line starts with # this is a comment line, skip the parsing and ignore.
            Some(x) if (x.starts_with("#")) => continue,
            Some(x) if (*x == "=") => {
                output.push(parse_set_var(&command)?);
                continue;
            }
            Some(_) => {},
            None => {},
        }

        match command.last() {
            Some(x) if (*x == "+" || *x == "plus" || *x == "add") => {
                output.push(parse_add(&command)?);
            }
            Some(x) if (*x == "-" || *x == "minus" || *x == "subtract") => {
                output.push(parse_subtract(&command)?);
            }
            Some(x) if (*x == "x" || *x == "*" || *x == "times" || *x == "multiply") => {
                output.push(parse_multiply(&command)?);
            }
            Some(x) if (*x == "/" || *x == "div" || *x == "divide") => {
                output.push(parse_divide(&command)?);
            }
            Some(x) if (*x == "**" || *x == "^" || *x == "power") => {
                output.push(parse_power(&command)?);
            }
            Some(x) if (*x == "%" || *x == "mod" || *x == "modulus" || *x == "modulo") => {
                output.push(parse_modulo(&command)?);
            }
            Some(name) => return Err(EngineError::UnknownCommand(name.to_string())),
            None => {}
        }
    }
    Ok(output)
}

#[test]
fn test_eval_add() -> Result<(), EngineError> {
    let commands = vec![
        Command::Add(vec![Value::Operand(1.0), Value::Operand(2.0)]),
        Command::Add(vec![Value::Operand(3.0), Value::Operand(4.0), Value::Operand(5.0)]),
    ];

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(15.0));

    Ok(())
}

#[test]
fn test_eval_variables() -> Result<(), EngineError> {
    let commands = vec![
        Command::Add(vec![Value::Operand(5.0), Value::Operand(5.0)]),
        Command::SetVar(String::from("derp")),
        Command::Add(vec![Value::Operand(2.0), Value::Operand(2.0)]),
        Command::SetVar(String::from("blorp")),
        Command::Add(vec![Value::Operand(5.0), Value::Variable(String::from("derp"))]),
    ];

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(15.0));

    Ok(())
}

#[test]
fn test_parse_add() -> Result<(), EngineError> {
    let input = "1 2 3 +\n4 5 +";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(15.0));

    Ok(())
}

#[test]
fn test_parse_add_plus() -> Result<(), EngineError> {
    let input = "1 2 3 +\n4 5 plus";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(15.0));

    Ok(())
}

#[test]
fn test_parse_add_plus_add() -> Result<(), EngineError> {
    let input = "1 2 3 +\n4 5 plus\n 6 add";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(21.0));

    Ok(())
}

#[test]
fn test_parse_subtract() -> Result<(), EngineError> {
    let input = "20 2 -\n3 5 minus\n1 subtract";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(9.0));

    Ok(())
}

#[test]
fn test_parse_add_subtract() -> Result<(), EngineError> {
    let input = "20 5 +\n3 4 -";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(18.0));

    Ok(())
}

#[test]
fn test_parse_multiply() -> Result<(), EngineError> {
    let input = "2 5 x\n3 4 *\n5 times\n6 multiply";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(3600.0));

    Ok(())
}

#[test]
fn test_parse_divide() -> Result<(), EngineError> {
    let input = "100 2 /\n5 divide\n2 div";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(5.0));

    Ok(())
}

#[test]
fn test_parse_power() -> Result<(), EngineError> {
    let input = "2 1 **\n3 ^\n2 2 power";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(4096.0));

    Ok(())
}

#[test]
fn test_parse_comment() -> Result<(), EngineError> {
    let input = "#2 2\n# 2 1 +\n3 2 +\n4 5 plus";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(14.0));

    Ok(())
}

#[test]
fn test_parse_modulus() -> Result<(), EngineError> {
    let input = "29 17 %\n7 mod\n3 modulus\n3 modulo";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(2.0));

    Ok(())
}

#[test]
fn test_parse_variables() -> Result<(), EngineError> {
    let input = "5 5 +\n= derp\n2 2 +\n= blorp\n5 derp add";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(15.0));

    Ok(())
}

#[test]
fn test_parse_negatives() -> Result<(), EngineError> {
    let input = "5 -5 +";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(0.0));

    Ok(())
}

fn main() -> Result<(), EngineError> {
    for arg in std::env::args().skip(1) {
        let contents = std::fs::read_to_string(arg).unwrap();
        let engine = Evaluator::new();
        let commands = parse(&contents)?;
        let answer = engine.evaluate(&commands)?;

        match answer {
            Value::Nothing => {
                return Err(EngineError::NoValuesInQueue);
            },
            Value::Operand(ans) => {
                if ans.fract() == 0.0 {
                    println!("{:?}", ans as i64);
                } else {
                    println!("{:?}", ans);
                }
            },
            Value::Variable(_) => {
                return Err(EngineError::EvaluatorAnswerShouldNotBeValueVariable);
            }
        }
    }

    Ok(())
}