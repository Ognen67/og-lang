use crate::compilation_unit::CompilationUnit;

mod ast;
mod diagnostics;
mod text;
mod compilation_unit;


fn main() {
    let input = "\
        let a = 3
        let b = (1 + 2) * --a + 3 + 5 ** 2\
    ";

    let compilation_unit = CompilationUnit::compile(input);
    compilation_unit.maybe_run();
}