use trebek::runner::Runner;


fn main() {
    let program = String::from("
        (def x 2)
        (def y 3)
        (def inc (fn (x) (+ x 1)))
        (if true (print true) (print false))
        (if false (print true) (print false))
    ");

    let mut runner = Runner::new();
    let output = runner.run(program);

    eprintln!("{:?}", output);
}
