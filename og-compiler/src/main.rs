use crate::ast::Ast;
use crate::ast::evaluator::ASTEvaluator;
use crate::ast::parser::Parser;

mod ast;

fn main() {
    let input = "(10 * 4) + 12";

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("{:?}", tokens);

    let mut ast: Ast = Ast::new();
    let mut parser: Parser = Parser::new(tokens);

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement)
    }

    ast.visualize();
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    println!("Result: {:?}", eval.last_value);
}
