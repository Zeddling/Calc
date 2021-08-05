use crate::analyzer::{
    AnalyzedExpression, AnalyzedFactor, AnalyzedProgram, AnalyzedStatement, AnalyzedTerm,
};
use crate::parser::{ExprOperator, TermOperator};
use crate::symbol_table::SymbolTable;

fn to_factor(variables: &SymbolTable, analyzed_factor: &AnalyzedFactor) -> String {
    match analyzed_factor {
        AnalyzedFactor::Literal(value) => value.to_string() + "f64",
        AnalyzedFactor::Identifier(handle) => {
            "_".to_string() + &variables.get_name(*handle).to_string()
        }
        AnalyzedFactor::SubExpression(expr) => {
            "(".to_string() + &to_expression(variables, expr) + ")"
        }
    }
}

fn to_expression(variables: &SymbolTable, analyzed_expression: &AnalyzedExpression) -> String {
    let mut result = to_term(variables, &analyzed_expression.0);
    for term in &analyzed_expression.1 {
        match term.0 {
            ExprOperator::Add => {
                result += " + ";
                result += &to_term(variables, &term.1);
            }
            ExprOperator::Subtract => {
                result += " - ";
                result += &to_term(variables, &term.1);
            }
        }
    }
    result
}

fn to_term(variables: &SymbolTable, analyzed_term: &AnalyzedTerm) -> String {
    let mut result = to_factor(variables, &analyzed_term.0);
    for factor in &analyzed_term.1 {
        match factor.0 {
            TermOperator::Divide => {
                result += " / ";
                result += &to_factor(variables, &factor.1)
            }
            TermOperator::Multiply => {
                result += " * ";
                result += &to_factor(variables, &factor.1)
            }
        }
    }
    result
}

fn to_statement(variables: &SymbolTable, analyzed_statement: &AnalyzedStatement) -> String {
    match analyzed_statement {
        AnalyzedStatement::Assignment(handle, expression) => {
            format!(
                "_{} = {}",
                variables.get_name(*handle),
                to_expression(&variables, expression)
            )
        }
        AnalyzedStatement::Declaration(handle) => {
            format!("let mut _{} = 0.0", variables.get_name(*handle))
        }
        AnalyzedStatement::InputOperation(handle) => {
            format!("_{} = input()", variables.get_name(*handle))
        }
        AnalyzedStatement::OutputOperation(expression) => {
            format!(
                "println!(\"{}\", {})",
                "{}",
                to_expression(&variables, expression)
            )
        }
    }
}

pub fn to_program(variables: &SymbolTable, analyzed_program: &AnalyzedProgram) -> String {
    let mut program = std::fs::read_to_string("dependencies/prefix.txt").unwrap();

    for statement in analyzed_program {
        program += "    ";
        program += &to_statement(&variables, statement);
        program += ";\n";
    }
    program += "}\n";
    program
}
