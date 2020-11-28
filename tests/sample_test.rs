
pub mod tests {
  use trebek::lang::runtime::Runtime;

  #[test]
  fn it_runs() {
    let program = String::from("
        (def x 2)
        (def y 3)
        (def inc (fn (x) (+ x 1)))
        (if true (print true) (print false))
        (if false (print true) (print false))
    ");
    let mut runtime = Runtime::new();
    let output = runtime.run(program);

    assert_eq!(output, "Empty");
  }
}