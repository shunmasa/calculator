use serde::{Deserialize, Serialize};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum CustomNumber {
    Integer(i64),
    Float(f64),
}

impl CustomNumber {
    fn to_f64(&self) -> f64 {
        match self {
            CustomNumber::Integer(i) => *i as f64,
            CustomNumber::Float(f) => *f,
        }
    }
}

impl FromStr for CustomNumber {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(i) => Ok(CustomNumber::Integer(i)),
            Err(_) => {
                match s.parse::<f64>() {
                    Ok(f) => Ok(CustomNumber::Float(f)),
                    Err(e) => Err(e),
                }
            }
        }
    }
}

#[derive(Debug, StructOpt)]
struct CalculatorInput {
    #[structopt(subcommand)]
    operation: Operation,

    #[structopt(flatten)]
    numbers: Numbers,
}

#[derive(Debug, StructOpt)]
struct Numbers {
    num1: CustomNumber,
    num2: CustomNumber,
}

#[derive(Debug, StructOpt)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn apply(&self, num1: f64, num2: f64) -> Result<f64, &'static str> {
        match self {
            Operation::Add => Ok(num1 + num2),
            Operation::Subtract => Ok(num1 - num2),
            Operation::Multiply => Ok(num1 * num2),
            Operation::Divide => {
                if num2 != 0.0 {
                    Ok(num1 / num2)
                } else {
                    Err("Division by zero is not allowed")
                }
            }
        }
    }
}

fn main() {
    let args = CalculatorInput::from_args();

    match args.operation.apply(args.numbers.num1.to_f64(), args.numbers.num2.to_f64()) {
        Ok(result) => {
            let output = CalculatorOutput {
                result: CustomNumber::Float(result),
            };
            println!("Result: {:?}", output);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

#[derive(Debug, Serialize, Clone)]
struct CalculatorOutput {
    result: CustomNumber,
}
