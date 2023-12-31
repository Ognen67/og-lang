use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::Ast;
use crate::ast::evaluator::ASTEvaluator;
use crate::ast::parser::Parser;
use crate::diagnostics::DiagnosticsBagCell;
use crate::diagnostics::printer::DiagnosticsPrinter;

mod ast;
mod diagnostics;
mod text;

fn main() {
    let input = "(10 * 4) + 12 + (8*5) & 5";
    let text= text::SourceText::new(input.to_string());

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("{:?}", tokens);
    let diagnostics_bag: DiagnosticsBagCell = Rc::new(RefCell::new(diagnostics::DiagnosticsBag::new()));

    let mut ast: Ast = Ast::new();
    let mut parser: Parser = Parser::new(tokens, Rc::clone(&diagnostics_bag));

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement)
    }
    let diagnostics_binding = diagnostics_bag.borrow();


    ast.visualize();
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    println!("Result: {:?}", eval.last_value);

    if diagnostics_binding.diagnostics.len() > 0 {
        let diagnostic_printer = DiagnosticsPrinter::new(
            &text,
            &diagnostics_binding.diagnostics
        );
        diagnostic_printer.print();
        return;
    }
}
