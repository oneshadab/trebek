mod lang;

use lang::{runner::Runner};


fn main() {
    let program = String::from("
        (def x 2)
        (def y 3)
        (print (+ x y))
    ");

    let mut runner = Runner::new();
    let output = runner.run(program);

    println!("{:?}", output);
}
