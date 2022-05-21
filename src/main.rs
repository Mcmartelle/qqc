enum Command {
    Add(Vec<Value>),
    Subtract(Vec<Value>),
}

#[derive(Clone, PartialEq, Debug)]
enum Value {
    Nothing,
    Operand(f64),
}

#[derive(Debug)]
enum EngineError {
    MissingOperands,
    MismatchType,
    UnknownCommand(String),
}

struct Evaluator {
    answers: Vec<Value>, // Saving answers to display at the end, not used in evaluation.
    answer: Value, // The main accumulator
}

impl Evaluator {
    fn new() -> Evaluator {
        Self {
            answers: vec![],
            answer: Value::Nothing,
        }
    }

    fn add(&self, mut operands: Vec<Value>) -> Result<Value, EngineError> {
        operands.insert(0, self.answer.clone());
        Ok(Value::Operand(operands.into_iter()
            .filter_map(|v|
                match v {
                    Value::Nothing => None,
                    Value::Operand(num) => Some(num)  
                }
            )
            .reduce(|acc: f64, x: f64| acc + x).unwrap()))
    }

    fn subtract(&self, mut operands: Vec<Value>) -> Result<Value, EngineError> {
        operands.insert(0, self.answer.clone());
        Ok(Value::Operand(operands.into_iter()
            .filter_map(|v|
                match v {
                    Value::Nothing => None,
                    Value::Operand(num) => Some(num)  
                }
            )
            .reduce(|acc: f64, x: f64| acc - x).unwrap()))
    }

    fn evaluate(mut self, commands: &[Command]) -> Result<Value, EngineError> {
        for command in commands {
            match command {
                Command::Add(operands) => {
                    self.answer = self.add(operands.to_vec())?;
                    self.answers.push(self.answer.clone());
                }
                Command::Subtract(operands) => {
                    self.answer = self.subtract(operands.to_vec())?;
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
        _ => Err(EngineError::MismatchType),
    }
}

fn parse_operands(operand_strings: &[&str]) -> Result<Vec<Value>, EngineError> {
    Ok(operand_strings.iter().map(|s| parse_float(s).unwrap()).collect())
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

fn parse(input: &str) -> Result<Vec<Command>, EngineError> {
    let mut output = vec![];

    for line in input.lines() {
        let command: Vec<_> = line.split_whitespace().collect();

        match command.last() {
            Some(x) if *x == "+" => {
                output.push(parse_add(&command)?);
            }
            Some(x) if *x == "-" => {
                output.push(parse_subtract(&command)?);
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
fn test_parse_add() -> Result<(), EngineError> {
    let input = "1 2 3 +\n4 5 +";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(15.0));

    Ok(())
}

#[test]
fn test_parse_subtract() -> Result<(), EngineError> {
    let input = "20 2 -\n3 5 -";
    
    let commands = parse(input)?;

    let evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Operand(10.0));

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

fn main() -> Result<(), EngineError> {
    for arg in std::env::args().skip(1) {
        let contents = std::fs::read_to_string(arg).unwrap();
        let engine = Evaluator::new();
        let commands = parse(&contents)?;
        let answer = engine.evaluate(&commands)?;

        println!("{:?}", answer);
    }

    Ok(())
}