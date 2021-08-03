extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while,
    character::complete::{alpha1, char},
    combinator::map,
    multi::many0,
    number::complete::double,
    sequence::{delimited, preceded, tuple},
    IResult,
};

//  =========== Enums ===========
/// Holds the add and subtract operations
/// Backus-Naur -> <expr> ::= <term> | <expr> "+" <term> | <expr> "-" <term>
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExprOperator {
    Add,
    Subtract,
}

/// Represents a factor
/// Backus-Naur -> <factor> ::= <identifier> | <literal> | "(" <expr> ")"
#[derive(Debug, PartialEq)]
pub enum ParsedFactor<'a> {
    Literal(f64),
    Identifier(&'a str),
    SubExpression(Box<ParsedExpr<'a>>),
}

/// Represents a statement
/// Backus-Naur -> <statement> ::= "@" <identifier> | ">" <identifier> | "<" <identifier>
#[derive(Debug)]
pub enum ParsedStatement<'a> {
    Declaration(&'a str),
    InputOperation(&'a str),
    OutputOperation(ParsedExpr<'a>),
    Assignment(&'a str, ParsedExpr<'a>),
}

/// Represents a term
/// Backus-Naur -> <term> ::= <factor> | <term> "*" <factor> | <term> "/" <factor>
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TermOperator {
    Multiply,
    Divide,
}

//  =========== Types ===========
/// A tuple that represents a parsed term and a vector of expression operators and parsed terms
pub type ParsedExpr<'a> = (ParsedTerm<'a>, Vec<(ExprOperator, ParsedTerm<'a>)>);
/// Represents a vector of the parsed statements
pub type ParsedProgram<'a> = Vec<ParsedStatement<'a>>;
/// A tuple that represents a parsed factor and a vector of term operators and parsed factors
pub type ParsedTerm<'a> = (ParsedFactor<'a>, Vec<(TermOperator, ParsedFactor<'a>)>);

//  =========== Parser Functions ===========
//  The parser will be defined as per the Backus-Naur form
//  grammar specification

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_declaration(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((char('@'), skip_spaces, parse_identifier))(input)
        .map(|(input, output)| (input, ParsedStatement::Declaration(output.2)))
}

fn parse_input_statement(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((char('>'), skip_spaces, parse_identifier))(input)
        .map(|(input, output)| (input, ParsedStatement::InputOperation(output.2)))
}

fn parse_expression(input: &str) -> IResult<&str, ParsedExpr> {
    tuple((
        parse_term,
        many0(tuple((
            preceded(
                skip_spaces,
                alt((
                    map(char('+'), |_| ExprOperator::Add),
                    map(char('-'), |_| ExprOperator::Subtract),
                )),
            ),
            parse_term,
        ))),
    ))(input)
}

fn parse_output_statement(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((char('<'), skip_spaces, parse_expression))(input)
        .map(|(input, output)| (input, ParsedStatement::OutputOperation(output.2)))
}

fn parse_subexpression(input: &str) -> IResult<&str, ParsedExpr> {
    delimited(
        preceded(skip_spaces, char('(')),
        parse_expression,
        preceded(skip_spaces, char(')')),
    )(input)
}

fn parse_assignment(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((
        parse_identifier,
        skip_spaces,
        tag(":="),
        skip_spaces,
        parse_expression,
    ))(input)
    .map(|(input, output)| (input, ParsedStatement::Assignment(output.0, output.4)))
}

fn parse_factor(input: &str) -> IResult<&str, ParsedFactor> {
    preceded(
        skip_spaces,
        alt((
            map(parse_identifier, ParsedFactor::Identifier),
            map(double, ParsedFactor::Literal),
            map(parse_subexpression, |expression| {
                ParsedFactor::SubExpression(Box::new(expression))
            }),
        )),
    )(input)
}

fn parse_term(input: &str) -> IResult<&str, ParsedTerm> {
    tuple((
        parse_factor,
        many0(tuple((
            preceded(
                skip_spaces,
                alt((
                    map(char('*'), |_| TermOperator::Multiply),
                    map(char('/'), |_| TermOperator::Divide),
                )),
            ),
            parse_factor,
        ))),
    ))(input)
}

pub fn parse_progam(input: &str) -> IResult<&str, ParsedProgram> {
    many0(preceded(
        skip_spaces,
        alt((
            parse_declaration,
            parse_input_statement,
            parse_output_statement,
            parse_assignment,
        )),
    ))(input)
}

//  =========== Utility Functions ===========
fn skip_spaces(input: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |ch| chars.contains(ch))(input)
}
