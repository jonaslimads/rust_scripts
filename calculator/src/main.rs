#[macro_use]
extern crate lazy_static;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::{error, Parser};
use std::io;

#[derive(Parser)]
#[grammar = "calculator.pest"]
pub struct CalculatorParser;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left)
                | Operator::new(divide, Left)
                | Operator::new(remainder, Left),
            Operator::new(power, Right),
        ])
    };
}

fn main() -> io::Result<()> {
    loop {
        println!("> Type an expression (or \"exit\" to leave):");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let trimmed_buffer = buffer.trim();
        if trimmed_buffer == "exit" {
            break;
        }

        match calculate(trimmed_buffer) {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("{:?}", error),
        }
    }

    Ok(())
}

fn calculate(expression: &str) -> Result<f64, error::Error<Rule>> {
    let pairs = CalculatorParser::parse(Rule::calculation, expression)?;
    Ok(evaluate_expression(pairs))
}

fn evaluate_expression(expression: Pairs<Rule>) -> f64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<f64>().unwrap(),
            Rule::expression => evaluate_expression(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::remainder => lhs % rhs,
            Rule::power => lhs.powf(rhs),
            _ => unreachable!(),
        },
    )
}

#[test]
fn test_calculations() {
    assert_eq!(5.0, calculate("25 % 10").unwrap());
    assert_eq!(
        262008.22535211267,
        calculate("24 + 10 * (123 * 213 - 123 / 213)").unwrap()
    );
}
