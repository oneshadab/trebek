mod lang;

use lang::{runner::Runner};


fn main() {
    let program = String::from("
        (+ 1 2)
    ");

    let mut runner = Runner::new();
    let output = runner.eval(program);

    println!("{:?}", output);
}
