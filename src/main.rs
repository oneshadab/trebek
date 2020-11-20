mod lang;

use lang::parser::Parser;


fn main() {
    let program = "
        (+ 1 2)
    ";

    let parser = Parser{};
    let output = parser.tokenize(program.into());

    println!("{:?}", output);
}
